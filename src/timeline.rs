//! The timeline / Gantt-style visualization.
//!
//! Each housing is a horizontal row; each [`Stay`] is a colored bar spanning
//! its date range. A left column lists housing names (with an over-capacity
//! warning), and a header row labels the days.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use egui::{Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::model::{Housing, Id, Plan, Stay, Subject};

/// Selects what a [`show`] call renders: which housings become rows and which
/// stays are drawn.
pub struct Filter<'a> {
    /// Housing ids to render as rows, in order.
    pub housings: &'a [Id],
    /// Predicate choosing which stays to draw.
    pub include: &'a dyn Fn(&Stay) -> bool,
    /// Centered text shown when there are no rows to draw.
    pub empty_hint: &'a str,
}

/// Width of the left column listing housing names. Public so the app can map
/// pointer positions to dates when zooming.
pub const LABEL_WIDTH: f32 = 160.0;
const HEADER_HEIGHT: f32 = 28.0;
const ROW_HEIGHT: f32 = 34.0;
const BAR_PADDING: f32 = 5.0;
/// Vertical gap between stacked sub-lanes within a housing row.
const LANE_GAP: f32 = 1.5;
/// Marker color for a subject booked in two places at once.
const SUBJECT_CONFLICT: Color32 = Color32::from_rgb(255, 145, 0);

