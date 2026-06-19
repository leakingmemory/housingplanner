//! Minimal, dependency-free localization.
//!
//! UI code wraps English source strings in [`tr`]; for [`Lang::Swedish`] the
//! source is looked up in [`sv`] and any string without a translation falls back
//! to English, so partial coverage is always safe.

use serde::{Deserialize, Serialize};

/// Supported interface languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    English,
    Swedish,
    Norwegian,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::English
    }
}

impl Lang {
    /// All languages, for building a selector.
    pub const ALL: [Lang; 3] = [Lang::English, Lang::Swedish, Lang::Norwegian];

    /// Native name shown in the language picker.
    pub fn label(self) -> &'static str {
        match self {
            Lang::English => "English",
            Lang::Swedish => "Svenska",
            Lang::Norwegian => "Norsk bokmål",
        }
    }

    /// Best-effort default from the `LANG` / `LC_*` environment variables.
    pub fn from_env() -> Lang {
        let v = std::env::var("LC_ALL")
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .or_else(|_| std::env::var("LANG"))
            .unwrap_or_default()
            .to_ascii_lowercase();
        if v.starts_with("sv") {
            Lang::Swedish
        } else if v.starts_with("nb") || v.starts_with("no") || v.starts_with("nn") {
            Lang::Norwegian
        } else {
            Lang::English
        }
    }
}

/// Translate an English source string for the given language.
pub fn tr(lang: Lang, en: &'static str) -> &'static str {
    match lang {
        Lang::English => en,
        Lang::Swedish => sv(en),
        Lang::Norwegian => nb(en),
    }
}

/// Swedish translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn sv(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Översikt",
        "👥 Groups" => "👥 Grupper",
        "🧍 Persons" => "🧍 Personer",
        "🏠 Housings" => "🏠 Boenden",

        // Top bar
        "From:" => "Från:",
        "Days:" => "Dagar:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Eller Ctrl/Cmd + scroll (nyp på styrplatta) över tidslinjen"
        }
        "Today" => "Idag",
        "Fit to stays" => "Anpassa till vistelser",
        "💾 Save…" => "💾 Spara…",
        "📂 Load…" => "📂 Öppna…",
        "ℹ About" => "ℹ Om",
        "Language" => "Språk",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Sparad →",
        "Loaded ←" => "Öppnad ←",
        "Save failed:" => "Kunde inte spara:",
        "Encode failed:" => "Kunde inte koda:",
        "Read failed:" => "Kunde inte läsa:",
        "Parse failed:" => "Kunde inte tolka:",
        "File save is not available on Android yet." => {
            "Filsparning är inte tillgänglig på Android ännu."
        }
        "File load is not available on Android yet." => {
            "Filöppning är inte tillgänglig på Android ännu."
        }
        "Housing Planner plan" => "Housing Planner-plan",

        // About window
        "About / Licenses" => "Om / Licenser",
        "Version" => "Version",
        "Plan who stays where, and when." => "Planera vem som bor var, och när.",
        "📋 Copy dependency licenses" => "📋 Kopiera beroendelicenser",
        "This application" => "Den här applikationen",
        "Third-party dependencies" => "Tredjepartsberoenden",

        // Overview tab
        "Welcome to Housing Planner" => "Välkommen till Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Lägg till boenden, grupper och personer i flikarna ovan —"
        }
        "📋 Load example data" => "📋 Ladda exempeldata",
        "Add a housing in the Housings tab to start planning." => {
            "Lägg till ett boende i fliken Boenden för att börja planera."
        }

        // Selectors / common
        "Group" => "Grupp",
        "Person" => "Person",
        "Housing" => "Boende",
        "➕ New" => "➕ Ny",
        "Stays:" => "Vistelser:",
        "Stays (individual):" => "Vistelser (individuella):",
        "➕ Add stay" => "➕ Lägg till vistelse",
        "Add a housing and a person/group first." => {
            "Lägg till ett boende och en person/grupp först."
        }
        "(no stays)" => "(inga vistelser)",
        "(group)" => "(grupp)",

        // Groups tab
        "No groups yet — add one." => "Inga grupper än — lägg till en.",
        "🗑 Delete group" => "🗑 Ta bort grupp",
        "Members:" => "Medlemmar:",
        "(no members)" => "(inga medlemmar)",
        "➕ Add existing…" => "➕ Lägg till befintlig…",
        "➕ New person" => "➕ Ny person",
        "Select or create a group." => "Välj eller skapa en grupp.",
        "No stays for this group yet." => "Inga vistelser för den här gruppen än.",

        // Persons tab
        "No persons yet — add one." => "Inga personer än — lägg till en.",
        "— no group —" => "— ingen grupp —",
        "🗑 Delete person" => "🗑 Ta bort person",
        "Select or create a person." => "Välj eller skapa en person.",
        "No stays for this person yet." => "Inga vistelser för den här personen än.",

        // Housings tab
        "No housings yet — add one." => "Inga boenden än — lägg till ett.",
        "Capacity" => "Kapacitet",
        "Notes:" => "Anteckningar:",
        "🗑 Delete housing" => "🗑 Ta bort boende",
        "Select or create a housing." => "Välj eller skapa ett boende.",
        "No stays in this housing yet." => "Inga vistelser i det här boendet än.",

        // Timeline
        "cap" => "kap",
        "To:" => "Till:",
        "Nights:" => "Nätter:",
        "People:" => "Personer:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Även bokad någon annanstans samtidigt",
        "<deleted person>" => "<borttagen person>",
        "<deleted group>" => "<borttagen grupp>",

        // Fallback: English
        other => other,
    }
}

