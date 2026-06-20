//! Change-journal engine: diffing two plan states into journal entries, the
//! in-memory inverse operations that back session undo, and localized rendering
//! of entries for the Changelog tab.

use std::collections::HashSet;

use crate::i18n::{tr, Lang};
use crate::model::{Group, Housing, Id, LogEntry, LogKind, Person, Plan, Stay, Subject};

/// One detected change: the persisted [`LogKind`] (for the journal + display)
/// plus the in-memory [`InverseOp`] that reverts it (session undo only).
pub struct Change {
    pub kind: LogKind,
    pub inverse: InverseOp,
}

/// How to revert a single change. Carries cloned previous values; not persisted.
pub enum InverseOp {
    InsertHousing {
        housing: Housing,
        stays: Vec<Stay>,
    },
    RemoveHousing {
        id: Id,
    },
    SetHousingName {
        id: Id,
        name: String,
    },
    SetHousingCapacity {
        id: Id,
        capacity: u32,
    },
    SetHousingNotes {
        id: Id,
        notes: String,
    },

    InsertGroup {
        group: Group,
        members: Vec<Id>,
        stays: Vec<Stay>,
    },
    RemoveGroup {
        id: Id,
    },
    SetGroupName {
        id: Id,
        name: String,
    },
    SetGroupColor {
        id: Id,
        color: [u8; 3],
    },

    InsertPerson {
        person: Person,
        stays: Vec<Stay>,
    },
    RemovePerson {
        id: Id,
    },
    SetPersonName {
        id: Id,
        name: String,
    },
    SetPersonGroup {
        id: Id,
        group: Option<Id>,
    },

    InsertStay {
        stay: Stay,
    },
    RemoveStay {
        id: Id,
    },
    SetStayHousing {
        id: Id,
        housing: Id,
    },
    SetStaySubject {
        id: Id,
        subject: Subject,
    },
    SetStayDates {
        id: Id,
        arrival: chrono::NaiveDate,
        departure: chrono::NaiveDate,
    },
}

/// Apply an inverse operation to `plan` (used by undo).
pub fn apply_inverse(plan: &mut Plan, op: &InverseOp) {
    match op {
        InverseOp::InsertHousing { housing, stays } => {
            plan.housings.push(housing.clone());
            plan.stays.extend(stays.iter().cloned());
        }
        InverseOp::RemoveHousing { id } => plan.housings.retain(|h| h.id != *id),
        InverseOp::SetHousingName { id, name } => {
            if let Some(h) = plan.housings.iter_mut().find(|h| h.id == *id) {
                h.name = name.clone();
            }
        }
        InverseOp::SetHousingCapacity { id, capacity } => {
            if let Some(h) = plan.housings.iter_mut().find(|h| h.id == *id) {
                h.capacity = *capacity;
            }
        }
        InverseOp::SetHousingNotes { id, notes } => {
            if let Some(h) = plan.housings.iter_mut().find(|h| h.id == *id) {
                h.notes = notes.clone();
            }
        }

        InverseOp::InsertGroup {
            group,
            members,
            stays,
        } => {
            plan.groups.push(group.clone());
            for m in members {
                if let Some(p) = plan.persons.iter_mut().find(|p| p.id == *m) {
                    p.group = Some(group.id);
                }
            }
            plan.stays.extend(stays.iter().cloned());
        }
        InverseOp::RemoveGroup { id } => plan.groups.retain(|g| g.id != *id),
        InverseOp::SetGroupName { id, name } => {
            if let Some(g) = plan.groups.iter_mut().find(|g| g.id == *id) {
                g.name = name.clone();
            }
        }
        InverseOp::SetGroupColor { id, color } => {
            if let Some(g) = plan.groups.iter_mut().find(|g| g.id == *id) {
                g.color = *color;
            }
        }

        InverseOp::InsertPerson { person, stays } => {
            plan.persons.push(person.clone());
            plan.stays.extend(stays.iter().cloned());
        }
        InverseOp::RemovePerson { id } => plan.persons.retain(|p| p.id != *id),
        InverseOp::SetPersonName { id, name } => {
            if let Some(p) = plan.persons.iter_mut().find(|p| p.id == *id) {
                p.name = name.clone();
            }
        }
        InverseOp::SetPersonGroup { id, group } => {
            if let Some(p) = plan.persons.iter_mut().find(|p| p.id == *id) {
                p.group = *group;
            }
        }

        InverseOp::InsertStay { stay } => plan.stays.push(stay.clone()),
        InverseOp::RemoveStay { id } => plan.stays.retain(|s| s.id != *id),
        InverseOp::SetStayHousing { id, housing } => {
            if let Some(s) = plan.stays.iter_mut().find(|s| s.id == *id) {
                s.housing = *housing;
            }
        }
        InverseOp::SetStaySubject { id, subject } => {
            if let Some(s) = plan.stays.iter_mut().find(|s| s.id == *id) {
                s.subject = *subject;
            }
        }
        InverseOp::SetStayDates {
            id,
            arrival,
            departure,
        } => {
            if let Some(s) = plan.stays.iter_mut().find(|s| s.id == *id) {
                s.arrival = *arrival;
                s.departure = *departure;
            }
        }
    }
}

