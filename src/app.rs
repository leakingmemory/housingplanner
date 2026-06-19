//! The eframe application: a management side panel plus the timeline view.

use chrono::{Datelike, Duration, NaiveDate};

use std::collections::HashSet;

use crate::licenses;
use crate::model::{Group, Housing, Id, Person, Plan, Stay, Subject, GROUP_PALETTE};
use crate::timeline;

/// The active workspace tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Overview,
    Groups,
    Persons,
    Housings,
}

/// Storage key used for `eframe`'s built-in cross-platform persistence.
/// Kept as the original value so plans saved before the project rename still
/// load (only the migrated storage directory changed).
const STORAGE_KEY: &str = "hplan_plan";

/// Pixel width of a single day column, clamped to this range (shared by the
/// zoom slider and the Ctrl/Cmd + wheel / pinch zoom).
const MIN_DAY_WIDTH: f32 = 6.0;
const MAX_DAY_WIDTH: f32 = 80.0;

pub struct PlannerApp {
    plan: Plan,
    // --- View state (not persisted) ---
    view_start: NaiveDate,
    days_visible: i64,
    day_width: f32,
    /// Sub-day remainder accumulated while drag-panning the timeline.
    pan_remainder: f32,
    /// Sub-day remainder accumulated while pointer-anchored zooming.
    zoom_remainder: f32,
    /// Transient status line (e.g. result of the last file save/load).
    status: String,
    /// Whether the About / Licenses window is open.
    licenses_open: bool,
    /// App logo texture, shown in the About window.
    logo: Option<egui::TextureHandle>,
    // --- Tab navigation + per-tab selection (not persisted) ---
    active_tab: Tab,
    selected_group: Option<Id>,
    selected_person: Option<Id>,
    selected_housing: Option<Id>,
}

impl PlannerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let plan = cc
            .storage
            .and_then(|s| eframe::get_value::<Plan>(s, STORAGE_KEY))
            .unwrap_or_default();

        let today = chrono::Local::now().date_naive();
        let view_start = plan
            .earliest_arrival()
            .map(|d| d - Duration::days(2))
            .unwrap_or(today);

        // Decode the embedded icon PNG into a texture for the About window
        // (reuses eframe's PNG decoder, so no extra image dependency).
        let logo = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon-256.png"))
            .ok()
            .map(|d| {
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [d.width as usize, d.height as usize],
                    &d.rgba,
                );
                cc.egui_ctx
                    .load_texture("app-logo", image, egui::TextureOptions::LINEAR)
            });

        Self {
            plan,
            view_start,
            days_visible: 30,
            day_width: 26.0,
            pan_remainder: 0.0,
            zoom_remainder: 0.0,
            status: String::new(),
            licenses_open: false,
            logo,
            active_tab: Tab::Overview,
            selected_group: None,
            selected_person: None,
            selected_housing: None,
        }
    }
}

impl eframe::App for PlannerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, STORAGE_KEY, &self.plan);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.top_bar(ui);
        self.licenses_window(ui.ctx());

        egui::Panel::top("tabs").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Overview, "📊 Overview");
                ui.selectable_value(&mut self.active_tab, Tab::Groups, "👥 Groups");
                ui.selectable_value(&mut self.active_tab, Tab::Persons, "🧍 Persons");
                ui.selectable_value(&mut self.active_tab, Tab::Housings, "🏠 Housings");
            });
        });

        match self.active_tab {
            Tab::Overview => self.overview_tab(ui),
            Tab::Groups => self.groups_tab(ui),
            Tab::Persons => self.persons_tab(ui),
            Tab::Housings => self.housings_tab(ui),
        }
    }
}

