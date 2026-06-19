//! The eframe application: a management side panel plus the timeline view.

use chrono::{Datelike, Duration, NaiveDate};

use crate::model::{Group, Housing, Id, Person, Plan, Stay, Subject, GROUP_PALETTE};
use crate::timeline;

/// Storage key used for `eframe`'s built-in cross-platform persistence.
const STORAGE_KEY: &str = "hplan_plan";

pub struct PlannerApp {
    plan: Plan,
    // --- View state (not persisted) ---
    view_start: NaiveDate,
    days_visible: i64,
    day_width: f32,
    /// Sub-day remainder accumulated while drag-panning the timeline.
    pan_remainder: f32,
    /// Transient status line (e.g. result of the last file save/load).
    status: String,
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

        Self {
            plan,
            view_start,
            days_visible: 30,
            day_width: 26.0,
            pan_remainder: 0.0,
            status: String::new(),
        }
    }
}

impl eframe::App for PlannerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, STORAGE_KEY, &self.plan);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.top_bar(ui);
        self.side_panel(ui);

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Vertical scroll for many housings; horizontal movement is done by
            // dragging the canvas (which pans the date window).
            egui::ScrollArea::vertical().show(ui, |ui| {
                let response = timeline::show(
                    ui,
                    &self.plan,
                    self.view_start,
                    self.days_visible,
                    self.day_width,
                );
                self.handle_pan(ui, &response);
            });
        });
    }
}

impl PlannerApp {
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
            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("hplan");
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
                ui.add(egui::Slider::new(&mut self.day_width, 8.0..=60.0).show_value(false));

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
            });
        });
    }

    /// Prompt for a path and write the current plan as JSON.
    #[cfg(not(target_os = "android"))]
    fn save_to_file(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("hplan plan", &["json"])
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
            .add_filter("hplan plan", &["json"])
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

    fn side_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("manage")
            .resizable(true)
            .default_size(320.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let plan = &mut self.plan;

                    if plan.is_empty() && ui.button("📋 Load example data").clicked() {
                        plan.load_sample();
                    }

                    egui::CollapsingHeader::new("🏠 Housings")
                        .default_open(true)
                        .show(ui, |ui| housings_ui(ui, plan));

                    egui::CollapsingHeader::new("👥 Groups")
                        .default_open(true)
                        .show(ui, |ui| groups_ui(ui, plan));

                    egui::CollapsingHeader::new("🧍 Persons")
                        .default_open(true)
                        .show(ui, |ui| persons_ui(ui, plan));

                    egui::CollapsingHeader::new("📅 Stays")
                        .default_open(true)
                        .show(ui, |ui| stays_ui(ui, plan));
                });
            });
    }
}

fn housings_ui(ui: &mut egui::Ui, plan: &mut Plan) {
    let mut delete: Option<Id> = None;
    for housing in &mut plan.housings {
        ui.push_id(housing.id, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut housing.name).desired_width(140.0));
                ui.label("cap");
                ui.add(egui::DragValue::new(&mut housing.capacity).range(0..=999));
                if ui.button("🗑").clicked() {
                    delete = Some(housing.id);
                }
            });
        });
    }
    if let Some(id) = delete {
        plan.housings.retain(|h| h.id != id);
        plan.stays.retain(|s| s.housing != id);
    }
    if ui.button("➕ Add housing").clicked() {
        let id = plan.new_id();
        plan.housings.push(Housing {
            id,
            name: format!("Housing {}", plan.housings.len() + 1),
            capacity: 2,
            notes: String::new(),
        });
    }
}

fn groups_ui(ui: &mut egui::Ui, plan: &mut Plan) {
    let mut delete: Option<Id> = None;
    for group in &mut plan.groups {
        ui.push_id(group.id, |ui| {
            ui.horizontal(|ui| {
                ui.color_edit_button_srgb(&mut group.color);
                ui.add(egui::TextEdit::singleline(&mut group.name).desired_width(170.0));
                if ui.button("🗑").clicked() {
                    delete = Some(group.id);
                }
            });
        });
    }
    if let Some(id) = delete {
        plan.groups.retain(|g| g.id != id);
        // Detach members and drop group stays.
        for p in &mut plan.persons {
            if p.group == Some(id) {
                p.group = None;
            }
        }
        plan.stays.retain(|s| s.subject != Subject::Group(id));
    }
    if ui.button("➕ Add group").clicked() {
        let id = plan.new_id();
        let color = GROUP_PALETTE[plan.groups.len() % GROUP_PALETTE.len()];
        plan.groups.push(Group {
            id,
            name: format!("Group {}", plan.groups.len() + 1),
            color,
        });
    }
}

