//! Data model for the housing / stay planner.
//!
//! The core entities are [`Housing`], [`Group`], [`Person`] and [`Stay`].
//! A [`Stay`] assigns a *subject* (a single person or a whole group) to a
//! housing for an arrival..departure date range. Everything lives inside a
//! single [`Plan`] which is what gets persisted.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Stable identifier handed out by [`Plan::new_id`].
pub type Id = u64;

/// A place people can stay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Housing {
    pub id: Id,
    pub name: String,
    /// How many people fit. Used for over-capacity warnings.
    pub capacity: u32,
    pub notes: String,
}

/// A named group of people (e.g. a family or a team). Drawn in its own color.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Id,
    pub name: String,
    /// RGB color used for this group's bars in the timeline.
    pub color: [u8; 3],
}

/// A single person, optionally belonging to a [`Group`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Id,
    pub name: String,
    pub group: Option<Id>,
}

/// Who a [`Stay`] is for: a single person or an entire group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Subject {
    Person(Id),
    Group(Id),
}

/// One occupancy: a subject staying in a housing from `arrival` to `departure`.
///
/// The range is treated as half-open in days: `arrival` is the first night and
/// `departure` is the checkout day (so a same-day arrival/departure is 0 nights).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stay {
    pub id: Id,
    pub subject: Subject,
    pub housing: Id,
    pub arrival: NaiveDate,
    pub departure: NaiveDate,
}

/// The whole planning document. This is the unit of persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub housings: Vec<Housing>,
    pub groups: Vec<Group>,
    pub persons: Vec<Person>,
    pub stays: Vec<Stay>,
    next_id: Id,
}

impl Default for Plan {
    fn default() -> Self {
        Plan {
            housings: Vec::new(),
            groups: Vec::new(),
            persons: Vec::new(),
            stays: Vec::new(),
            next_id: 1,
        }
    }
}

impl Plan {
    /// Allocate a fresh unique id.
    pub fn new_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn housing(&self, id: Id) -> Option<&Housing> {
        self.housings.iter().find(|h| h.id == id)
    }

    pub fn group(&self, id: Id) -> Option<&Group> {
        self.groups.iter().find(|g| g.id == id)
    }

    pub fn person(&self, id: Id) -> Option<&Person> {
        self.persons.iter().find(|p| p.id == id)
    }