impl PlannerApp {
    /// Ctrl/Cmd + wheel (or trackpad pinch) zooms the day width, anchored on the
    /// date under the pointer so it stays put. egui zeroes the scroll delta when
    /// the zoom modifier is held, so the surrounding scroll area doesn't move.
    fn handle_zoom(&mut self, ui: &egui::Ui, response: &egui::Response) {
        // contains_pointer (not hovered) so zoom keeps working when the pointer
        // is over a bar's hover-tooltip region, which sits on top of the canvas.
        if !response.contains_pointer() {
            return;
        }
        let zoom = ui.ctx().input(|i| i.zoom_delta());
        if (zoom - 1.0).abs() < 1e-4 {
            return;
        }

        let old_w = self.day_width;
        let new_w = (old_w * zoom).clamp(MIN_DAY_WIDTH, MAX_DAY_WIDTH);
        if (new_w - old_w).abs() < f32::EPSILON {
            return;
        }

        // Keep the date under the pointer fixed (fall back to the left edge).
        // Use the context pointer pos so it's correct even over a bar region.
        let plot_left = response.rect.min.x + timeline::LABEL_WIDTH;
        let pointer_x = ui.ctx().pointer_hover_pos().map_or(plot_left, |p| p.x);
        let anchor_px = (pointer_x - plot_left).max(0.0);

        // view_start shifts by the change in days spanned up to the anchor.
        self.zoom_remainder += anchor_px / old_w - anchor_px / new_w;
        let whole_days = self.zoom_remainder.trunc();
        if whole_days != 0.0 {
            self.view_start += Duration::days(whole_days as i64);
            self.zoom_remainder -= whole_days;
        }
        self.day_width = new_w;
    }