// --- name resolution helpers (resolved against whichever plan still has the entity) ---

fn hname(plan: &Plan, id: Id) -> String {
    plan.housing(id)
        .map(|h| h.name.clone())
        .unwrap_or_else(|| "?".to_owned())
}

fn subj(plan: &Plan, s: Subject) -> String {
    match s {
        Subject::Person(id) => plan.person(id).map(|p| p.name.clone()),
        Subject::Group(id) => plan.group(id).map(|g| g.name.clone()),
    }
    .unwrap_or_else(|| "?".to_owned())
}

/// Group name, or "" for "no group" (rendered localized in [`describe`]).
fn group_label(plan: &Plan, g: Option<Id>) -> String {
    match g {
        None => String::new(),
        Some(id) => plan
            .group(id)
            .map(|x| x.name.clone())
            .unwrap_or_else(|| "?".to_owned()),
    }
}

/// Diff two plan states into the changes between them (added / deleted / edited
/// entities). Deletes fold their cascaded stays (and a group's detached members)
/// into the single Delete* change so nothing is logged twice.
pub fn diff(old: &Plan, new: &Plan) -> Vec<Change> {
    let mut out = Vec::new();

    // --- Housings ---
    let mut cascaded_stays: HashSet<Id> = HashSet::new();
    for h in &old.housings {
        if !new.housings.iter().any(|x| x.id == h.id) {
            let stays: Vec<Stay> = old
                .stays
                .iter()
                .filter(|s| s.housing == h.id)
                .cloned()
                .collect();
            cascaded_stays.extend(stays.iter().map(|s| s.id));
            out.push(Change {
                kind: LogKind::DeleteHousing {
                    name: h.name.clone(),
                },
                inverse: InverseOp::InsertHousing {
                    housing: h.clone(),
                    stays,
                },
            });
        }
    }
    for h in &new.housings {
        match old.housings.iter().find(|x| x.id == h.id) {
            None => out.push(Change {
                kind: LogKind::AddHousing {
                    name: h.name.clone(),
                },
                inverse: InverseOp::RemoveHousing { id: h.id },
            }),
            Some(o) => {
                if o.name != h.name {
                    out.push(Change {
                        kind: LogKind::RenameHousing {
                            from: o.name.clone(),
                            to: h.name.clone(),
                        },
                        inverse: InverseOp::SetHousingName {
                            id: h.id,
                            name: o.name.clone(),
                        },
                    });
                }
                if o.capacity != h.capacity {
                    out.push(Change {
                        kind: LogKind::SetCapacity {
                            name: h.name.clone(),
                            from: o.capacity,
                            to: h.capacity,
                        },
                        inverse: InverseOp::SetHousingCapacity {
                            id: h.id,
                            capacity: o.capacity,
                        },
                    });
                }
                if o.notes != h.notes {
                    out.push(Change {
                        kind: LogKind::EditNotes {
                            name: h.name.clone(),
                        },
                        inverse: InverseOp::SetHousingNotes {
                            id: h.id,
                            notes: o.notes.clone(),
                        },
                    });
                }
            }
        }
    }

    // --- Groups ---
    let mut detached_persons: HashSet<Id> = HashSet::new();
    for g in &old.groups {
        if !new.groups.iter().any(|x| x.id == g.id) {
            let members: Vec<Id> = old
                .persons
                .iter()
                .filter(|p| p.group == Some(g.id))
                .map(|p| p.id)
                .collect();
            detached_persons.extend(members.iter().copied());
            let stays: Vec<Stay> = old
                .stays
                .iter()
                .filter(|s| s.subject == Subject::Group(g.id))
                .cloned()
                .collect();
            cascaded_stays.extend(stays.iter().map(|s| s.id));
            out.push(Change {
                kind: LogKind::DeleteGroup {
                    name: g.name.clone(),
                },
                inverse: InverseOp::InsertGroup {
                    group: g.clone(),
                    members,
                    stays,
                },
            });
        }
    }
    for g in &new.groups {
        match old.groups.iter().find(|x| x.id == g.id) {
            None => out.push(Change {
                kind: LogKind::AddGroup {
                    name: g.name.clone(),
                },
                inverse: InverseOp::RemoveGroup { id: g.id },
            }),
            Some(o) => {
                if o.name != g.name {
                    out.push(Change {
                        kind: LogKind::RenameGroup {
                            from: o.name.clone(),
                            to: g.name.clone(),
                        },
                        inverse: InverseOp::SetGroupName {
                            id: g.id,
                            name: o.name.clone(),
                        },
                    });
                }
                if o.color != g.color {
                    out.push(Change {
                        kind: LogKind::SetGroupColor {
                            name: g.name.clone(),
                        },
                        inverse: InverseOp::SetGroupColor {
                            id: g.id,
                            color: o.color,
                        },
                    });
                }
            }
        }
    }

    // --- Persons ---
    for p in &old.persons {
        if !new.persons.iter().any(|x| x.id == p.id) {
            let stays: Vec<Stay> = old
                .stays
                .iter()
                .filter(|s| s.subject == Subject::Person(p.id))
                .cloned()
                .collect();
            cascaded_stays.extend(stays.iter().map(|s| s.id));
            out.push(Change {
                kind: LogKind::DeletePerson {
                    name: p.name.clone(),
                },
                inverse: InverseOp::InsertPerson {
                    person: p.clone(),
                    stays,
                },
            });
        }
    }
    for p in &new.persons {
        match old.persons.iter().find(|x| x.id == p.id) {
            None => out.push(Change {
                kind: LogKind::AddPerson {
                    name: p.name.clone(),
                },
                inverse: InverseOp::RemovePerson { id: p.id },
            }),
            Some(o) => {
                if o.name != p.name {
                    out.push(Change {
                        kind: LogKind::RenamePerson {
                            from: o.name.clone(),
                            to: p.name.clone(),
                        },
                        inverse: InverseOp::SetPersonName {
                            id: p.id,
                            name: o.name.clone(),
                        },
                    });
                }
                if o.group != p.group && !detached_persons.contains(&p.id) {
                    out.push(Change {
                        kind: LogKind::SetPersonGroup {
                            person: p.name.clone(),
                            from: group_label(old, o.group),
                            to: group_label(new, p.group),
                        },
                        inverse: InverseOp::SetPersonGroup {
                            id: p.id,
                            group: o.group,
                        },
                    });
                }
            }
        }
    }

    // --- Stays (skip ones already folded into a Delete* above) ---
    for s in &old.stays {
        if cascaded_stays.contains(&s.id) {
            continue;
        }
        if !new.stays.iter().any(|x| x.id == s.id) {
            out.push(Change {
                kind: LogKind::RemoveStay {
                    subject: subj(old, s.subject),
                    housing: hname(old, s.housing),
                },
                inverse: InverseOp::InsertStay { stay: s.clone() },
            });
        }
    }
    for s in &new.stays {
        if cascaded_stays.contains(&s.id) {
            continue;
        }
        match old.stays.iter().find(|x| x.id == s.id) {
            None => out.push(Change {
                kind: LogKind::AddStay {
                    subject: subj(new, s.subject),
                    housing: hname(new, s.housing),
                },
                inverse: InverseOp::RemoveStay { id: s.id },
            }),
            Some(o) => {
                if o.housing != s.housing {
                    out.push(Change {
                        kind: LogKind::MoveStay {
                            subject: subj(new, s.subject),
                            from: hname(old, o.housing),
                            to: hname(new, s.housing),
                        },
                        inverse: InverseOp::SetStayHousing {
                            id: s.id,
                            housing: o.housing,
                        },
                    });
                }
                if o.subject != s.subject {
                    out.push(Change {
                        kind: LogKind::ChangeStaySubject {
                            from: subj(old, o.subject),
                            to: subj(new, s.subject),
                            housing: hname(new, s.housing),
                        },
                        inverse: InverseOp::SetStaySubject {
                            id: s.id,
                            subject: o.subject,
                        },
                    });
                }
                if o.arrival != s.arrival || o.departure != s.departure {
                    out.push(Change {
                        kind: LogKind::ChangeStayDates {
                            subject: subj(new, s.subject),
                            housing: hname(new, s.housing),
                            from: format!("{} – {}", o.arrival, o.departure),
                            to: format!("{} – {}", s.arrival, s.departure),
                        },
                        inverse: InverseOp::SetStayDates {
                            id: s.id,
                            arrival: o.arrival,
                            departure: o.departure,
                        },
                    });
                }
            }
        }
    }

    out
}