    /// Human-readable label for a stay's subject.
    pub fn subject_label(&self, subject: Subject) -> String {
        match subject {
            Subject::Person(id) => self
                .person(id)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "<deleted person>".to_owned()),
            Subject::Group(id) => self
                .group(id)
                .map(|g| format!("{} (group)", g.name))
                .unwrap_or_else(|| "<deleted group>".to_owned()),
        }
    }

    /// How many people a subject represents (a group counts its members).
    pub fn subject_headcount(&self, subject: Subject) -> u32 {
        match subject {
            Subject::Person(_) => 1,
            Subject::Group(id) => {
                self.persons.iter().filter(|p| p.group == Some(id)).count() as u32
            }
        }
    }

    /// The color a stay should be drawn with, derived from its subject's group.
    pub fn subject_color(&self, subject: Subject) -> [u8; 3] {
        let group_id = match subject {
            Subject::Group(id) => Some(id),
            Subject::Person(id) => self.person(id).and_then(|p| p.group),
        };
        group_id
            .and_then(|id| self.group(id))
            .map(|g| g.color)
            .unwrap_or(DEFAULT_BAR_COLOR)
    }

    /// True if `stay` covers `date` (arrival inclusive, departure exclusive).
    pub fn stay_covers(stay: &Stay, date: NaiveDate) -> bool {
        date >= stay.arrival && date < stay.departure
    }

    /// Total headcount occupying a housing on a given date.
    pub fn occupancy(&self, housing: Id, date: NaiveDate) -> u32 {
        self.stays
            .iter()
            .filter(|s| s.housing == housing && Self::stay_covers(s, date))
            .map(|s| self.subject_headcount(s.subject))
            .sum()
    }

    /// Earliest arrival across all stays, if any.
    pub fn earliest_arrival(&self) -> Option<NaiveDate> {
        self.stays.iter().map(|s| s.arrival).min()
    }

    /// Ensure the id counter is past every id currently in use. Call this after
    /// loading a plan from an external file so freshly created entities can't
    /// collide with loaded ones.
    pub fn reseed_ids(&mut self) {
        let max = self
            .housings
            .iter()
            .map(|h| h.id)
            .chain(self.groups.iter().map(|g| g.id))
            .chain(self.persons.iter().map(|p| p.id))
            .chain(self.stays.iter().map(|s| s.id))
            .max()
            .unwrap_or(0);
        if self.next_id <= max {
            self.next_id = max + 1;
        }
    }

    /// True if nothing has been entered yet.
    pub fn is_empty(&self) -> bool {
        self.housings.is_empty() && self.persons.is_empty() && self.groups.is_empty()
    }

    /// Populate a small illustrative plan around today's date.
    pub fn load_sample(&mut self) {
        let today = chrono::Local::now().date_naive();
        let d = |offset: i64| today + chrono::Duration::days(offset);

        let cabin = self.new_id();
        let lodge = self.new_id();
        self.housings.push(Housing { id: cabin, name: "Cabin A".into(), capacity: 2, notes: String::new() });
        self.housings.push(Housing { id: lodge, name: "Lodge".into(), capacity: 6, notes: String::new() });

        let smiths = self.new_id();
        let crew = self.new_id();
        self.groups.push(Group { id: smiths, name: "Smith family".into(), color: GROUP_PALETTE[0] });
        self.groups.push(Group { id: crew, name: "Crew".into(), color: GROUP_PALETTE[2] });

        let alice = self.new_id();
        let bob = self.new_id();
        let dana = self.new_id();
        self.persons.push(Person { id: alice, name: "Alice Smith".into(), group: Some(smiths) });
        self.persons.push(Person { id: bob, name: "Bob Smith".into(), group: Some(smiths) });
        self.persons.push(Person { id: dana, name: "Dana".into(), group: Some(crew) });

        let s1 = self.new_id();
        let s2 = self.new_id();
        let s3 = self.new_id();
        let s4 = self.new_id();
        self.stays.push(Stay { id: s1, subject: Subject::Group(smiths), housing: cabin, arrival: d(0), departure: d(5) });
        self.stays.push(Stay { id: s2, subject: Subject::Person(dana), housing: lodge, arrival: d(2), departure: d(9) });
        self.stays.push(Stay { id: s3, subject: Subject::Group(crew), housing: lodge, arrival: d(6), departure: d(12) });
        // Overlaps the Smiths in Cabin A (capacity 2) on days 2–4 → double booking.
        self.stays.push(Stay { id: s4, subject: Subject::Person(dana), housing: cabin, arrival: d(2), departure: d(4) });
    }
}

/// Fallback bar color for subjects without a group.
pub const DEFAULT_BAR_COLOR: [u8; 3] = [110, 140, 200];

/// A small palette cycled through when creating new groups.
pub const GROUP_PALETTE: [[u8; 3]; 8] = [
    [231, 76, 60],   // red
    [46, 204, 113],  // green
    [52, 152, 219],  // blue
    [241, 196, 15],  // yellow
    [155, 89, 182],  // purple
    [230, 126, 34],  // orange
    [26, 188, 156],  // teal
    [233, 30, 99],   // pink
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip_preserves_plan() {
        let mut plan = Plan::default();
        plan.load_sample();

        let json = serde_json::to_string_pretty(&plan).expect("serialize");
        let back: Plan = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(plan.housings.len(), back.housings.len());
        assert_eq!(plan.persons.len(), back.persons.len());
        assert_eq!(plan.groups.len(), back.groups.len());
        assert_eq!(plan.stays.len(), back.stays.len());
        // Spot-check a value survives the trip.
        assert_eq!(plan.housings[0].name, back.housings[0].name);
        assert_eq!(plan.stays[0].arrival, back.stays[0].arrival);
    }

    #[test]
    fn sample_has_an_over_capacity_day() {
        let mut plan = Plan::default();
        plan.load_sample();

        let cabin = plan
            .housings
            .iter()
            .find(|h| h.name == "Cabin A")
            .expect("cabin exists");
        let today = chrono::Local::now().date_naive();
        // Smiths (2) overlap Dana (1) on day +2..+4 in a capacity-2 housing.
        let day = today + chrono::Duration::days(2);
        assert!(plan.occupancy(cabin.id, day) > cabin.capacity);
    }

    #[test]
    fn reseed_ids_avoids_collisions() {
        // Simulate a file whose ids are high but whose next_id is stale.
        let json = r#"{
            "housings": [{"id": 50, "name": "H", "capacity": 2, "notes": ""}],
            "groups": [], "persons": [], "stays": [], "next_id": 1
        }"#;
        let mut plan: Plan = serde_json::from_str(json).expect("deserialize");
        plan.reseed_ids();
        let fresh = plan.new_id();
        assert!(fresh > 50, "new id {fresh} must not collide with loaded ids");
    }
}