    /// Drag-to-pan: shift the visible date window as the canvas is dragged.
    /// Dragging right reveals earlier dates (a "grab the content" gesture); a
    /// fractional-day remainder is carried over so panning stays smooth.
    fn handle_pan(&mut self, ui: &egui::Ui, response: &egui::Response) {
        if response.dragged() && self.day_width > 0.0 {
            self.pan_remainder += response.drag_delta().x / self.day_width;
            let whole_days = self.pan_remainder.trunc();
            if whole_days != 0.0 {
                self.view_start -= Duration::days(whole_days as i64);
                self.pan_remainder -= whole_days;
            }
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        } else {
            self.pan_remainder = 0.0;
            if response.contains_pointer() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Housing Planner");
                ui.separator();

                ui.label("From:");
                date_edit(ui, &mut self.view_start);

                ui.label("Days:");
                ui.add(
                    egui::DragValue::new(&mut self.days_visible)
                        .range(7..=365)
                        .speed(1.0),
                );

                ui.label("Zoom:");
                ui.add(
                    egui::Slider::new(&mut self.day_width, MIN_DAY_WIDTH..=MAX_DAY_WIDTH)
                        .show_value(false),
                )
                .on_hover_text("Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline");

                if ui.button("Today").clicked() {
                    self.view_start = chrono::Local::now().date_naive();
                }
                if ui.button("Fit to stays").clicked() {
                    if let Some(first) = self.plan.earliest_arrival() {
                        self.view_start = first - Duration::days(2);
                    }
                }

                ui.separator();
                if ui.button("💾 Save…").clicked() {
                    self.save_to_file();
                }
                if ui.button("📂 Load…").clicked() {
                    self.load_from_file();
                }

                if !self.status.is_empty() {
                    ui.separator();
                    ui.label(egui::RichText::new(&self.status).weak());
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("ℹ About").clicked() {
                        self.licenses_open = true;
                    }
                });
            });
        });
    }

    /// The About window: app info, this app's license, and the embedded
    /// third-party dependency licenses (the cross-platform attribution surface).
    fn licenses_window(&mut self, ctx: &egui::Context) {
        // Clone the (Arc-backed) handle so the closure doesn't borrow `self`,
        // which `.open(&mut self.licenses_open)` already borrows mutably.
        let logo = self.logo.clone();
        egui::Window::new("About / Licenses")
            .open(&mut self.licenses_open)
            .resizable(true)
            .default_size([720.0, 560.0])
            .show(ctx, |ui| about_contents(ui, logo.as_ref()));
    }

    /// Prompt for a path and write the current plan as JSON.
    #[cfg(not(target_os = "android"))]
    fn save_to_file(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Housing Planner plan", &["json"])
            .set_file_name("plan.json")
            .save_file()
        else {
            return; // user cancelled
        };
        match serde_json::to_string_pretty(&self.plan) {
            Ok(json) => match std::fs::write(&path, json) {
                Ok(()) => self.status = format!("Saved → {}", path.display()),
                Err(e) => self.status = format!("Save failed: {e}"),
            },
            Err(e) => self.status = format!("Encode failed: {e}"),
        }
    }

    /// Prompt for a path and replace the current plan with one loaded from JSON.
    #[cfg(not(target_os = "android"))]
    fn load_from_file(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Housing Planner plan", &["json"])
            .pick_file()
        else {
            return; // user cancelled
        };
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                self.status = format!("Read failed: {e}");
                return;
            }
        };
        match serde_json::from_str::<crate::model::Plan>(&text) {
            Ok(mut plan) => {
                plan.reseed_ids();
                self.view_start = plan
                    .earliest_arrival()
                    .map(|d| d - Duration::days(2))
                    .unwrap_or_else(|| chrono::Local::now().date_naive());
                self.plan = plan;
                self.status = format!("Loaded ← {}", path.display());
            }
            Err(e) => self.status = format!("Parse failed: {e}"),
        }
    }

    // On Android the native file picker needs the activity/intents plumbing;
    // for now these are no-ops there (auto-persistence still applies).
    #[cfg(target_os = "android")]
    fn save_to_file(&mut self) {
        self.status = "File save is not available on Android yet.".to_owned();
    }
    #[cfg(target_os = "android")]
    fn load_from_file(&mut self) {
        self.status = "File load is not available on Android yet.".to_owned();
    }

    /// Central timeline for the active tab: render the filtered timeline, then
    /// apply the shared zoom/pan handlers to the returned response.
    fn timeline_panel(
        &mut self,
        ui: &mut egui::Ui,
        housings: &[Id],
        include: &dyn Fn(&Stay) -> bool,
        empty_hint: &str,
    ) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let filter = timeline::Filter {
                    housings,
                    include,
                    empty_hint,
                };
                let response = timeline::show(
                    ui,
                    &self.plan,
                    self.view_start,
                    self.days_visible,
                    self.day_width,
                    &filter,
                );
                self.handle_zoom(ui, &response);
                self.handle_pan(ui, &response);
            });
        });
    }

    fn overview_tab(&mut self, ui: &mut egui::Ui) {
        if self.plan.is_empty() {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading("Welcome to Housing Planner");
                    ui.label("Add housings, groups and people in the tabs above —");
                    ui.add_space(6.0);
                    if ui.button("📋 Load example data").clicked() {
                        self.plan.load_sample();
                    }
                });
            });
            return;
        }
        let housings: Vec<Id> = self.plan.housings.iter().map(|h| h.id).collect();
        self.timeline_panel(
            ui,
            &housings,
            &|_| true,
            "Add a housing in the Housings tab to start planning.",
        );
    }

    fn groups_tab(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("groups_panel")
            .resizable(true)
            .default_size(340.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| self.groups_editor(ui));
            });

        if let Some(gid) = self.selected_group {
            let housings: Vec<Id> = self
                .plan
                .housings
                .iter()
                .filter(|h| {
                    self.plan
                        .stays
                        .iter()
                        .any(|s| s.housing == h.id && s.subject == Subject::Group(gid))
                })
                .map(|h| h.id)
                .collect();
            let include = move |s: &Stay| s.subject == Subject::Group(gid);
            self.timeline_panel(ui, &housings, &include, "No stays for this group yet.");
        } else {
            egui::CentralPanel::default()
                .show_inside(ui, |ui| centered_hint(ui, "Select or create a group."));
        }
    }

    fn persons_tab(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("persons_panel")
            .resizable(true)
            .default_size(340.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| self.persons_editor(ui));
            });

        if let Some(pid) = self.selected_person {
            // Own stays plus stays of any group the person belongs to.
            let included: HashSet<Id> = self
                .plan
                .stays
                .iter()
                .filter(|s| self.plan.stay_includes_person(s, pid))
                .map(|s| s.id)
                .collect();
            let housings: Vec<Id> = self
                .plan
                .housings
                .iter()
                .filter(|h| {
                    self.plan
                        .stays
                        .iter()
                        .any(|s| s.housing == h.id && included.contains(&s.id))
                })
                .map(|h| h.id)
                .collect();
            let include = move |s: &Stay| included.contains(&s.id);
            self.timeline_panel(ui, &housings, &include, "No stays for this person yet.");
        } else {
            egui::CentralPanel::default()
                .show_inside(ui, |ui| centered_hint(ui, "Select or create a person."));
        }
    }

    fn housings_tab(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("housings_panel")
            .resizable(true)
            .default_size(340.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| self.housings_editor(ui));
            });

        if let Some(hid) = self.selected_housing {
            let housings = [hid];
            self.timeline_panel(ui, &housings, &|_| true, "No stays in this housing yet.");
        } else {
            egui::CentralPanel::default()
                .show_inside(ui, |ui| centered_hint(ui, "Select or create a housing."));
        }
    }

    fn groups_editor(&mut self, ui: &mut egui::Ui) {
        let Self {
            plan,
            selected_group: selected,
            ..
        } = self;

        let items: Vec<(Id, String)> =
            plan.groups.iter().map(|g| (g.id, g.name.clone())).collect();
        let ids: Vec<Id> = items.iter().map(|(i, _)| *i).collect();
        ensure_selection(selected, &ids);

        ui.horizontal(|ui| {
            entity_selector(ui, "Group", selected, &items);
            if ui.button("➕ New").clicked() {
                let id = plan.new_id();
                let color = GROUP_PALETTE[plan.groups.len() % GROUP_PALETTE.len()];
                plan.groups.push(Group {
                    id,
                    name: format!("Group {}", plan.groups.len() + 1),
                    color,
                });
                *selected = Some(id);
            }
        });

        let Some(gid) = *selected else {
            ui.label("No groups yet — add one.");
            return;
        };
        ui.separator();

        if let Some(group) = plan.groups.iter_mut().find(|g| g.id == gid) {
            ui.horizontal(|ui| {
                ui.color_edit_button_srgb(&mut group.color);
                ui.add(egui::TextEdit::singleline(&mut group.name).desired_width(200.0));
            });
        }
        if ui.button("🗑 Delete group").clicked() {
            plan.groups.retain(|g| g.id != gid);
            for p in &mut plan.persons {
                if p.group == Some(gid) {
                    p.group = None;
                }
            }
            plan.stays.retain(|s| s.subject != Subject::Group(gid));
            *selected = None;
            return;
        }

        ui.separator();
        ui.label("Members:");
        let members: Vec<(Id, String)> = plan
            .persons
            .iter()
            .filter(|p| p.group == Some(gid))
            .map(|p| (p.id, p.name.clone()))
            .collect();
        let mut detach: Option<Id> = None;
        for (pid, name) in &members {
            ui.horizontal(|ui| {
                if ui.small_button("✕").clicked() {
                    detach = Some(*pid);
                }
                ui.label(name);
            });
        }
        if members.is_empty() {
            ui.label(egui::RichText::new("(no members)").weak().small());
        }
        if let Some(pid) = detach {
            if let Some(p) = plan.persons.iter_mut().find(|p| p.id == pid) {
                p.group = None;
            }
        }
        ui.horizontal(|ui| {
            let mut attach: Option<Id> = None;
            egui::ComboBox::from_id_salt("add_member")
                .selected_text("➕ Add existing…")
                .show_ui(ui, |ui| {
                    for p in plan.persons.iter().filter(|p| p.group != Some(gid)) {
                        if ui.selectable_label(false, p.name.as_str()).clicked() {
                            attach = Some(p.id);
                        }
                    }
                });
            if let Some(pid) = attach {
                if let Some(p) = plan.persons.iter_mut().find(|p| p.id == pid) {
                    p.group = Some(gid);
                }
            }
            if ui.button("➕ New person").clicked() {
                let id = plan.new_id();
                plan.persons.push(Person {
                    id,
                    name: format!("Person {}", plan.persons.len() + 1),
                    group: Some(gid),
                });
            }
        });

        ui.separator();
        ui.label("Stays:");
        stay_editor(ui, plan, |s| s.subject == Subject::Group(gid), false, true);
        add_stay_button(ui, plan, Some(Subject::Group(gid)), None);
    }

    fn persons_editor(&mut self, ui: &mut egui::Ui) {
        let Self {
            plan,
            selected_person: selected,
            ..
        } = self;

        let items: Vec<(Id, String)> =
            plan.persons.iter().map(|p| (p.id, p.name.clone())).collect();
        let ids: Vec<Id> = items.iter().map(|(i, _)| *i).collect();
        ensure_selection(selected, &ids);

        ui.horizontal(|ui| {
            entity_selector(ui, "Person", selected, &items);
            if ui.button("➕ New").clicked() {
                let id = plan.new_id();
                plan.persons.push(Person {
                    id,
                    name: format!("Person {}", plan.persons.len() + 1),
                    group: None,
                });
                *selected = Some(id);
            }
        });

        let Some(pid) = *selected else {
            ui.label("No persons yet — add one.");
            return;
        };
        ui.separator();

        let groups: Vec<(Id, String)> =
            plan.groups.iter().map(|g| (g.id, g.name.clone())).collect();
        if let Some(person) = plan.persons.iter_mut().find(|p| p.id == pid) {
            ui.add(egui::TextEdit::singleline(&mut person.name).desired_width(200.0));
            let current = person
                .group
                .and_then(|gid| groups.iter().find(|(i, _)| *i == gid))
                .map(|(_, n)| n.as_str())
                .unwrap_or("— no group —");
            egui::ComboBox::from_label("Group")
                .selected_text(current)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut person.group, None, "— no group —");
                    for (gid, name) in &groups {
                        ui.selectable_value(&mut person.group, Some(*gid), name);
                    }
                });
        }
        if ui.button("🗑 Delete person").clicked() {
            plan.persons.retain(|p| p.id != pid);
            plan.stays.retain(|s| s.subject != Subject::Person(pid));
            *selected = None;
            return;
        }

        ui.separator();
        ui.label("Stays (individual):");
        stay_editor(ui, plan, |s| s.subject == Subject::Person(pid), false, true);
        add_stay_button(ui, plan, Some(Subject::Person(pid)), None);
    }

    fn housings_editor(&mut self, ui: &mut egui::Ui) {
        let Self {
            plan,
            selected_housing: selected,
            ..
        } = self;

        let items: Vec<(Id, String)> =
            plan.housings.iter().map(|h| (h.id, h.name.clone())).collect();
        let ids: Vec<Id> = items.iter().map(|(i, _)| *i).collect();
        ensure_selection(selected, &ids);

        ui.horizontal(|ui| {
            entity_selector(ui, "Housing", selected, &items);
            if ui.button("➕ New").clicked() {
                let id = plan.new_id();
                plan.housings.push(Housing {
                    id,
                    name: format!("Housing {}", plan.housings.len() + 1),
                    capacity: 2,
                    notes: String::new(),
                });
                *selected = Some(id);
            }
        });

        let Some(hid) = *selected else {
            ui.label("No housings yet — add one.");
            return;
        };
        ui.separator();

        if let Some(h) = plan.housings.iter_mut().find(|h| h.id == hid) {
            ui.add(egui::TextEdit::singleline(&mut h.name).desired_width(200.0));
            ui.horizontal(|ui| {
                ui.label("Capacity");
                ui.add(egui::DragValue::new(&mut h.capacity).range(0..=999));
            });
            ui.label("Notes:");
            ui.add(
                egui::TextEdit::multiline(&mut h.notes)
                    .desired_rows(2)
                    .desired_width(220.0),
            );
        }
        if ui.button("🗑 Delete housing").clicked() {
            plan.housings.retain(|h| h.id != hid);
            plan.stays.retain(|s| s.housing != hid);
            *selected = None;
            return;
        }

        ui.separator();
        ui.label("Stays:");
        stay_editor(ui, plan, |s| s.housing == hid, true, false);
        add_stay_button(ui, plan, None, Some(hid));
    }
}