/// Render the timeline into `ui` and return the canvas [`egui::Response`] so the
/// caller can implement drag-to-pan. The caller is responsible for wrapping this
/// in a [`egui::ScrollArea`] when there are more housings than fit vertically.
pub fn show(
    ui: &mut egui::Ui,
    plan: &Plan,
    view_start: NaiveDate,
    days_visible: i64,
    day_width: f32,
    filter: &Filter,
) -> egui::Response {
    // Resolve the requested housing ids to rows, skipping any that vanished.
    let row_housings: Vec<&Housing> =
        filter.housings.iter().filter_map(|id| plan.housing(*id)).collect();
    let rows = row_housings.len().max(1);
    let plot_width = days_visible as f32 * day_width;
    let total_size = Vec2::new(
        LABEL_WIDTH + plot_width,
        HEADER_HEIGHT + rows as f32 * ROW_HEIGHT,
    );

    // `click_and_drag` so the canvas consumes horizontal drags for panning.
    let (response, painter) = ui.allocate_painter(total_size, Sense::click_and_drag());
    let origin = response.rect.min;
    let today = chrono::Local::now().date_naive();
    // Stays whose subject is booked in two places at once.
    let subject_conflicts = plan.subject_double_bookings();

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
        let col = Rect::from_min_max(
            Pos2::new(x, plot_top),
            Pos2::new(x + day_width, plot_bottom),
        );

        if matches!(date.weekday(), Weekday::Sat | Weekday::Sun) {
            painter.rect_filled(col, CornerRadius::ZERO, weekend_bg);
        }
        painter.line_segment(
            [Pos2::new(x, origin.y), Pos2::new(x, plot_bottom)],
            Stroke::new(1.0, grid_color),
        );

        // Label every day when zoomed in, otherwise thin it out to avoid clutter.
        let label_every = if day_width >= 36.0 {
            1
        } else if day_width >= 18.0 {
            2
        } else {
            7
        };
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
    for (i, housing) in row_housings.iter().enumerate() {
        let row_top = plot_top + i as f32 * ROW_HEIGHT;
        let row_bottom = row_top + ROW_HEIGHT;

        // Horizontal separator.
        painter.line_segment(
            [
                Pos2::new(origin.x, row_bottom),
                Pos2::new(plot_right, row_bottom),
            ],
            Stroke::new(1.0, grid_color),
        );

        // Find the date spans where occupancy exceeds capacity (double booking),
        // merging consecutive over-capacity days into runs.
        let mut conflict_spans: Vec<(NaiveDate, NaiveDate)> = Vec::new();
        let mut run_start: Option<NaiveDate> = None;
        for d in 0..=days_visible {
            let date = view_start + Duration::days(d);
            let over = d < days_visible && plan.occupancy(housing.id, date) > housing.capacity;
            match (over, run_start) {
                (true, None) => run_start = Some(date),
                (false, Some(start)) => {
                    conflict_spans.push((start, date));
                    run_start = None;
                }
                _ => {}
            }
        }
        let over_capacity = !conflict_spans.is_empty();

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

        // Bars for every stay in this housing. Overlapping stays are packed
        // into stacked sub-lanes so no bar is ever hidden behind another.
        let mut stays: Vec<&Stay> = plan
            .stays
            .iter()
            .filter(|s| s.housing == housing.id && (filter.include)(s))
            .collect();
        stays.sort_by(|a, b| {
            a.arrival
                .cmp(&b.arrival)
                .then(a.departure.cmp(&b.departure))
        });

        let intervals: Vec<(NaiveDate, NaiveDate)> =
            stays.iter().map(|s| (s.arrival, s.departure)).collect();
        let (lane_of, lane_count) = pack_lanes(&intervals);

        let band_top = row_top + BAR_PADDING;
        let band_h = ROW_HEIGHT - 2.0 * BAR_PADDING;
        let lane_h = band_h / lane_count as f32;
        let view_end = view_start + Duration::days(days_visible);

        for (idx, stay) in stays.iter().enumerate() {
            // Skip stays entirely outside the visible window.
            if stay.departure <= view_start || stay.arrival >= view_end {
                continue;
            }
            let lane = lane_of[idx];
            let x0 = date_x(stay.arrival).max(plot_left);
            let x1 = date_x(stay.departure).min(plot_right).max(x0 + 2.0);
            let y0 = band_top + lane as f32 * lane_h;
            let y1 = y0 + (lane_h - LANE_GAP).max(2.0);
            let bar = Rect::from_min_max(Pos2::new(x0 + 1.0, y0), Pos2::new(x1 - 1.0, y1));

            let [r, g, b] = plan.subject_color(stay.subject);
            let fill = Color32::from_rgb(r, g, b);
            let corner = CornerRadius::same(if lane_h >= 12.0 { 4 } else { 2 });
            painter.rect_filled(bar, corner, fill);
            painter.rect_stroke(
                bar,
                corner,
                Stroke::new(1.0, fill.gamma_multiply(0.6)),
                StrokeKind::Inside,
            );

            // Only label when the lane is tall enough to read; otherwise the
            // color alone distinguishes the occupant.
            if lane_h >= 13.0 {
                let text_color = contrast_color(fill);
                painter.with_clip_rect(bar).text(
                    Pos2::new(bar.min.x + 4.0, bar.center().y),
                    Align2::LEFT_CENTER,
                    plan.subject_label(stay.subject),
                    bar_font.clone(),
                    text_color,
                );
            }

            // Subject double-booked elsewhere at the same time: amber border, and
            // a "!" badge at the right edge when there's room.
            if subject_conflicts.contains(&stay.id) {
                painter.rect_stroke(
                    bar,
                    corner,
                    Stroke::new(2.5, SUBJECT_CONFLICT),
                    StrokeKind::Inside,
                );
                if lane_h >= 12.0 {
                    let r = (lane_h * 0.5 - 1.5).clamp(4.0, 7.0);
                    let cx = (bar.max.x - r - 2.0).max(bar.min.x + r);
                    let center = Pos2::new(cx, bar.center().y);
                    let badge = painter.with_clip_rect(bar);
                    badge.circle_filled(center, r, SUBJECT_CONFLICT);
                    badge.text(
                        center,
                        Align2::CENTER_CENTER,
                        "!",
                        FontId::proportional(r * 1.5),
                        Color32::WHITE,
                    );
                }
            }

            // Hover tooltip: who / from / to. A hover-only region (it doesn't
            // sense drags, so canvas panning still works underneath).
            let conflicted = subject_conflicts.contains(&stay.id);
            ui.interact(bar, response.id.with(stay.id), Sense::hover())
                .on_hover_ui(|ui| stay_tooltip(ui, plan, housing, stay, conflicted));
        }

        // Double-booking overlay: a diagonal red hatch over every over-capacity
        // span. Translucent so the underlying bars still show, but unmistakable.
        for (start, end) in &conflict_spans {
            let cx0 = date_x(*start).max(plot_left);
            let cx1 = date_x(*end).min(plot_right);
            if cx1 <= cx0 {
                continue;
            }
            let span = Rect::from_min_max(
                Pos2::new(cx0, row_top + 1.0),
                Pos2::new(cx1, row_bottom - 1.0),
            );
            draw_conflict_hatch(&painter, span);
        }
    }

    // Outer border around the plot area.
    painter.rect_stroke(
        Rect::from_min_size(origin, total_size),
        CornerRadius::ZERO,
        Stroke::new(1.0, grid_color),
        StrokeKind::Inside,
    );

    if row_housings.is_empty() {
        painter.text(
            response.rect.center(),
            Align2::CENTER_CENTER,
            filter.empty_hint,
            label_font,
            visuals.weak_text_color(),
        );
    }

    response
}

