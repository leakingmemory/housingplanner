//! The eframe application: a management side panel plus the timeline view.

use chrono::{Datelike, Duration, NaiveDate};

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::i18n::{tr, Lang};
use crate::journal::{self, InverseOp};
use crate::licenses;
use crate::model::{Group, Housing, Id, LogKind, Person, Plan, Stay, Subject, GROUP_PALETTE};
use crate::timeline;

/// The active workspace tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Overview,
    Groups,
    Persons,
    Housings,
    Changelog,
}

/// Storage key used for `eframe`'s built-in cross-platform persistence.
/// Kept as the original value so plans saved before the project rename still
/// load (only the migrated storage directory changed).
const STORAGE_KEY: &str = "hplan_plan";

/// Storage key for the selected interface language.
const LANG_KEY: &str = "hplan_lang";

/// Storage key for the path of the file currently open (reopened on next launch).
const FILE_KEY: &str = "hplan_current_file";

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
    /// Interface language (persisted).
    lang: Lang,
    // --- Tab navigation + per-tab selection (not persisted) ---
    active_tab: Tab,
    selected_group: Option<Id>,
    selected_person: Option<Id>,
    selected_housing: Option<Id>,
    // --- Change journal (not persisted; the journal itself lives in `plan`) ---
    /// Last-journaled plan content; diffed against `plan` to detect changes.
    baseline: Plan,
    /// Session undo stack: (journal entry id, how to revert it). Cleared on load.
    undo_stack: Vec<(u64, InverseOp)>,
    // --- Current file + unsaved-changes tracking ---
    /// File currently open (saved-to / loaded-from); reopened on next launch.
    current_file: Option<PathBuf>,
    /// Plan content as of the last save/open; `plan` differing from it means dirty.
    saved_baseline: Plan,
    /// Close-confirmation flow.
    pending_close: bool,
    allow_close: bool,
    /// When discarding on close, persist `saved_baseline` (not the dirty plan).
    discarding: bool,
}

impl PlannerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // The file we had open last time (reopened below, taking precedence).
        let current_file: Option<PathBuf> = cc
            .storage
            .and_then(|s| eframe::get_value::<String>(s, FILE_KEY))
            .map(PathBuf::from);

        // Prefer reopening that file; fall back to the auto-stored plan.
        let plan = current_file
            .as_ref()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|t| serde_json::from_str::<Plan>(&t).ok())
            .map(|mut p| {
                p.reseed_ids();
                p
            })
            .or_else(|| {
                cc.storage
                    .and_then(|s| eframe::get_value::<Plan>(s, STORAGE_KEY))
            })
            .unwrap_or_default();

        let today = chrono::Local::now().date_naive();
        let view_start = plan
            .earliest_arrival()
            .map(|d| d - Duration::days(2))
            .unwrap_or(today);

        // Persisted language, defaulting to the system locale on first run.
        let lang = cc
            .storage
            .and_then(|s| eframe::get_value::<Lang>(s, LANG_KEY))
            .unwrap_or_else(Lang::from_env);

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
            baseline: plan.clone(),
            saved_baseline: plan.clone(),
            current_file,
            pending_close: false,
            allow_close: false,
            discarding: false,
            plan,
            view_start,
            days_visible: 30,
            day_width: 26.0,
            pan_remainder: 0.0,
            zoom_remainder: 0.0,
            status: String::new(),
            licenses_open: false,
            logo,
            lang,
            active_tab: Tab::Overview,
            selected_group: None,
            selected_person: None,
            selected_housing: None,
            undo_stack: Vec::new(),
        }
    }
}

