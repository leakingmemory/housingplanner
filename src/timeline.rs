//! The timeline / Gantt-style visualization.
//!
//! Each housing is a horizontal row; each [`Stay`] is a colored bar spanning
//! its date range. A left column lists housing names (with an over-capacity
//! warning), and a header row labels the days.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use egui::{Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::model::{Plan, Stay};

/// Width of the left column listing housing names. Public so the app can map
/// pointer positions to dates when zooming.
pub const LABEL_WIDTH: f32 = 160.0;
const HEADER_HEIGHT: f32 = 28.0;
const ROW_HEIGHT: f32 = 34.0;
const BAR_PADDING: f32 = 5.0;

/// Render the timeline into `ui` and return the canvas [`egui::Response`] so the
/// caller can implement drag-to-pan. The caller is responsible for wrapping this
/// in a [`egui::ScrollArea`] when there are more housings than fit vertically.
pub fn show(
    ui: &mut egui::Ui,
    plan: &Plan,
    view_start: NaiveDate,
    days_visible: i64,
    day_width: f32,
) -> egui::Response {
    let rows = plan.housings.len().max(1);
    let plot_width = days_visible as f32 * day_width;
    let total_size = Vec2::new(
        LABEL_WIDTH + plot_width,
        HEADER_HEIGHT + rows as f32 * ROW_HEIGHT,
    );

    // `click_and_drag` so the canvas consumes horizontal drags for panning.
    let (response, painter) = ui.allocate_painter(total_size, Sense::click_and_drag());
    let origin = response.rect.min;
    let today = chrono::Local::now().date_naive();

    // Maps a calendar date to an x coordinate in screen space.
    let date_x = |date: NaiveDate| -> f32 {
        origin.x + LABEL_WIDTH + (date - view_start).num_days() as f32 * day_width
    };

    let plot_left = origin.x + LABEL_WIDTH;
    let plot_right = origin.x + total_size.x;
    let plot_top = origin.y + HEADER_HEIGHT;
    let plot_bottom = origin.y + total_size.y;

    let visuals = ui.visuals();
    let grid_color = visuals.weak_text_color().gamma_multiply(0.4);
    let weekend_bg = if visuals.dark_mode {
        Color32::from_rgba_unmultiplied(255, 255, 255, 8)
    } else {
        Color32::from_rgba_unmultiplied(0, 0, 0, 10)
    };
    let header_font = FontId::proportional(12.0);
    let bar_font = FontId::proportional(13.0);
    let label_font = FontId::proportional(14.0);

    // --- Day columns: weekend shading, grid lines and date labels. ---
    for d in 0..days_visible {
        let date = view_start + Duration::days(d);
        let x = origin.x + LABEL_WIDTH + d as f32 * day_width;
        let col = Rect::from_min_max(Pos2::new(x, plot_top), Pos2::new(x + day_width, plot_bottom));

        if matches!(date.weekday(), Weekday::Sat | Weekday::Sun) {
            painter.rect_filled(col, CornerRadius::ZERO, weekend_bg);
        }
        painter.line_segment(
            [Pos2::new(x, origin.y), Pos2::new(x, plot_bottom)],
            Stroke::new(1.0, grid_color),
        );

        // Label every day when zoomed in, otherwise thin it out to avoid clutter.
        let label_every = if day_width >= 36.0 { 1 } else if day_width >= 18.0 { 2 } else { 7 };
        if d % label_every == 0 {
            painter.text(
                Pos2::new(x + 3.0, origin.y + HEADER_HEIGHT / 2.0),
                Align2::LEFT_CENTER,
                date.format("%d %b").to_string(),
                header_font.clone(),
                visuals.text_color(),
            );
        }
    }

    // --- "Today" marker. ---
    if today >= view_start && today < view_start + Duration::days(days_visible) {
        let x = date_x(today);
        painter.line_segment(
            [Pos2::new(x, origin.y), Pos2::new(x, plot_bottom)],
            Stroke::new(2.0, Color32::from_rgb(220, 60, 60)),
        );
    }

    // --- Housing rows. ---
    for (i, housing) in plan.housings.iter().enumerate() {
        let row_top = plot_top + i as f32 * ROW_HEIGHT;
        let row_bottom = row_top + ROW_HEIGHT;

        // Horizontal separator.
        painter.line_segment(
            [Pos2::new(origin.x, row_bottom), Pos2::new(plot_right, row_bottom)],
            Stroke::new(1.0, grid_color),
        );

        // Over-capacity check across the visible range.
        let mut over_capacity = false;
        for d in 0..days_visible {
            let date = view_start + Duration::days(d);
            if plan.occupancy(housing.id, date) > housing.capacity {
                over_capacity = true;
                break;
            }
        }

        // Left label column.
        let name_color = if over_capacity {
            Color32::from_rgb(220, 60, 60)
        } else {
            visuals.text_color()
        };
        painter.text(
            Pos2::new(origin.x + 6.0, row_top + ROW_HEIGHT / 2.0),
            Align2::LEFT_CENTER,
            &housing.name,
            label_font.clone(),
            name_color,
        );
        let cap_text = if over_capacity {
            format!("cap {} ⚠", housing.capacity)
        } else {
            format!("cap {}", housing.capacity)
        };
        painter.text(
            Pos2::new(origin.x + 6.0, row_top + ROW_HEIGHT - 8.0),
            Align2::LEFT_CENTER,
            cap_text,
            FontId::proportional(10.0),
            visuals.weak_text_color(),
        );

        // Bars for every stay in this housing.
        let mut stays: Vec<&Stay> = plan.stays.iter().filter(|s| s.housing == housing.id).collect();
        stays.sort_by_key(|s| s.arrival);
        for stay in stays {
            let view_end = view_start + Duration::days(days_visible);
            // Skip stays entirely outside the visible window.
            if stay.departure <= view_start || stay.arrival >= view_end {
                continue;
            }
            let x0 = date_x(stay.arrival).max(plot_left);
            let x1 = date_x(stay.departure).min(plot_right).max(x0 + 2.0);
            let bar = Rect::from_min_max(
                Pos2::new(x0 + 1.0, row_top + BAR_PADDING),
                Pos2::new(x1 - 1.0, row_bottom - BAR_PADDING),
            );

            let [r, g, b] = plan.subject_color(stay.subject);
            let fill = Color32::from_rgb(r, g, b);
            painter.rect_filled(bar, CornerRadius::same(4), fill);
            painter.rect_stroke(
                bar,
                CornerRadius::same(4),
                Stroke::new(1.0, fill.gamma_multiply(0.6)),
                StrokeKind::Inside,
            );

            // Label, clipped to the bar so it never overflows into neighbours.
            let text_color = contrast_color(fill);
            painter
                .with_clip_rect(bar)
                .text(
                    Pos2::new(bar.min.x + 4.0, bar.center().y),
                    Align2::LEFT_CENTER,
                    plan.subject_label(stay.subject),
                    bar_font.clone(),
                    text_color,
                );
        }
    }

    // Outer border around the plot area.
    painter.rect_stroke(
        Rect::from_min_size(origin, total_size),
        CornerRadius::ZERO,
        Stroke::new(1.0, grid_color),
        StrokeKind::Inside,
    );

    if plan.housings.is_empty() {
        painter.text(
            response.rect.center(),
            Align2::CENTER_CENTER,
            "Add a housing in the side panel to start planning.",
            label_font,
            visuals.weak_text_color(),
        );
    }

    response
}

/// Pick black or white text depending on background luminance.
fn contrast_color(bg: Color32) -> Color32 {
    let luma = 0.299 * bg.r() as f32 + 0.587 * bg.g() as f32 + 0.114 * bg.b() as f32;
    if luma > 140.0 {
        Color32::from_rgb(20, 20, 20)
    } else {
        Color32::from_rgb(245, 245, 245)
    }
}