/// Compact year / month / day editor for a [`NaiveDate`].
///
/// Uses three drag values so there is no parse state to manage and the result
/// is always a valid date (the day is clamped to the selected month's length).
fn date_edit(ui: &mut egui::Ui, date: &mut NaiveDate) {
    let mut y = date.year();
    let mut m = date.month() as i32;
    let mut d = date.day() as i32;

    ui.add(
        egui::DragValue::new(&mut y)
            .range(1900..=2200)
            .fixed_decimals(0),
    );
    ui.add(egui::DragValue::new(&mut m).range(1..=12).prefix("/"));
    ui.add(egui::DragValue::new(&mut d).range(1..=31).prefix("/"));

    let dim = days_in_month(y, m as u32) as i32;
    let d = d.clamp(1, dim) as u32;
    if let Some(nd) = NaiveDate::from_ymd_opt(y, m as u32, d) {
        *date = nd;
    }
}

/// Number of days in the given month.
fn days_in_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd_opt(ny, nm, 1)
        .and_then(|first_of_next| first_of_next.pred_opt())
        .map(|last| last.day())
        .unwrap_or(28)
}

/// Contents of the About / Licenses window.
fn about_contents(ui: &mut egui::Ui, logo: Option<&egui::TextureHandle>) {
    ui.horizontal(|ui| {
        if let Some(tex) = logo {
            ui.add(egui::Image::new((tex.id(), egui::vec2(72.0, 72.0))));
        }
        ui.vertical(|ui| {
            ui.heading("Housing Planner");
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            ui.label("Plan who stays where, and when.");
        });
    });
    ui.add_space(6.0);

    if ui.button("📋 Copy dependency licenses").clicked() {
        ui.ctx()
            .copy_text(licenses::dependency_licenses().to_owned());
    }
    ui.separator();

    egui::CollapsingHeader::new("This application")
        .default_open(false)
        .show(ui, |ui| {
            ui.label(egui::RichText::new(licenses::MAIN_LICENSE).small());
        });

    egui::CollapsingHeader::new("Third-party dependencies")
        .default_open(true)
        .show(ui, |ui| {
            // The license dump is large (tens of thousands of lines), so render
            // only the visible rows.
            let lines = licenses::dependency_license_lines();
            let row_h = ui.text_style_height(&egui::TextStyle::Monospace);
            egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                row_h,
                lines.len(),
                |ui, range| {
                    for line in &lines[range] {
                        ui.monospace(line);
                    }
                },
            );
        });
}

