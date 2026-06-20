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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Housing {
    pub id: Id,
    pub name: String,
    /// How many people fit. Used for over-capacity warnings.
    pub capacity: u32,
    pub notes: String,
}

/// A named group of people (e.g. a family or a team). Drawn in its own color.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: Id,
    pub name: String,
    /// RGB color used for this group's bars in the timeline.
    pub color: [u8; 3],
}

/// A single person, optionally belonging to a [`Group`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stay {
    pub id: Id,
    pub subject: Subject,
    pub housing: Id,
    pub arrival: NaiveDate,
    pub departure: NaiveDate,
}

/// A single entry in the persisted change journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Stable change id (see [`Plan::new_log_id`]). An [`LogKind::Undo`] entry
    /// references the id of the change it reverted.
    pub id: u64,
    /// When the change happened, RFC 3339 (local time).
    pub time: String,
    pub kind: LogKind,
}

/// The structured content of a [`LogEntry`]. Carries the names/values needed to
/// render a (localized) human description even after the entity is gone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogKind {
    AddHousing {
        name: String,
    },
    DeleteHousing {
        name: String,
    },
    RenameHousing {
        from: String,
        to: String,
    },
    SetCapacity {
        name: String,
        from: u32,
        to: u32,
    },
    EditNotes {
        name: String,
    },

    AddGroup {
        name: String,
    },
    DeleteGroup {
        name: String,
    },
    RenameGroup {
        from: String,
        to: String,
    },
    SetGroupColor {
        name: String,
    },

    AddPerson {
        name: String,
    },
    DeletePerson {
        name: String,
    },
    RenamePerson {
        from: String,
        to: String,
    },
    SetPersonGroup {
        person: String,
        from: String,
        to: String,
    },

    AddStay {
        subject: String,
        housing: String,
    },
    RemoveStay {
        subject: String,
        housing: String,
    },
    MoveStay {
        subject: String,
        from: String,
        to: String,
    },
    ChangeStaySubject {
        from: String,
        to: String,
        housing: String,
    },
    ChangeStayDates {
        subject: String,
        housing: String,
        from: String,
        to: String,
    },

    LoadedExample,
    LoadedFile {
        name: String,
    },
    LoadedFileNoHistory {
        name: String,
    },
    Undo {
        undoes: u64,
    },
}

fn default_next_log_id() -> u64 {
    1
}

/// The whole planning document. This is the unit of persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub housings: Vec<Housing>,
    pub groups: Vec<Group>,
    pub persons: Vec<Person>,
    pub stays: Vec<Stay>,
    next_id: Id,
    /// Append-only change journal. `serde(default)` keeps pre-journal files
    /// loadable (empty log); old apps ignore this unknown field.
    #[serde(default)]
    pub changelog: Vec<LogEntry>,
    #[serde(default = "default_next_log_id")]
    next_log_id: u64,
}