/// Tooltip contents for a stay bar: who, where, and the from/to dates.
fn stay_tooltip(ui: &mut egui::Ui, plan: &Plan, housing: &Housing, stay: &Stay, conflicted: bool) {
    ui.strong(plan.subject_label(stay.subject));
    ui.label(format!("🏠 {}", housing.name));
    ui.separator();
    ui.label(format!("From:   {}", stay.arrival.format("%a %d %b %Y")));
    ui.label(format!("To:     {}", stay.departure.format("%a %d %b %Y")));
    let nights = (stay.departure - stay.arrival).num_days().max(0);
    ui.label(format!("Nights: {nights}"));
    if let Subject::Group(_) = stay.subject {
        ui.label(format!("People: {}", plan.subject_headcount(stay.subject)));
    }
    if conflicted {
        ui.colored_label(
            SUBJECT_CONFLICT,
            "⚠ Also booked elsewhere at the same time",
        );
    }
}

/// Greedily pack time intervals into stacked lanes so overlapping ones never
/// share a lane. `intervals` must be sorted by arrival. Returns the lane index
/// chosen for each interval and the total number of lanes used (>= 1).
///
/// A lane is free for a new interval once its previous occupant's departure is
/// at or before the new arrival (departure is exclusive — checkout day).
fn pack_lanes(intervals: &[(NaiveDate, NaiveDate)]) -> (Vec<usize>, usize) {
    let mut lane_free_from: Vec<NaiveDate> = Vec::new();
    let mut lane_of: Vec<usize> = Vec::with_capacity(intervals.len());
    for &(arrival, departure) in intervals {
        let lane = match lane_free_from.iter().position(|&end| end <= arrival) {
            Some(l) => {
                lane_free_from[l] = departure;
                l
            }
            None => {
                lane_free_from.push(departure);
                lane_free_from.len() - 1
            }
        };
        lane_of.push(lane);
    }
    (lane_of, lane_free_from.len().max(1))
}

/// Overlay a translucent diagonal red hatch (plus border) marking a date span
/// where occupancy exceeds capacity — a clearly distinguishable double-booking
/// indicator that doesn't hide the underlying bars.
fn draw_conflict_hatch(painter: &egui::Painter, span: Rect) {
    let line = Color32::from_rgba_unmultiplied(220, 40, 40, 150);
    let p = painter.with_clip_rect(span);
    let h = span.height();
    let step = 7.0;
    // Diagonal lines from bottom-left to top-right, offset to fill the rect.
    let mut x = span.min.x - h;
    while x < span.max.x {
        p.line_segment(
            [Pos2::new(x, span.max.y), Pos2::new(x + h, span.min.y)],
            Stroke::new(1.5, line),
        );
        x += step;
    }
    painter.rect_stroke(
        span,
        CornerRadius::ZERO,
        Stroke::new(1.5, Color32::from_rgb(200, 30, 30)),
        StrokeKind::Inside,
    );
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

#[cfg(test)]
mod tests {
    use super::pack_lanes;
    use chrono::NaiveDate;

    fn d(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 1, day).unwrap()
    }

    #[test]
    fn non_overlapping_share_one_lane() {
        // Back-to-back (checkout day == next arrival) may reuse the same lane.
        let intervals = [(d(1), d(3)), (d(3), d(5)), (d(5), d(7))];
        let (lanes, count) = pack_lanes(&intervals);
        assert_eq!(count, 1);
        assert_eq!(lanes, vec![0, 0, 0]);
    }

    #[test]
    fn overlapping_get_separate_lanes() {
        // Three intervals all covering day 4 -> three distinct lanes.
        let intervals = [(d(1), d(5)), (d(2), d(6)), (d(3), d(7))];
        let (lanes, count) = pack_lanes(&intervals);
        assert_eq!(count, 3);
        assert_eq!(lanes, vec![0, 1, 2]);
    }

    #[test]
    fn freed_lane_is_reused() {
        // Second interval overlaps the first (needs lane 1), third starts after
        // the first ends and should reuse lane 0.
        let intervals = [(d(1), d(4)), (d(2), d(6)), (d(4), d(8))];
        let (lanes, count) = pack_lanes(&intervals);
        assert_eq!(count, 2);
        assert_eq!(lanes, vec![0, 1, 0]);
    }
}