#[cfg(test)]
mod tests {
    #[test]
    fn embedded_icon_is_valid_png() {
        // The window icon and About-window logo both decode this; make sure the
        // committed asset stays a valid PNG.
        let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon-256.png"));
        assert!(
            icon.is_ok(),
            "icon-256.png failed to decode: {:?}",
            icon.err()
        );
        let icon = icon.unwrap();
        assert_eq!(icon.width, 256);
        assert_eq!(icon.height, 256);
    }
}

/// Ensure `sel` points at an existing id: clear it if the target is gone, then
/// default to the first available id.
fn ensure_selection(sel: &mut Option<Id>, ids: &[Id]) {
    if let Some(id) = *sel {
        if !ids.contains(&id) {
            *sel = None;
        }
    }
    if sel.is_none() {
        *sel = ids.first().copied();
    }
}

/// A labelled combo box for picking one entity by id.
fn entity_selector(ui: &mut egui::Ui, label: &str, sel: &mut Option<Id>, items: &[(Id, String)]) {
    let current = sel
        .and_then(|id| items.iter().find(|(i, _)| *i == id))
        .map(|(_, n)| n.as_str())
        .unwrap_or("—");
    egui::ComboBox::from_label(label)
        .selected_text(current)
        .show_ui(ui, |ui| {
            for (id, name) in items {
                ui.selectable_value(sel, Some(*id), name);
            }
        });
}