/// Render a localized, human-readable description of a journal entry. `log` is
/// the full changelog (so an `Undo` entry can describe what it reverted).
pub fn describe(entry: &LogEntry, log: &[LogEntry], lang: Lang) -> String {
    let grp = |s: &str| -> String {
        if s.is_empty() {
            tr(lang, "(no group)").to_owned()
        } else {
            s.to_owned()
        }
    };
    match &entry.kind {
        LogKind::AddHousing { name } => format!("{}: {name}", tr(lang, "Created housing")),
        LogKind::DeleteHousing { name } => format!("{}: {name}", tr(lang, "Deleted housing")),
        LogKind::RenameHousing { from, to } => {
            format!("{}: {from} → {to}", tr(lang, "Renamed housing"))
        }
        LogKind::SetCapacity { name, from, to } => {
            format!("{} {name}: {from} → {to}", tr(lang, "Changed capacity of"))
        }
        LogKind::EditNotes { name } => format!("{} {name}", tr(lang, "Edited notes of")),

        LogKind::AddGroup { name } => format!("{}: {name}", tr(lang, "Created group")),
        LogKind::DeleteGroup { name } => format!("{}: {name}", tr(lang, "Deleted group")),
        LogKind::RenameGroup { from, to } => {
            format!("{}: {from} → {to}", tr(lang, "Renamed group"))
        }
        LogKind::SetGroupColor { name } => format!("{} {name}", tr(lang, "Changed colour of")),

        LogKind::AddPerson { name } => format!("{}: {name}", tr(lang, "Added person")),
        LogKind::DeletePerson { name } => format!("{}: {name}", tr(lang, "Deleted person")),
        LogKind::RenamePerson { from, to } => {
            format!("{}: {from} → {to}", tr(lang, "Renamed person"))
        }
        LogKind::SetPersonGroup { person, from, to } => format!(
            "{} {person}: {} → {}",
            tr(lang, "Changed group of"),
            grp(from),
            grp(to)
        ),

        LogKind::AddStay { subject, housing } => {
            format!("{}: {subject} → {housing}", tr(lang, "Added stay"))
        }
        LogKind::RemoveStay { subject, housing } => {
            format!("{}: {subject} → {housing}", tr(lang, "Removed stay"))
        }
        LogKind::MoveStay { subject, from, to } => {
            format!("{}: {subject} ({from} → {to})", tr(lang, "Moved stay"))
        }
        LogKind::ChangeStaySubject { from, to, housing } => {
            format!(
                "{} ({housing}): {from} → {to}",
                tr(lang, "Changed occupant of stay")
            )
        }
        LogKind::ChangeStayDates {
            subject,
            housing,
            from,
            to,
        } => {
            format!(
                "{}: {subject} ({housing}) {from} → {to}",
                tr(lang, "Changed dates of stay")
            )
        }

        LogKind::LoadedExample => tr(lang, "Loaded example data").to_owned(),
        LogKind::LoadedFile { name } => format!("{}: {name}", tr(lang, "Loaded plan from file")),
        LogKind::LoadedFileNoHistory { name } => {
            format!(
                "{}: {name}",
                tr(lang, "Loaded a plan with no change history")
            )
        }
        LogKind::Undo { undoes } => match log.iter().find(|e| e.id == *undoes) {
            Some(e) if !matches!(e.kind, LogKind::Undo { .. }) => {
                format!("{}: {}", tr(lang, "Undid"), describe(e, log, lang))
            }
            _ => format!("{} #{undoes}", tr(lang, "Undid")),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Person;
    use chrono::NaiveDate;

    fn d(n: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 1, n).unwrap()
    }

    #[test]
    fn rename_group_diff_and_inverse() {
        let mut old = Plan::default();
        let g = old.new_id();
        old.groups.push(Group {
            id: g,
            name: "A".into(),
            color: [1, 2, 3],
        });
        let mut new = old.clone();
        new.groups[0].name = "B".into();

        let changes = diff(&old, &new);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].kind, LogKind::RenameGroup { .. }));

        let mut reverted = new.clone();
        apply_inverse(&mut reverted, &changes[0].inverse);
        assert_eq!(reverted.groups[0].name, "A");
    }

    #[test]
    fn delete_housing_folds_its_stays() {
        let mut old = Plan::default();
        let h = old.new_id();
        old.housings.push(Housing {
            id: h,
            name: "H".into(),
            capacity: 2,
            notes: String::new(),
        });
        let p = old.new_id();
        old.persons.push(Person {
            id: p,
            name: "P".into(),
            group: None,
        });
        let s = old.new_id();
        old.stays.push(Stay {
            id: s,
            subject: Subject::Person(p),
            housing: h,
            arrival: d(1),
            departure: d(3),
        });

        let mut new = old.clone();
        new.housings.clear();
        new.stays.clear();

        // One DeleteHousing — the cascaded stay is folded in, not a separate entry.
        let changes = diff(&old, &new);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].kind, LogKind::DeleteHousing { .. }));

        // Its inverse restores both the housing and the stay.
        let mut reverted = new.clone();
        apply_inverse(&mut reverted, &changes[0].inverse);
        assert_eq!(reverted.housings.len(), 1);
        assert_eq!(reverted.stays.len(), 1);
    }
}