impl eframe::App for PlannerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // On a "Discard" close, persist the last-saved content, not the dirty plan.
        let plan = if self.discarding {
            &self.saved_baseline
        } else {
            &self.plan
        };
        eframe::set_value(storage, STORAGE_KEY, plan);
        eframe::set_value(storage, LANG_KEY, &self.lang);
        if let Some(p) = &self.current_file {
            eframe::set_value(storage, FILE_KEY, &p.to_string_lossy().to_string());
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.top_bar(ui);
        self.licenses_window(ui.ctx());

        let lang = self.lang;
        egui::Panel::top("tabs").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Overview, tr(lang, "📊 Overview"));
                ui.selectable_value(&mut self.active_tab, Tab::Groups, tr(lang, "👥 Groups"));
                ui.selectable_value(&mut self.active_tab, Tab::Persons, tr(lang, "🧍 Persons"));
                ui.selectable_value(&mut self.active_tab, Tab::Housings, tr(lang, "🏠 Housings"));
                ui.selectable_value(
                    &mut self.active_tab,
                    Tab::Changelog,
                    tr(lang, "📜 Changelog"),
                );
            });
        });

        match self.active_tab {
            Tab::Overview => self.overview_tab(ui),
            Tab::Groups => self.groups_tab(ui),
            Tab::Persons => self.persons_tab(ui),
            Tab::Housings => self.housings_tab(ui),
            Tab::Changelog => self.changelog_tab(ui),
        }

        // After the editors have run, journal any committed edits.
        self.commit_changes(ui.ctx());

        // Intercept window close while there are unsaved changes.
        self.handle_close(ui.ctx());
        self.close_dialog(ui.ctx());
    }
}

impl PlannerApp {
    /// Diff the plan against the baseline at a commit boundary and journal any
    /// changes. Skipped while the user is mid-edit (a text field has focus or a
    /// drag is in progress) so an edit is logged once, on commit.
    fn commit_changes(&mut self, ctx: &egui::Context) {
        if ctx.egui_wants_keyboard_input() || ctx.egui_is_using_pointer() {
            return;
        }
        let changes = journal::diff(&self.baseline, &self.plan);
        if changes.is_empty() {
            return;
        }
        for ch in changes {
            let id = self.plan.push_log(ch.kind);
            self.undo_stack.push((id, ch.inverse));
        }
        self.baseline = self.plan.clone();
    }

    /// Revert the most recent change and log an `Undo` entry referencing it.
    fn undo_last(&mut self) {
        if let Some((id, inverse)) = self.undo_stack.pop() {
            journal::apply_inverse(&mut self.plan, &inverse);
            self.plan.push_log(LogKind::Undo { undoes: id });
            self.baseline = self.plan.clone();
        }
    }

    /// Load the bundled example, logged as a single event (not entity-by-entity).
    fn load_example(&mut self) {
        self.plan.load_sample();
        self.plan.push_log(LogKind::LoadedExample);
        self.baseline = self.plan.clone();
    }

    /// True if the plan differs from the last saved/opened file content.
    fn is_dirty(&self) -> bool {
        !self.plan.content_eq(&self.saved_baseline)
    }