/// Centered, dimmed placeholder text for an empty central panel.
fn centered_hint(ui: &mut egui::Ui, text: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.label(egui::RichText::new(text).weak());
    });
}

/// First person, else first group — used as the default subject for a new stay.
fn default_subject(plan: &Plan) -> Option<Subject> {
    plan.persons
        .first()
        .map(|p| Subject::Person(p.id))
        .or_else(|| plan.groups.first().map(|g| Subject::Group(g.id)))
}

/// "Add stay" button. `subject`/`housing` pin those fields when `Some`, otherwise
/// a sensible default is chosen; disabled (with a hint) if no default exists.
fn add_stay_button(ui: &mut egui::Ui, plan: &mut Plan, subject: Option<Subject>, housing: Option<Id>) {
    let subject = subject.or_else(|| default_subject(plan));
    let housing = housing.or_else(|| plan.housings.first().map(|h| h.id));
    let enabled = subject.is_some() && housing.is_some();
    ui.add_enabled_ui(enabled, |ui| {
        if ui.button("➕ Add stay").clicked() {
            let id = plan.new_id();
            let today = chrono::Local::now().date_naive();
            plan.stays.push(Stay {
                id,
                subject: subject.unwrap(),
                housing: housing.unwrap(),
                arrival: today,
                departure: today + Duration::days(7),
            });
        }
    });
    if !enabled {
        ui.label(
            egui::RichText::new("Add a housing and a person/group first.")
                .small()
                .weak(),
        );
    }
}