fn persons_ui(ui: &mut egui::Ui, plan: &mut Plan) {
    // Snapshot groups for the combo box so we can mutate persons freely.
    let groups: Vec<(Id, String)> = plan.groups.iter().map(|g| (g.id, g.name.clone())).collect();

    let mut delete: Option<Id> = None;
    for person in &mut plan.persons {
        ui.push_id(person.id, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut person.name).desired_width(130.0));

                let current = person
                    .group
                    .and_then(|gid| groups.iter().find(|(id, _)| *id == gid))
                    .map(|(_, name)| name.as_str())
                    .unwrap_or("— no group —");
                egui::ComboBox::from_id_salt("grp")
                    .selected_text(current)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut person.group, None, "— no group —");
                        for (gid, name) in &groups {
                            ui.selectable_value(&mut person.group, Some(*gid), name);
                        }
                    });

                if ui.button("🗑").clicked() {
                    delete = Some(person.id);
                }
            });
        });
    }
    if let Some(id) = delete {
        plan.persons.retain(|p| p.id != id);
        plan.stays.retain(|s| s.subject != Subject::Person(id));
    }
    if ui.button("➕ Add person").clicked() {
        let id = plan.new_id();
        plan.persons.push(Person {
            id,
            name: format!("Person {}", plan.persons.len() + 1),
            group: None,
        });
    }
}

fn stays_ui(ui: &mut egui::Ui, plan: &mut Plan) {
    // Snapshots for the combo boxes.
    let housings: Vec<(Id, String)> = plan.housings.iter().map(|h| (h.id, h.name.clone())).collect();
    let persons: Vec<(Id, String)> = plan.persons.iter().map(|p| (p.id, p.name.clone())).collect();
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
    for stay in &mut plan.stays {
        ui.push_id(stay.id, |ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
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
                    ui.label("→");
                    egui::ComboBox::from_id_salt("house")
                        .selected_text(housing_label(stay.housing))
                        .show_ui(ui, |ui| {
                            for (id, name) in &housings {
                                ui.selectable_value(&mut stay.housing, *id, name);
                            }
                        });
                    if ui.button("🗑").clicked() {
                        delete = Some(stay.id);
                    }
                });
                ui.horizontal(|ui| {
                    date_edit(ui, &mut stay.arrival);
                    ui.label("→");
                    date_edit(ui, &mut stay.departure);
                    // Keep the range sane.
                    if stay.departure < stay.arrival {
                        stay.departure = stay.arrival;
                    }
                });
            });
        });
    }
    if let Some(id) = delete {
        plan.stays.retain(|s| s.id != id);
    }

    // Adding a stay needs at least one housing and one subject.
    let default_subject = persons
        .first()
        .map(|(id, _)| Subject::Person(*id))
        .or_else(|| groups.first().map(|(id, _)| Subject::Group(*id)));
    let can_add = !housings.is_empty() && default_subject.is_some();

    ui.add_enabled_ui(can_add, |ui| {
        if ui.button("➕ Add stay").clicked() {
            let id = plan.new_id();
            let today = chrono::Local::now().date_naive();
            plan.stays.push(Stay {
                id,
                subject: default_subject.unwrap(),
                housing: housings[0].0,
                arrival: today,
                departure: today + Duration::days(7),
            });
        }
    });
    if !can_add {
        ui.label(
            egui::RichText::new("Add a housing and a person/group first.")
                .small()
                .weak(),
        );
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

    ui.add(egui::DragValue::new(&mut y).range(1900..=2200).fixed_decimals(0));
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
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd_opt(ny, nm, 1)
        .and_then(|first_of_next| first_of_next.pred_opt())
        .map(|last| last.day())
        .unwrap_or(28)
}