    /// Intercept a window-close request when there are unsaved changes.
    fn handle_close(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allow_close || !self.is_dirty() {
                return; // let the close proceed
            }
            self.pending_close = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }
    }

    /// The "save before closing?" modal, shown while a close is pending.
    fn close_dialog(&mut self, ctx: &egui::Context) {
        if !self.pending_close {
            return;
        }
        let lang = self.lang;
        egui::Modal::new(egui::Id::new("unsaved_close")).show(ctx, |ui| {
            ui.set_width(360.0);
            ui.heading(tr(lang, "Unsaved changes"));
            ui.label(tr(lang, "You have unsaved changes. Save before closing?"));
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button(tr(lang, "Save")).clicked() {
                    self.save_current();
                    if !self.is_dirty() {
                        self.allow_close = true;
                        self.pending_close = false;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                if ui.button(tr(lang, "Discard")).clicked() {
                    self.discarding = true;
                    self.allow_close = true;
                    self.pending_close = false;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ui.button(tr(lang, "Cancel")).clicked() {
                    self.pending_close = false;
                }
            });
        });
    }

    fn changelog_tab(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
        egui::Panel::top("changelog_top").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_enabled_ui(!self.undo_stack.is_empty(), |ui| {
                    if ui.button(tr(lang, "↩ Undo last change")).clicked() {
                        self.undo_last();
                    }
                });
                ui.separator();
                ui.label(format!(
                    "{} {}",
                    self.plan.changelog.len(),
                    tr(lang, "entries")
                ));
            });
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if self.plan.changelog.is_empty() {
                centered_hint(ui, tr(lang, "No changes yet."));
                return;
            }
            // ids referenced by a later Undo entry → shown as undone.
            let undone: HashSet<u64> = self
                .plan
                .changelog
                .iter()
                .filter_map(|e| match e.kind {
                    LogKind::Undo { undoes } => Some(undoes),
                    _ => None,
                })
                .collect();
            // Newest first.
            let rows: Vec<(String, String, bool, bool)> = self
                .plan
                .changelog
                .iter()
                .rev()
                .map(|e| {
                    (
                        short_time(&e.time),
                        journal::describe(e, &self.plan.changelog, lang),
                        undone.contains(&e.id),
                        matches!(e.kind, LogKind::Undo { .. }),
                    )
                })
                .collect();
            let row_h = ui.text_style_height(&egui::TextStyle::Body) + 6.0;
            egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                row_h,
                rows.len(),
                |ui, range| {
                    for (time, desc, was_undone, is_undo) in &rows[range] {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(time).weak().monospace());
                            let mut rt = egui::RichText::new(desc);
                            if *was_undone {
                                rt = rt.strikethrough().weak();
                            }
                            if *is_undo {
                                rt = rt.italics();
                            }
                            ui.label(rt);
                        });
                    }
                },
            );
        });
    }

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
        let lang = self.lang;
        egui::Panel::top("top").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Housing Planner");
                ui.separator();

                ui.label(tr(lang, "From:"));
                date_edit(ui, &mut self.view_start);

                ui.label(tr(lang, "Days:"));
                ui.add(
                    egui::DragValue::new(&mut self.days_visible)
                        .range(7..=365)
                        .speed(1.0),
                );

                ui.label(tr(lang, "Zoom:"));
                ui.add(
                    egui::Slider::new(&mut self.day_width, MIN_DAY_WIDTH..=MAX_DAY_WIDTH)
                        .show_value(false),
                )
                .on_hover_text(tr(
                    lang,
                    "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline",
                ));

                if ui.button(tr(lang, "Today")).clicked() {
                    self.view_start = chrono::Local::now().date_naive();
                }
                if ui.button(tr(lang, "Fit to stays")).clicked() {
                    if let Some(first) = self.plan.earliest_arrival() {
                        self.view_start = first - Duration::days(2);
                    }
                }

                ui.separator();
                if ui.button(tr(lang, "💾 Save")).clicked() {
                    self.save_current();
                }
                if ui.button(tr(lang, "Save As…")).clicked() {
                    self.save_as();
                }
                if ui.button(tr(lang, "📂 Load…")).clicked() {
                    self.load_from_file();
                }

                // Current file name + an unsaved-changes dot.
                ui.separator();
                let name = self
                    .current_file
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| tr(lang, "untitled").to_owned());
                let dirty = self.is_dirty();
                let label = if dirty { format!("{name} ●") } else { name };
                let resp = ui.label(egui::RichText::new(label).weak());
                if dirty {
                    resp.on_hover_text(tr(lang, "Unsaved changes"));
                }

                if !self.status.is_empty() {
                    ui.separator();
                    ui.label(egui::RichText::new(&self.status).weak());
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tr(lang, "ℹ About")).clicked() {
                        self.licenses_open = true;
                    }
                    egui::ComboBox::from_id_salt("lang")
                        .selected_text(format!("🌐 {}", self.lang.label()))
                        .show_ui(ui, |ui| {
                            for l in Lang::ALL {
                                ui.selectable_value(&mut self.lang, l, l.label());
                            }
                        })
                        .response
                        .on_hover_text(tr(lang, "Language"));
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
        let lang = self.lang;
        egui::Window::new(tr(lang, "About / Licenses"))
            .open(&mut self.licenses_open)
            .resizable(true)
            .default_size([720.0, 560.0])
            .show(ctx, |ui| about_contents(ui, logo.as_ref(), lang));
    }

    /// Write the current plan as JSON to `path`. On success this becomes the
    /// current file and the saved baseline (clearing the dirty state).
    #[cfg(not(target_os = "android"))]
    fn write_to(&mut self, path: &Path) -> bool {
        let lang = self.lang;
        match serde_json::to_string_pretty(&self.plan) {
            Ok(json) => match std::fs::write(path, json) {
                Ok(()) => {
                    self.current_file = Some(path.to_path_buf());
                    self.saved_baseline = self.plan.clone();
                    self.status = format!("{} {}", tr(lang, "Saved →"), path.display());
                    true
                }
                Err(e) => {
                    self.status = format!("{} {e}", tr(lang, "Save failed:"));
                    false
                }
            },
            Err(e) => {
                self.status = format!("{} {e}", tr(lang, "Encode failed:"));
                false
            }
        }
    }

    /// Save to the current file if one is open, otherwise prompt (Save As).
    #[cfg(not(target_os = "android"))]
    fn save_current(&mut self) {
        match self.current_file.clone() {
            Some(path) => {
                self.write_to(&path);
            }
            None => self.save_as(),
        }
    }

    /// Always prompt for a path, then save there.
    #[cfg(not(target_os = "android"))]
    fn save_as(&mut self) {
        let lang = self.lang;
        let start_name = self
            .current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "plan.json".to_owned());
        let Some(path) = rfd::FileDialog::new()
            .add_filter(tr(lang, "Housing Planner plan"), &["json"])
            .set_file_name(start_name)
            .save_file()
        else {
            return; // user cancelled
        };
        self.write_to(&path);
    }

    /// Prompt for a path and replace the current plan with one loaded from JSON.
    #[cfg(not(target_os = "android"))]
    fn load_from_file(&mut self) {
        let lang = self.lang;
        let Some(path) = rfd::FileDialog::new()
            .add_filter(tr(lang, "Housing Planner plan"), &["json"])
            .pick_file()
        else {
            return; // user cancelled
        };
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                self.status = format!("{} {e}", tr(lang, "Read failed:"));
                return;
            }
        };
        // Detect whether the file already carried a change journal.
        let had_history = serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .and_then(|v| v.get("changelog").cloned())
            .is_some();
        match serde_json::from_str::<crate::model::Plan>(&text) {
            Ok(mut plan) => {
                plan.reseed_ids();
                self.view_start = plan
                    .earliest_arrival()
                    .map(|d| d - Duration::days(2))
                    .unwrap_or_else(|| chrono::Local::now().date_naive());
                self.plan = plan;
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                self.plan.push_log(if had_history {
                    LogKind::LoadedFile { name }
                } else {
                    LogKind::LoadedFileNoHistory { name }
                });
                // This becomes the current file; nothing unsaved yet.
                self.current_file = Some(path.clone());
                self.saved_baseline = self.plan.clone();
                // Session undo restarts for the freshly loaded plan.
                self.undo_stack.clear();
                self.baseline = self.plan.clone();
                self.status = format!("{} {}", tr(lang, "Loaded ←"), path.display());
            }
            Err(e) => self.status = format!("{} {e}", tr(lang, "Parse failed:")),
        }
    }

    // On Android the native file picker needs the activity/intents plumbing;
    // for now these are no-ops there (auto-persistence still applies).
    #[cfg(target_os = "android")]
    fn save_current(&mut self) {
        self.status = tr(self.lang, "File save is not available on Android yet.").to_owned();
    }
    #[cfg(target_os = "android")]
    fn save_as(&mut self) {
        self.save_current();
    }
    #[cfg(target_os = "android")]
    fn load_from_file(&mut self) {
        self.status = tr(self.lang, "File load is not available on Android yet.").to_owned();
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
                    self.lang,
                    &filter,
                );
                self.handle_zoom(ui, &response);
                self.handle_pan(ui, &response);
            });
        });
    }

    fn overview_tab(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
        if self.plan.is_empty() {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading(tr(lang, "Welcome to Housing Planner"));
                    ui.label(tr(
                        lang,
                        "Add housings, groups and people in the tabs above —",
                    ));
                    ui.add_space(6.0);
                    if ui.button(tr(lang, "📋 Load example data")).clicked() {
                        self.load_example();
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
            tr(lang, "Add a housing in the Housings tab to start planning."),
        );
    }

    fn groups_tab(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
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
            self.timeline_panel(
                ui,
                &housings,
                &include,
                tr(lang, "No stays for this group yet."),
            );
        } else {
            let hint = tr(lang, "Select or create a group.");
            egui::CentralPanel::default().show_inside(ui, |ui| centered_hint(ui, hint));
        }
    }

    fn persons_tab(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
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
            self.timeline_panel(
                ui,
                &housings,
                &include,
                tr(lang, "No stays for this person yet."),
            );
        } else {
            let hint = tr(lang, "Select or create a person.");
            egui::CentralPanel::default().show_inside(ui, |ui| centered_hint(ui, hint));
        }
    }

    fn housings_tab(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
        egui::Panel::left("housings_panel")
            .resizable(true)
            .default_size(340.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| self.housings_editor(ui));
            });

        if let Some(hid) = self.selected_housing {
            let housings = [hid];
            self.timeline_panel(
                ui,
                &housings,
                &|_| true,
                tr(lang, "No stays in this housing yet."),
            );
        } else {
            let hint = tr(lang, "Select or create a housing.");
            egui::CentralPanel::default().show_inside(ui, |ui| centered_hint(ui, hint));
        }
    }

    fn groups_editor(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
        let Self {
            plan,
            selected_group: selected,
            ..
        } = self;

        let items: Vec<(Id, String)> = plan.groups.iter().map(|g| (g.id, g.name.clone())).collect();
        let ids: Vec<Id> = items.iter().map(|(i, _)| *i).collect();
        ensure_selection(selected, &ids);

        ui.horizontal(|ui| {
            entity_selector(ui, tr(lang, "Group"), selected, &items);
            if ui.button(tr(lang, "➕ New")).clicked() {
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
            ui.label(tr(lang, "No groups yet — add one."));
            return;
        };
        ui.separator();

        if let Some(group) = plan.groups.iter_mut().find(|g| g.id == gid) {
            ui.horizontal(|ui| {
                ui.color_edit_button_srgb(&mut group.color);
                ui.add(egui::TextEdit::singleline(&mut group.name).desired_width(200.0));
            });
        }
        if ui.button(tr(lang, "🗑 Delete group")).clicked() {
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
        ui.label(tr(lang, "Members:"));
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
            ui.label(egui::RichText::new(tr(lang, "(no members)")).weak().small());
        }
        if let Some(pid) = detach {
            if let Some(p) = plan.persons.iter_mut().find(|p| p.id == pid) {
                p.group = None;
            }
        }
        ui.horizontal(|ui| {
            let mut attach: Option<Id> = None;
            egui::ComboBox::from_id_salt("add_member")
                .selected_text(tr(lang, "➕ Add existing…"))
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
            if ui.button(tr(lang, "➕ New person")).clicked() {
                let id = plan.new_id();
                plan.persons.push(Person {
                    id,
                    name: format!("Person {}", plan.persons.len() + 1),
                    group: Some(gid),
                });
            }
        });

        ui.separator();
        ui.label(tr(lang, "Stays:"));
        stay_editor(
            ui,
            plan,
            lang,
            |s| s.subject == Subject::Group(gid),
            false,
            true,
        );
        add_stay_button(ui, plan, lang, Some(Subject::Group(gid)), None);
    }

    fn persons_editor(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
        let Self {
            plan,
            selected_person: selected,
            ..
        } = self;

        let items: Vec<(Id, String)> = plan
            .persons
            .iter()
            .map(|p| (p.id, p.name.clone()))
            .collect();
        let ids: Vec<Id> = items.iter().map(|(i, _)| *i).collect();
        ensure_selection(selected, &ids);

        ui.horizontal(|ui| {
            entity_selector(ui, tr(lang, "Person"), selected, &items);
            if ui.button(tr(lang, "➕ New")).clicked() {
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
            ui.label(tr(lang, "No persons yet — add one."));
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
                .unwrap_or_else(|| tr(lang, "— no group —"));
            egui::ComboBox::from_label(tr(lang, "Group"))
                .selected_text(current)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut person.group, None, tr(lang, "— no group —"));
                    for (gid, name) in &groups {
                        ui.selectable_value(&mut person.group, Some(*gid), name);
                    }
                });
        }
        if ui.button(tr(lang, "🗑 Delete person")).clicked() {
            plan.persons.retain(|p| p.id != pid);
            plan.stays.retain(|s| s.subject != Subject::Person(pid));
            *selected = None;
            return;
        }

        ui.separator();
        ui.label(tr(lang, "Stays (individual):"));
        stay_editor(
            ui,
            plan,
            lang,
            |s| s.subject == Subject::Person(pid),
            false,
            true,
        );
        add_stay_button(ui, plan, lang, Some(Subject::Person(pid)), None);
    }

    fn housings_editor(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
        let Self {
            plan,
            selected_housing: selected,
            ..
        } = self;

        let items: Vec<(Id, String)> = plan
            .housings
            .iter()
            .map(|h| (h.id, h.name.clone()))
            .collect();
        let ids: Vec<Id> = items.iter().map(|(i, _)| *i).collect();
        ensure_selection(selected, &ids);

        ui.horizontal(|ui| {
            entity_selector(ui, tr(lang, "Housing"), selected, &items);
            if ui.button(tr(lang, "➕ New")).clicked() {
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
            ui.label(tr(lang, "No housings yet — add one."));
            return;
        };
        ui.separator();

        if let Some(h) = plan.housings.iter_mut().find(|h| h.id == hid) {
            ui.add(egui::TextEdit::singleline(&mut h.name).desired_width(200.0));
            ui.horizontal(|ui| {
                ui.label(tr(lang, "Capacity"));
                ui.add(egui::DragValue::new(&mut h.capacity).range(0..=999));
            });
            ui.label(tr(lang, "Notes:"));
            ui.add(
                egui::TextEdit::multiline(&mut h.notes)
                    .desired_rows(2)
                    .desired_width(220.0),
            );
        }
        if ui.button(tr(lang, "🗑 Delete housing")).clicked() {
            plan.housings.retain(|h| h.id != hid);
            plan.stays.retain(|s| s.housing != hid);
            *selected = None;
            return;
        }

        ui.separator();
        ui.label(tr(lang, "Stays:"));
        stay_editor(ui, plan, lang, |s| s.housing == hid, true, false);
        add_stay_button(ui, plan, lang, None, Some(hid));
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
/// Compact "MM-DD HH:MM" rendering of an RFC 3339 timestamp for the changelog.
fn short_time(rfc3339: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(rfc3339)
        .map(|dt| dt.format("%m-%d %H:%M").to_string())
        .unwrap_or_else(|_| rfc3339.to_owned())
}

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
fn about_contents(ui: &mut egui::Ui, logo: Option<&egui::TextureHandle>, lang: Lang) {
    ui.horizontal(|ui| {
        if let Some(tex) = logo {
            ui.add(egui::Image::new((tex.id(), egui::vec2(72.0, 72.0))));
        }
        ui.vertical(|ui| {
            ui.heading("Housing Planner");
            ui.label(format!(
                "{} {}",
                tr(lang, "Version"),
                env!("CARGO_PKG_VERSION")
            ));
            ui.label(tr(lang, "Plan who stays where, and when."));
        });
    });
    ui.add_space(6.0);

    if ui.button(tr(lang, "📋 Copy dependency licenses")).clicked() {
        ui.ctx()
            .copy_text(licenses::dependency_licenses().to_owned());
    }
    ui.separator();

    egui::CollapsingHeader::new(tr(lang, "This application"))
        .default_open(false)
        .show(ui, |ui| {
            ui.label(egui::RichText::new(licenses::MAIN_LICENSE).small());
        });

    egui::CollapsingHeader::new(tr(lang, "Third-party dependencies"))
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
fn add_stay_button(
    ui: &mut egui::Ui,
    plan: &mut Plan,
    lang: Lang,
    subject: Option<Subject>,
    housing: Option<Id>,
) {
    let subject = subject.or_else(|| default_subject(plan));
    let housing = housing.or_else(|| plan.housings.first().map(|h| h.id));
    let enabled = subject.is_some() && housing.is_some();
    ui.add_enabled_ui(enabled, |ui| {
        if ui.button(tr(lang, "➕ Add stay")).clicked() {
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
            egui::RichText::new(tr(lang, "Add a housing and a person/group first."))
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
    lang: Lang,
    matches: impl Fn(&Stay) -> bool,
    edit_subject: bool,
    edit_housing: bool,
) {
    let housings: Vec<(Id, String)> = plan
        .housings
        .iter()
        .map(|h| (h.id, h.name.clone()))
        .collect();
    let persons: Vec<(Id, String)> = plan
        .persons
        .iter()
        .map(|p| (p.id, p.name.clone()))
        .collect();
    let groups: Vec<(Id, String)> = plan.groups.iter().map(|g| (g.id, g.name.clone())).collect();
    let group_suffix = tr(lang, "(group)");

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
                .map(|(_, n)| format!("{} {}", n, group_suffix))
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
                                    ui.selectable_value(
                                        &mut stay.subject,
                                        Subject::Person(*id),
                                        name,
                                    );
                                }
                                for (id, name) in &groups {
                                    ui.selectable_value(
                                        &mut stay.subject,
                                        Subject::Group(*id),
                                        format!("{} {}", name, group_suffix),
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
        ui.label(egui::RichText::new(tr(lang, "(no stays)")).weak().small());
    }
    if let Some(id) = delete {
        plan.stays.retain(|s| s.id != id);
    }
}