/// Edit the stays matching `matches`. `edit_subject` / `edit_housing` show those
/// combos (the field that's fixed for the current tab is hidden).
fn stay_editor(
    ui: &mut egui::Ui,
    plan: &mut Plan,
    matches: impl Fn(&Stay) -> bool,
    edit_subject: bool,
    edit_housing: bool,
) {
    let housings: Vec<(Id, String)> =
        plan.housings.iter().map(|h| (h.id, h.name.clone())).collect();
    let persons: Vec<(Id, String)> =
        plan.persons.iter().map(|p| (p.id, p.name.clone())).collect();
    let groups: Vec<(Id, String)> = plan.groups.iter().map(|g| (g.id, g.name.clone())).collect();

    let subject_label = |s: Subject| -> String {
        match s {
            Subject::Person(id) => persons
                .iter()
                .find(|(i, _)| *i == id)
                .map(|(_, n)| n.clone())
                .unwrap_or_else(|| "<person>".into()),
            Subject::Group(id) => groups
                .iter()
                .find(|(i, _)| *i == id)
                .map(|(_, n)| format!("{} (group)", n))
                .unwrap_or_else(|| "<group>".into()),
        }
    };
    let housing_label = |id: Id| -> String {
        housings
            .iter()
            .find(|(i, _)| *i == id)
            .map(|(_, n)| n.clone())
            .unwrap_or_else(|| "<housing>".into())
    };

    let mut delete: Option<Id> = None;
    let mut any = false;
    for stay in plan.stays.iter_mut() {
        if !matches(stay) {
            continue;
        }
        any = true;
        ui.push_id(stay.id, |ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    if edit_subject {
                        egui::ComboBox::from_id_salt("subj")
                            .selected_text(subject_label(stay.subject))
                            .show_ui(ui, |ui| {
                                for (id, name) in &persons {
                                    ui.selectable_value(&mut stay.subject, Subject::Person(*id), name);
                                }
                                for (id, name) in &groups {
                                    ui.selectable_value(
                                        &mut stay.subject,
                                        Subject::Group(*id),
                                        format!("{} (group)", name),
                                    );
                                }
                            });
                    }
                    if edit_housing {
                        egui::ComboBox::from_id_salt("house")
                            .selected_text(housing_label(stay.housing))
                            .show_ui(ui, |ui| {
                                for (id, name) in &housings {
                                    ui.selectable_value(&mut stay.housing, *id, name);
                                }
                            });
                    }
                    if ui.button("🗑").clicked() {
                        delete = Some(stay.id);
                    }
                });
                ui.horizontal(|ui| {
                    date_edit(ui, &mut stay.arrival);
                    ui.label("→");
                    date_edit(ui, &mut stay.departure);
                    if stay.departure < stay.arrival {
                        stay.departure = stay.arrival;
                    }
                });
            });
        });
    }
    if !any {
        ui.label(egui::RichText::new("(no stays)").weak().small());
    }
    if let Some(id) = delete {
        plan.stays.retain(|s| s.id != id);
    }
}