impl Default for Plan {
    fn default() -> Self {
        Plan {
            housings: Vec::new(),
            groups: Vec::new(),
            persons: Vec::new(),
            stays: Vec::new(),
            next_id: 1,
            changelog: Vec::new(),
            next_log_id: 1,
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

    /// Allocate a fresh change-journal id.
    pub fn new_log_id(&mut self) -> u64 {
        let id = self.next_log_id;
        self.next_log_id += 1;
        id
    }

    /// Append a journal entry for `kind` (stamping id + local time); returns its id.
    pub fn push_log(&mut self, kind: LogKind) -> u64 {
        let id = self.new_log_id();
        self.changelog.push(LogEntry {
            id,
            time: chrono::Local::now().to_rfc3339(),
            kind,
        });
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

    /// Human-readable label for a stay's subject, localized for `lang`.
    pub fn subject_label(&self, subject: Subject, lang: crate::i18n::Lang) -> String {
        use crate::i18n::tr;
        match subject {
            Subject::Person(id) => self
                .person(id)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| tr(lang, "<deleted person>").to_owned()),
            Subject::Group(id) => self
                .group(id)
                .map(|g| format!("{} {}", g.name, tr(lang, "(group)")))
                .unwrap_or_else(|| tr(lang, "<deleted group>").to_owned()),
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

    /// The set of persons a stay's subject puts in a housing (a group expands to
    /// its current members).
    fn stay_persons(&self, subject: Subject) -> Vec<Id> {
        match subject {
            Subject::Person(id) => vec![id],
            Subject::Group(gid) => self
                .persons
                .iter()
                .filter(|p| p.group == Some(gid))
                .map(|p| p.id)
                .collect(),
        }
    }

    /// True if `stay` puts `person` in a housing — directly, or via a group the
    /// person belongs to.
    pub fn stay_includes_person(&self, stay: &Stay, person: Id) -> bool {
        match stay.subject {
            Subject::Person(id) => id == person,
            Subject::Group(gid) => self.person(person).map_or(false, |p| p.group == Some(gid)),
        }
    }

    /// The persons that belong to a group, in declaration order.
    pub fn persons_in_group(&self, gid: Id) -> Vec<&Person> {
        self.persons
            .iter()
            .filter(|p| p.group == Some(gid))
            .collect()
    }

    /// Ids of stays that put the same person in more than one housing at the
    /// same time. A person can't be in two places at once, so both stays of each
    /// clashing pair are returned. Groups are expanded to members, so an
    /// individual stay overlapping that person's group elsewhere is caught too.
    pub fn subject_double_bookings(&self) -> std::collections::HashSet<Id> {
        let persons: Vec<Vec<Id>> = self
            .stays
            .iter()
            .map(|s| self.stay_persons(s.subject))
            .collect();

        let mut conflicted = std::collections::HashSet::new();
        for i in 0..self.stays.len() {
            for j in (i + 1)..self.stays.len() {
                let a = &self.stays[i];
                let b = &self.stays[j];
                // Only a problem if it's a *different* location and the dates
                // actually overlap (departure is exclusive).
                if a.housing == b.housing {
                    continue;
                }
                if !(a.arrival < b.departure && b.arrival < a.departure) {
                    continue;
                }
                if persons[i].iter().any(|p| persons[j].contains(p)) {
                    conflicted.insert(a.id);
                    conflicted.insert(b.id);
                }
            }
        }
        conflicted
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

        let max_log = self.changelog.iter().map(|e| e.id).max().unwrap_or(0);
        if self.next_log_id <= max_log {
            self.next_log_id = max_log + 1;
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
        self.housings.push(Housing {
            id: cabin,
            name: "Cabin A".into(),
            capacity: 2,
            notes: String::new(),
        });
        self.housings.push(Housing {
            id: lodge,
            name: "Lodge".into(),
            capacity: 6,
            notes: String::new(),
        });

        let smiths = self.new_id();
        let crew = self.new_id();
        self.groups.push(Group {
            id: smiths,
            name: "Smith family".into(),
            color: GROUP_PALETTE[0],
        });
        self.groups.push(Group {
            id: crew,
            name: "Crew".into(),
            color: GROUP_PALETTE[2],
        });

        let alice = self.new_id();
        let bob = self.new_id();
        let dana = self.new_id();
        self.persons.push(Person {
            id: alice,
            name: "Alice Smith".into(),
            group: Some(smiths),
        });
        self.persons.push(Person {
            id: bob,
            name: "Bob Smith".into(),
            group: Some(smiths),
        });
        self.persons.push(Person {
            id: dana,
            name: "Dana".into(),
            group: Some(crew),
        });

        let s1 = self.new_id();
        let s2 = self.new_id();
        let s3 = self.new_id();
        let s4 = self.new_id();
        self.stays.push(Stay {
            id: s1,
            subject: Subject::Group(smiths),
            housing: cabin,
            arrival: d(0),
            departure: d(5),
        });
        self.stays.push(Stay {
            id: s2,
            subject: Subject::Person(dana),
            housing: lodge,
            arrival: d(2),
            departure: d(9),
        });
        self.stays.push(Stay {
            id: s3,
            subject: Subject::Group(crew),
            housing: lodge,
            arrival: d(6),
            departure: d(12),
        });
        // Overlaps the Smiths in Cabin A (capacity 2) on days 2–4 → double booking.
        self.stays.push(Stay {
            id: s4,
            subject: Subject::Person(dana),
            housing: cabin,
            arrival: d(2),
            departure: d(4),
        });
    }
}

/// Fallback bar color for subjects without a group.
pub const DEFAULT_BAR_COLOR: [u8; 3] = [110, 140, 200];

/// A small palette cycled through when creating new groups.
pub const GROUP_PALETTE: [[u8; 3]; 8] = [
    [231, 76, 60],  // red
    [46, 204, 113], // green
    [52, 152, 219], // blue
    [241, 196, 15], // yellow
    [155, 89, 182], // purple
    [230, 126, 34], // orange
    [26, 188, 156], // teal
    [233, 30, 99],  // pink
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
    fn detects_person_in_two_places() {
        let d = |n| chrono::NaiveDate::from_ymd_opt(2026, 1, n).unwrap();
        let mut plan = Plan::default();
        let (h1, h2) = (plan.new_id(), plan.new_id());
        plan.housings.push(Housing {
            id: h1,
            name: "A".into(),
            capacity: 9,
            notes: String::new(),
        });
        plan.housings.push(Housing {
            id: h2,
            name: "B".into(),
            capacity: 9,
            notes: String::new(),
        });
        let p = plan.new_id();
        plan.persons.push(Person {
            id: p,
            name: "P".into(),
            group: None,
        });

        let (s1, s2, s3) = (plan.new_id(), plan.new_id(), plan.new_id());
        // s1 & s2 overlap in different housings -> clash. s3 is later, no clash.
        plan.stays.push(Stay {
            id: s1,
            subject: Subject::Person(p),
            housing: h1,
            arrival: d(1),
            departure: d(5),
        });
        plan.stays.push(Stay {
            id: s2,
            subject: Subject::Person(p),
            housing: h2,
            arrival: d(3),
            departure: d(7),
        });
        plan.stays.push(Stay {
            id: s3,
            subject: Subject::Person(p),
            housing: h2,
            arrival: d(10),
            departure: d(12),
        });

        let clash = plan.subject_double_bookings();
        assert!(clash.contains(&s1) && clash.contains(&s2));
        assert!(!clash.contains(&s3));
    }

    #[test]
    fn group_member_clashes_with_individual_booking() {
        let d = |n| chrono::NaiveDate::from_ymd_opt(2026, 1, n).unwrap();
        let mut plan = Plan::default();
        let (h1, h2) = (plan.new_id(), plan.new_id());
        plan.housings.push(Housing {
            id: h1,
            name: "A".into(),
            capacity: 9,
            notes: String::new(),
        });
        plan.housings.push(Housing {
            id: h2,
            name: "B".into(),
            capacity: 9,
            notes: String::new(),
        });
        let g = plan.new_id();
        plan.groups.push(Group {
            id: g,
            name: "G".into(),
            color: [1, 2, 3],
        });
        let p = plan.new_id();
        plan.persons.push(Person {
            id: p,
            name: "P".into(),
            group: Some(g),
        });

        let (gs, ps) = (plan.new_id(), plan.new_id());
        // Group booked at A; the member booked individually at B, overlapping.
        plan.stays.push(Stay {
            id: gs,
            subject: Subject::Group(g),
            housing: h1,
            arrival: d(1),
            departure: d(5),
        });
        plan.stays.push(Stay {
            id: ps,
            subject: Subject::Person(p),
            housing: h2,
            arrival: d(2),
            departure: d(4),
        });

        let clash = plan.subject_double_bookings();
        assert!(clash.contains(&gs) && clash.contains(&ps));
    }

    #[test]
    fn same_housing_overlap_is_not_a_double_location() {
        let d = |n| chrono::NaiveDate::from_ymd_opt(2026, 1, n).unwrap();
        let mut plan = Plan::default();
        let h = plan.new_id();
        plan.housings.push(Housing {
            id: h,
            name: "A".into(),
            capacity: 9,
            notes: String::new(),
        });
        let p = plan.new_id();
        plan.persons.push(Person {
            id: p,
            name: "P".into(),
            group: None,
        });

        let (s1, s2) = (plan.new_id(), plan.new_id());
        plan.stays.push(Stay {
            id: s1,
            subject: Subject::Person(p),
            housing: h,
            arrival: d(1),
            departure: d(5),
        });
        plan.stays.push(Stay {
            id: s2,
            subject: Subject::Person(p),
            housing: h,
            arrival: d(3),
            departure: d(7),
        });

        assert!(plan.subject_double_bookings().is_empty());
    }

    #[test]
    fn stay_includes_person_direct_and_via_group() {
        let d = |n| chrono::NaiveDate::from_ymd_opt(2026, 1, n).unwrap();
        let mut plan = Plan::default();
        let h = plan.new_id();
        plan.housings.push(Housing {
            id: h,
            name: "A".into(),
            capacity: 9,
            notes: String::new(),
        });
        let g = plan.new_id();
        plan.groups.push(Group {
            id: g,
            name: "G".into(),
            color: [1, 2, 3],
        });
        let member = plan.new_id();
        let outsider = plan.new_id();
        plan.persons.push(Person {
            id: member,
            name: "M".into(),
            group: Some(g),
        });
        plan.persons.push(Person {
            id: outsider,
            name: "O".into(),
            group: None,
        });

        let own = Stay {
            id: plan.new_id(),
            subject: Subject::Person(member),
            housing: h,
            arrival: d(1),
            departure: d(3),
        };
        let grp = Stay {
            id: plan.new_id(),
            subject: Subject::Group(g),
            housing: h,
            arrival: d(1),
            departure: d(3),
        };

        assert!(plan.stay_includes_person(&own, member));
        assert!(!plan.stay_includes_person(&own, outsider));
        assert!(plan.stay_includes_person(&grp, member)); // via group
        assert!(!plan.stay_includes_person(&grp, outsider));

        assert_eq!(plan.persons_in_group(g).len(), 1);
    }

    #[test]
    fn plan_changelog_round_trips() {
        let mut plan = Plan::default();
        plan.push_log(LogKind::LoadedExample);
        plan.push_log(LogKind::AddGroup {
            name: "Crew".into(),
        });
        let json = serde_json::to_string(&plan).expect("serialize");
        let back: Plan = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.changelog.len(), 2);
    }

    #[test]
    fn loads_legacy_plan_without_changelog() {
        // A plan file from before the journal feature: no changelog/next_log_id.
        let json = r#"{"housings":[],"groups":[],"persons":[],"stays":[],"next_id":1}"#;
        let plan: Plan = serde_json::from_str(json).expect("legacy plan loads");
        assert!(plan.changelog.is_empty());
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
        assert!(
            fresh > 50,
            "new id {fresh} must not collide with loaded ids"
        );
    }
}