/// Norwegian Bokmål translations, keyed by the English source string. Unknown
/// strings fall back to English.
fn nb(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Oversikt",
        "👥 Groups" => "👥 Grupper",
        "🧍 Persons" => "🧍 Personer",
        "🏠 Housings" => "🏠 Boliger",

        // Top bar
        "From:" => "Fra:",
        "Days:" => "Dager:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Eller Ctrl/Cmd + rull (knip på styreplate) over tidslinjen"
        }
        "Today" => "I dag",
        "Fit to stays" => "Tilpass til opphold",
        "💾 Save…" => "💾 Lagre…",
        "📂 Load…" => "📂 Åpne…",
        "ℹ About" => "ℹ Om",
        "Language" => "Språk",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Lagret →",
        "Loaded ←" => "Åpnet ←",
        "Save failed:" => "Kunne ikke lagre:",
        "Encode failed:" => "Kunne ikke kode:",
        "Read failed:" => "Kunne ikke lese:",
        "Parse failed:" => "Kunne ikke tolke:",
        "File save is not available on Android yet." => {
            "Fillagring er ikke tilgjengelig på Android ennå."
        }
        "File load is not available on Android yet." => {
            "Filåpning er ikke tilgjengelig på Android ennå."
        }
        "Housing Planner plan" => "Housing Planner-plan",

        // About window
        "About / Licenses" => "Om / Lisenser",
        "Version" => "Versjon",
        "Plan who stays where, and when." => "Planlegg hvem som bor hvor, og når.",
        "📋 Copy dependency licenses" => "📋 Kopier avhengighetslisenser",
        "This application" => "Denne applikasjonen",
        "Third-party dependencies" => "Tredjepartsavhengigheter",

        // Overview tab
        "Welcome to Housing Planner" => "Velkommen til Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Legg til boliger, grupper og personer i fanene over —"
        }
        "📋 Load example data" => "📋 Last inn eksempeldata",
        "Add a housing in the Housings tab to start planning." => {
            "Legg til en bolig i fanen Boliger for å begynne å planlegge."
        }

        // Selectors / common
        "Group" => "Gruppe",
        "Person" => "Person",
        "Housing" => "Bolig",
        "➕ New" => "➕ Ny",
        "Stays:" => "Opphold:",
        "Stays (individual):" => "Opphold (individuelle):",
        "➕ Add stay" => "➕ Legg til opphold",
        "Add a housing and a person/group first." => "Legg til en bolig og en person/gruppe først.",
        "(no stays)" => "(ingen opphold)",
        "(group)" => "(gruppe)",

        // Groups tab
        "No groups yet — add one." => "Ingen grupper ennå — legg til en.",
        "🗑 Delete group" => "🗑 Slett gruppe",
        "Members:" => "Medlemmer:",
        "(no members)" => "(ingen medlemmer)",
        "➕ Add existing…" => "➕ Legg til eksisterende…",
        "➕ New person" => "➕ Ny person",
        "Select or create a group." => "Velg eller opprett en gruppe.",
        "No stays for this group yet." => "Ingen opphold for denne gruppen ennå.",

        // Persons tab
        "No persons yet — add one." => "Ingen personer ennå — legg til en.",
        "— no group —" => "— ingen gruppe —",
        "🗑 Delete person" => "🗑 Slett person",
        "Select or create a person." => "Velg eller opprett en person.",
        "No stays for this person yet." => "Ingen opphold for denne personen ennå.",

        // Housings tab
        "No housings yet — add one." => "Ingen boliger ennå — legg til en.",
        "Capacity" => "Kapasitet",
        "Notes:" => "Notater:",
        "🗑 Delete housing" => "🗑 Slett bolig",
        "Select or create a housing." => "Velg eller opprett en bolig.",
        "No stays in this housing yet." => "Ingen opphold i denne boligen ennå.",

        // Timeline
        "cap" => "kap",
        "To:" => "Til:",
        "Nights:" => "Netter:",
        "People:" => "Personer:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Også booket et annet sted samtidig",
        "<deleted person>" => "<slettet person>",
        "<deleted group>" => "<slettet gruppe>",

        // Fallback: English
        other => other,
    }
}
