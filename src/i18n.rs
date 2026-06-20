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
    NorwegianNynorsk,
    NorthernSami,
    Danish,
    Ukrainian,
    German,
    French,
    Italian,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::English
    }
}

impl Lang {
    /// All languages, for building a selector.
    pub const ALL: [Lang; 10] = [
        Lang::English,
        Lang::Swedish,
        Lang::Norwegian,
        Lang::NorwegianNynorsk,
        Lang::NorthernSami,
        Lang::Danish,
        Lang::Ukrainian,
        Lang::German,
        Lang::French,
        Lang::Italian,
    ];

    /// Native name shown in the language picker.
    pub fn label(self) -> &'static str {
        match self {
            Lang::English => "English",
            Lang::Swedish => "Svenska",
            Lang::Norwegian => "Norsk bokmål",
            Lang::NorwegianNynorsk => "Norsk nynorsk",
            Lang::NorthernSami => "Davvisámegiella",
            Lang::Danish => "Dansk",
            Lang::Ukrainian => "Українська",
            Lang::German => "Deutsch",
            Lang::French => "Français",
            Lang::Italian => "Italiano",
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
        } else if v.starts_with("nn") {
            Lang::NorwegianNynorsk
        } else if v.starts_with("nb") || v.starts_with("no") {
            Lang::Norwegian
        } else if v.starts_with("se") {
            Lang::NorthernSami
        } else if v.starts_with("da") {
            Lang::Danish
        } else if v.starts_with("uk") {
            Lang::Ukrainian
        } else if v.starts_with("de") {
            Lang::German
        } else if v.starts_with("fr") {
            Lang::French
        } else if v.starts_with("it") {
            Lang::Italian
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
        Lang::NorwegianNynorsk => nn(en),
        Lang::NorthernSami => se(en),
        Lang::Danish => da(en),
        Lang::Ukrainian => uk(en),
        Lang::German => de(en),
        Lang::French => fr(en),
        Lang::Italian => it(en),
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

        // Changelog
        "📜 Changelog" => "📜 Ändringslogg",
        "↩ Undo last change" => "↩ Ångra senaste ändring",
        "entries" => "poster",
        "No changes yet." => "Inga ändringar än.",
        "(no group)" => "(ingen grupp)",
        "Created housing" => "Skapade boende",
        "Deleted housing" => "Tog bort boende",
        "Renamed housing" => "Bytte namn på boende",
        "Changed capacity of" => "Ändrade kapacitet för",
        "Edited notes of" => "Redigerade anteckningar för",
        "Created group" => "Skapade grupp",
        "Deleted group" => "Tog bort grupp",
        "Renamed group" => "Bytte namn på grupp",
        "Changed colour of" => "Ändrade färg för",
        "Added person" => "La till person",
        "Deleted person" => "Tog bort person",
        "Renamed person" => "Bytte namn på person",
        "Changed group of" => "Ändrade grupp för",
        "Added stay" => "La till vistelse",
        "Removed stay" => "Tog bort vistelse",
        "Moved stay" => "Flyttade vistelse",
        "Changed occupant of stay" => "Ändrade person för vistelse",
        "Changed dates of stay" => "Ändrade datum för vistelse",
        "Loaded example data" => "Laddade exempeldata",
        "Loaded plan from file" => "Öppnade plan från fil",
        "Loaded a plan with no change history" => "Öppnade en plan utan ändringshistorik",
        "Undid" => "Ångrade",
        // Files / close
        "💾 Save" => "💾 Spara",
        "Save As…" => "Spara som…",
        "Unsaved changes" => "Osparade ändringar",
        "You have unsaved changes. Save before closing?" => {
            "Du har osparade ändringar. Spara innan du stänger?"
        }
        "Save" => "Spara",
        "Discard" => "Förkasta",
        "Cancel" => "Avbryt",
        "untitled" => "namnlös",
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

        // Changelog
        "📜 Changelog" => "📜 Endringslogg",
        "↩ Undo last change" => "↩ Angre siste endring",
        "entries" => "oppføringer",
        "No changes yet." => "Ingen endringer ennå.",
        "(no group)" => "(ingen gruppe)",
        "Created housing" => "Opprettet bolig",
        "Deleted housing" => "Slettet bolig",
        "Renamed housing" => "Ga nytt navn til bolig",
        "Changed capacity of" => "Endret kapasitet for",
        "Edited notes of" => "Redigerte notater for",
        "Created group" => "Opprettet gruppe",
        "Deleted group" => "Slettet gruppe",
        "Renamed group" => "Ga nytt navn til gruppe",
        "Changed colour of" => "Endret farge for",
        "Added person" => "La til person",
        "Deleted person" => "Slettet person",
        "Renamed person" => "Ga nytt navn til person",
        "Changed group of" => "Endret gruppe for",
        "Added stay" => "La til opphold",
        "Removed stay" => "Fjernet opphold",
        "Moved stay" => "Flyttet opphold",
        "Changed occupant of stay" => "Endret person for opphold",
        "Changed dates of stay" => "Endret datoer for opphold",
        "Loaded example data" => "Lastet inn eksempeldata",
        "Loaded plan from file" => "Åpnet plan fra fil",
        "Loaded a plan with no change history" => "Åpnet en plan uten endringshistorikk",
        "Undid" => "Angret",
        // Files / close
        "💾 Save" => "💾 Lagre",
        "Save As…" => "Lagre som…",
        "Unsaved changes" => "Ulagrede endringer",
        "You have unsaved changes. Save before closing?" => {
            "Du har ulagrede endringer. Lagre før du lukker?"
        }
        "Save" => "Lagre",
        "Discard" => "Forkast",
        "Cancel" => "Avbryt",
        "untitled" => "uten navn",
        // Fallback: English
        other => other,
    }
}

/// Norwegian Nynorsk translations, keyed by the English source string. Unknown
/// strings fall back to English.
fn nn(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Oversikt",
        "👥 Groups" => "👥 Grupper",
        "🧍 Persons" => "🧍 Personar",
        "🏠 Housings" => "🏠 Bustader",

        // Top bar
        "From:" => "Frå:",
        "Days:" => "Dagar:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Eller Ctrl/Cmd + rull (knip på styreplate) over tidslinja"
        }
        "Today" => "I dag",
        "Fit to stays" => "Tilpass til opphald",
        "📂 Load…" => "📂 Opne…",
        "ℹ About" => "ℹ Om",
        "Language" => "Språk",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Lagra →",
        "Loaded ←" => "Opna ←",
        "Save failed:" => "Kunne ikkje lagre:",
        "Encode failed:" => "Kunne ikkje kode:",
        "Read failed:" => "Kunne ikkje lese:",
        "Parse failed:" => "Kunne ikkje tolke:",
        "File save is not available on Android yet." => {
            "Fillagring er ikkje tilgjengeleg på Android enno."
        }
        "File load is not available on Android yet." => {
            "Filopning er ikkje tilgjengeleg på Android enno."
        }
        "Housing Planner plan" => "Housing Planner-plan",

        // About window
        "About / Licenses" => "Om / Lisensar",
        "Version" => "Versjon",
        "Plan who stays where, and when." => "Planlegg kven som bur kvar, og når.",
        "📋 Copy dependency licenses" => "📋 Kopier avhengnadslisensar",
        "This application" => "Denne applikasjonen",
        "Third-party dependencies" => "Tredjepartsavhengnader",

        // Overview tab
        "Welcome to Housing Planner" => "Velkomen til Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Legg til bustader, grupper og personar i fanene over —"
        }
        "📋 Load example data" => "📋 Last inn eksempeldata",
        "Add a housing in the Housings tab to start planning." => {
            "Legg til ein bustad i fana Bustader for å byrje å planleggje."
        }

        // Selectors / common
        "Group" => "Gruppe",
        "Person" => "Person",
        "Housing" => "Bustad",
        "➕ New" => "➕ Ny",
        "Stays:" => "Opphald:",
        "Stays (individual):" => "Opphald (individuelle):",
        "➕ Add stay" => "➕ Legg til opphald",
        "Add a housing and a person/group first." => {
            "Legg til ein bustad og ein person/gruppe først."
        }
        "(no stays)" => "(ingen opphald)",
        "(group)" => "(gruppe)",

        // Groups tab
        "No groups yet — add one." => "Ingen grupper enno — legg til ei.",
        "🗑 Delete group" => "🗑 Slett gruppe",
        "Members:" => "Medlemmer:",
        "(no members)" => "(ingen medlemmer)",
        "➕ Add existing…" => "➕ Legg til eksisterande…",
        "➕ New person" => "➕ Ny person",
        "Select or create a group." => "Vel eller opprett ei gruppe.",
        "No stays for this group yet." => "Ingen opphald for denne gruppa enno.",

        // Persons tab
        "No persons yet — add one." => "Ingen personar enno — legg til ein.",
        "— no group —" => "— inga gruppe —",
        "🗑 Delete person" => "🗑 Slett person",
        "Select or create a person." => "Vel eller opprett ein person.",
        "No stays for this person yet." => "Ingen opphald for denne personen enno.",

        // Housings tab
        "No housings yet — add one." => "Ingen bustader enno — legg til ein.",
        "Capacity" => "Kapasitet",
        "Notes:" => "Notat:",
        "🗑 Delete housing" => "🗑 Slett bustad",
        "Select or create a housing." => "Vel eller opprett ein bustad.",
        "No stays in this housing yet." => "Ingen opphald i denne bustaden enno.",

        // Timeline
        "cap" => "kap",
        "To:" => "Til:",
        "Nights:" => "Netter:",
        "People:" => "Personar:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Også booka ein annan stad samstundes",
        "<deleted person>" => "<sletta person>",
        "<deleted group>" => "<sletta gruppe>",

        // Changelog
        "📜 Changelog" => "📜 Endringslogg",
        "↩ Undo last change" => "↩ Angre siste endring",
        "entries" => "oppføringar",
        "No changes yet." => "Ingen endringar enno.",
        "(no group)" => "(inga gruppe)",
        "Created housing" => "Oppretta bustad",
        "Deleted housing" => "Sletta bustad",
        "Renamed housing" => "Gav nytt namn til bustad",
        "Changed capacity of" => "Endra kapasitet for",
        "Edited notes of" => "Redigerte notat for",
        "Created group" => "Oppretta gruppe",
        "Deleted group" => "Sletta gruppe",
        "Renamed group" => "Gav nytt namn til gruppe",
        "Changed colour of" => "Endra farge for",
        "Added person" => "La til person",
        "Deleted person" => "Sletta person",
        "Renamed person" => "Gav nytt namn til person",
        "Changed group of" => "Endra gruppe for",
        "Added stay" => "La til opphald",
        "Removed stay" => "Fjerna opphald",
        "Moved stay" => "Flytta opphald",
        "Changed occupant of stay" => "Endra person for opphald",
        "Changed dates of stay" => "Endra datoar for opphald",
        "Loaded example data" => "Lasta inn eksempeldata",
        "Loaded plan from file" => "Opna plan frå fil",
        "Loaded a plan with no change history" => "Opna ein plan utan endringshistorikk",
        "Undid" => "Angra",
        // Files / close
        "💾 Save" => "💾 Lagre",
        "Save As…" => "Lagre som…",
        "Unsaved changes" => "Ulagra endringar",
        "You have unsaved changes. Save before closing?" => {
            "Du har ulagra endringar. Lagre før du lukkar?"
        }
        "Save" => "Lagre",
        "Discard" => "Forkast",
        "Cancel" => "Avbryt",
        "untitled" => "utan namn",
        // Fallback: English
        other => other,
    }
}

/// Northern Sami (davvisámegiella) translations, keyed by the English source
/// string. Unknown strings fall back to English.
///
/// NOTE: best-effort translation — should be reviewed by a fluent speaker.
fn se(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Geahčastat",
        "👥 Groups" => "👥 Joavkkut",
        "🧍 Persons" => "🧍 Olbmot",
        "🏠 Housings" => "🏠 Orrunsajit",

        // Top bar
        "From:" => "Rájes:",
        "Days:" => "Beaivvit:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Dahje Ctrl/Cmd + rulle (njeaikkas touchpad) áigelinnjá badjel"
        }
        "Today" => "Odne",
        "Fit to stays" => "Heivet orrumiidda",
        "📂 Load…" => "📂 Raba…",
        "ℹ About" => "ℹ Birra",
        "Language" => "Giella",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Vurkejuvvon →",
        "Loaded ←" => "Viežžojuvvon ←",
        "Save failed:" => "Ii sáhttán vurket:",
        "Encode failed:" => "Ii sáhttán kodet:",
        "Read failed:" => "Ii sáhttán lohkat:",
        "Parse failed:" => "Ii sáhttán dulkot:",
        "File save is not available on Android yet." => {
            "Fiilavurken ii leat vel olamuttus Android:as."
        }
        "File load is not available on Android yet." => {
            "Fiilaviežžan ii leat vel olamuttus Android:as."
        }
        "Housing Planner plan" => "Housing Planner-plána",

        // About window
        "About / Licenses" => "Birra / Liseanssat",
        "Version" => "Veršuvdna",
        "Plan who stays where, and when." => "Plánes gii orru gos, ja goas.",
        "📋 Copy dependency licenses" => "📋 Máŋge sorjjasvuođaliseanssaid",
        "This application" => "Dát prográmma",
        "Third-party dependencies" => "Goalmmát beali sorjjasvuođat",

        // Overview tab
        "Welcome to Housing Planner" => "Bures boahtin Housing Planner:ii",
        "Add housings, groups and people in the tabs above —" => {
            "Lasit orrunsajiid, joavkkuid ja olbmuid bajábeale gilkoriin —"
        }
        "📋 Load example data" => "📋 Viečča ovdamearkadáhtaid",
        "Add a housing in the Housings tab to start planning." => {
            "Lasit orrunsaji Orrunsajit-gilkoris vai sáhtát álgit plánet."
        }

        // Selectors / common
        "Group" => "Joavku",
        "Person" => "Olmmoš",
        "Housing" => "Orrunsadji",
        "➕ New" => "➕ Ođas",
        "Stays:" => "Orrumat:",
        "Stays (individual):" => "Orrumat (ovttaskas):",
        "➕ Add stay" => "➕ Lasit orruma",
        "Add a housing and a person/group first." => "Lasit vuos orrunsaji ja olbmo/joavkku.",
        "(no stays)" => "(eai orrumat)",
        "(group)" => "(joavku)",

        // Groups tab
        "No groups yet — add one." => "Eai joavkkut velá — lasit ovtta.",
        "🗑 Delete group" => "🗑 Sihko joavkku",
        "Members:" => "Lahtut:",
        "(no members)" => "(eai lahtut)",
        "➕ Add existing…" => "➕ Lasit dálá…",
        "➕ New person" => "➕ Ođđa olmmoš",
        "Select or create a group." => "Vállje dahje ráhkat joavkku.",
        "No stays for this group yet." => "Eai orrumat dán joavkkus velá.",

        // Persons tab
        "No persons yet — add one." => "Eai olbmot velá — lasit ovtta.",
        "— no group —" => "— ii joavku —",
        "🗑 Delete person" => "🗑 Sihko olbmo",
        "Select or create a person." => "Vállje dahje ráhkat olbmo.",
        "No stays for this person yet." => "Eai orrumat dán olbmui velá.",

        // Housings tab
        "No housings yet — add one." => "Eai orrunsajit velá — lasit ovtta.",
        "Capacity" => "Kapasitehta",
        "Notes:" => "Mearkkašumit:",
        "🗑 Delete housing" => "🗑 Sihko orrunsaji",
        "Select or create a housing." => "Vállje dahje ráhkat orrunsaji.",
        "No stays in this housing yet." => "Eai orrumat dán orrunsajis velá.",

        // Timeline
        "cap" => "kap",
        "To:" => "Gitta:",
        "Nights:" => "Ijat:",
        "People:" => "Olbmot:",
        "⚠ Also booked elsewhere at the same time" => {
            "⚠ Maiddái diŋgojuvvon eará sajis seamma áigge"
        }
        "<deleted person>" => "<sihkkojuvvon olmmoš>",
        "<deleted group>" => "<sihkkojuvvon joavku>",

        // Changelog
        "📜 Changelog" => "📜 Rievdadanlogga",
        "↩ Undo last change" => "↩ Máhcat maŋimuš rievdadusa",
        "entries" => "merkošumit",
        "No changes yet." => "Ii leat vel rievdadusaid.",
        "(no group)" => "(ii joavku)",
        "Created housing" => "Ráhkadii orrunsaji",
        "Deleted housing" => "Sihkui orrunsaji",
        "Renamed housing" => "Rievdadii orrunsaji nama",
        "Changed capacity of" => "Rievdadii kapasitehta",
        "Edited notes of" => "Doaimmahii notáhtaid",
        "Created group" => "Ráhkadii joavkku",
        "Deleted group" => "Sihkui joavkku",
        "Renamed group" => "Rievdadii joavkku nama",
        "Changed colour of" => "Rievdadii ivnni",
        "Added person" => "Lasihii olbmo",
        "Deleted person" => "Sihkui olbmo",
        "Renamed person" => "Rievdadii olbmo nama",
        "Changed group of" => "Rievdadii olbmo joavkku",
        "Added stay" => "Lasihii orruma",
        "Removed stay" => "Sihkui orruma",
        "Moved stay" => "Sirddii orruma",
        "Changed occupant of stay" => "Rievdadii orruma olbmo",
        "Changed dates of stay" => "Rievdadii orruma beivviid",
        "Loaded example data" => "Viežžai ovdamearkadáhtaid",
        "Loaded plan from file" => "Viežžai plána fiillas",
        "Loaded a plan with no change history" => "Viežžai plána mas ii lean rievdadanhistorjá",
        "Undid" => "Máhcahii",
        // Files / close
        "💾 Save" => "💾 Vurke",
        "Save As…" => "Vurke nugo…",
        "Unsaved changes" => "Vurkekeahtes rievdadusat",
        "You have unsaved changes. Save before closing?" => {
            "Dus leat vurkekeahtes rievdadusat. Háliidatgo vurket ovdal go giddet?"
        }
        "Save" => "Vurke",
        "Discard" => "Hilgo",
        "Cancel" => "Gaskkalduhte",
        "untitled" => "namahis",
        // Fallback: English
        other => other,
    }
}

/// Danish translations, keyed by the English source string. Unknown strings fall
/// back to English.
fn da(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Oversigt",
        "👥 Groups" => "👥 Grupper",
        "🧍 Persons" => "🧍 Personer",
        "🏠 Housings" => "🏠 Boliger",

        // Top bar
        "From:" => "Fra:",
        "Days:" => "Dage:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Eller Ctrl/Cmd + rul (knib på pegefelt) over tidslinjen"
        }
        "Today" => "I dag",
        "Fit to stays" => "Tilpas til ophold",
        "📂 Load…" => "📂 Åbn…",
        "ℹ About" => "ℹ Om",
        "Language" => "Sprog",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Gemt →",
        "Loaded ←" => "Indlæst ←",
        "Save failed:" => "Kunne ikke gemme:",
        "Encode failed:" => "Kunne ikke kode:",
        "Read failed:" => "Kunne ikke læse:",
        "Parse failed:" => "Kunne ikke fortolke:",
        "File save is not available on Android yet." => {
            "Det er endnu ikke muligt at gemme til fil på Android."
        }
        "File load is not available on Android yet." => {
            "Det er endnu ikke muligt at indlæse fra fil på Android."
        }
        "Housing Planner plan" => "Housing Planner-plan",

        // About window
        "About / Licenses" => "Om / Licenser",
        "Version" => "Version",
        "Plan who stays where, and when." => "Planlæg hvem der bor hvor, og hvornår.",
        "📋 Copy dependency licenses" => "📋 Kopiér afhængighedslicenser",
        "This application" => "Denne applikation",
        "Third-party dependencies" => "Tredjepartsafhængigheder",

        // Overview tab
        "Welcome to Housing Planner" => "Velkommen til Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Tilføj boliger, grupper og personer i fanerne ovenfor —"
        }
        "📋 Load example data" => "📋 Indlæs eksempeldata",
        "Add a housing in the Housings tab to start planning." => {
            "Tilføj en bolig under fanen Boliger for at begynde at planlægge."
        }

        // Selectors / common
        "Group" => "Gruppe",
        "Person" => "Person",
        "Housing" => "Bolig",
        "➕ New" => "➕ Ny",
        "Stays:" => "Ophold:",
        "Stays (individual):" => "Ophold (individuelle):",
        "➕ Add stay" => "➕ Tilføj ophold",
        "Add a housing and a person/group first." => "Tilføj en bolig og en person/gruppe først.",
        "(no stays)" => "(ingen ophold)",
        "(group)" => "(gruppe)",

        // Groups tab
        "No groups yet — add one." => "Ingen grupper endnu — tilføj en.",
        "🗑 Delete group" => "🗑 Slet gruppe",
        "Members:" => "Medlemmer:",
        "(no members)" => "(ingen medlemmer)",
        "➕ Add existing…" => "➕ Tilføj eksisterende…",
        "➕ New person" => "➕ Ny person",
        "Select or create a group." => "Vælg eller opret en gruppe.",
        "No stays for this group yet." => "Ingen ophold for denne gruppe endnu.",

        // Persons tab
        "No persons yet — add one." => "Ingen personer endnu — tilføj en.",
        "— no group —" => "— ingen gruppe —",
        "🗑 Delete person" => "🗑 Slet person",
        "Select or create a person." => "Vælg eller opret en person.",
        "No stays for this person yet." => "Ingen ophold for denne person endnu.",

        // Housings tab
        "No housings yet — add one." => "Ingen boliger endnu — tilføj en.",
        "Capacity" => "Kapacitet",
        "Notes:" => "Noter:",
        "🗑 Delete housing" => "🗑 Slet bolig",
        "Select or create a housing." => "Vælg eller opret en bolig.",
        "No stays in this housing yet." => "Ingen ophold i denne bolig endnu.",

        // Timeline
        "cap" => "kap",
        "To:" => "Til:",
        "Nights:" => "Nætter:",
        "People:" => "Personer:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Også booket et andet sted samtidig",
        "<deleted person>" => "<slettet person>",
        "<deleted group>" => "<slettet gruppe>",

        // Changelog
        "📜 Changelog" => "📜 Ændringslog",
        "↩ Undo last change" => "↩ Fortryd seneste ændring",
        "entries" => "poster",
        "No changes yet." => "Ingen ændringer endnu.",
        "(no group)" => "(ingen gruppe)",
        "Created housing" => "Oprettede bolig",
        "Deleted housing" => "Slettede bolig",
        "Renamed housing" => "Omdøbte bolig",
        "Changed capacity of" => "Ændrede kapacitet for",
        "Edited notes of" => "Redigerede noter for",
        "Created group" => "Oprettede gruppe",
        "Deleted group" => "Slettede gruppe",
        "Renamed group" => "Omdøbte gruppe",
        "Changed colour of" => "Ændrede farve for",
        "Added person" => "Tilføjede person",
        "Deleted person" => "Slettede person",
        "Renamed person" => "Omdøbte person",
        "Changed group of" => "Ændrede gruppe for",
        "Added stay" => "Tilføjede ophold",
        "Removed stay" => "Fjernede ophold",
        "Moved stay" => "Flyttede ophold",
        "Changed occupant of stay" => "Ændrede person for ophold",
        "Changed dates of stay" => "Ændrede datoer for ophold",
        "Loaded example data" => "Indlæste eksempeldata",
        "Loaded plan from file" => "Åbnede plan fra fil",
        "Loaded a plan with no change history" => "Åbnede en plan uden ændringshistorik",
        "Undid" => "Fortrød",
        // Files / close
        "💾 Save" => "💾 Gem",
        "Save As…" => "Gem som…",
        "Unsaved changes" => "Ugemte ændringer",
        "You have unsaved changes. Save before closing?" => {
            "Du har ugemte ændringer. Vil du gemme før du lukker?"
        }
        "Save" => "Gem",
        "Discard" => "Kassér",
        "Cancel" => "Annuller",
        "untitled" => "unavngivet",
        // Fallback: English
        other => other,
    }
}

/// Ukrainian translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn uk(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Огляд",
        "👥 Groups" => "👥 Групи",
        "🧍 Persons" => "🧍 Особи",
        "🏠 Housings" => "🏠 Помешкання",

        // Top bar
        "From:" => "Від:",
        "Days:" => "Дні:",
        "Zoom:" => "Масштаб:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Або Ctrl/Cmd + прокручування (щипок на тачпаді) над шкалою часу"
        }
        "Today" => "Сьогодні",
        "Fit to stays" => "Підлаштувати під перебування",
        "📂 Load…" => "📂 Відкрити…",
        "ℹ About" => "ℹ Про програму",
        "Language" => "Мова",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Збережено →",
        "Loaded ←" => "Завантажено ←",
        "Save failed:" => "Не вдалося зберегти:",
        "Encode failed:" => "Не вдалося закодувати:",
        "Read failed:" => "Не вдалося прочитати:",
        "Parse failed:" => "Не вдалося розібрати:",
        "File save is not available on Android yet." => {
            "Збереження у файл поки що недоступне на Android."
        }
        "File load is not available on Android yet." => {
            "Завантаження з файлу поки що недоступне на Android."
        }
        "Housing Planner plan" => "План Housing Planner",

        // About window
        "About / Licenses" => "Про програму / Ліцензії",
        "Version" => "Версія",
        "Plan who stays where, and when." => "Плануйте, хто де живе і коли.",
        "📋 Copy dependency licenses" => "📋 Копіювати ліцензії залежностей",
        "This application" => "Ця програма",
        "Third-party dependencies" => "Сторонні залежності",

        // Overview tab
        "Welcome to Housing Planner" => "Ласкаво просимо до Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Додавайте помешкання, групи та людей у вкладках вище —"
        }
        "📋 Load example data" => "📋 Завантажити приклад даних",
        "Add a housing in the Housings tab to start planning." => {
            "Додайте помешкання у вкладці «Помешкання», щоб почати планувати."
        }

        // Selectors / common
        "Group" => "Група",
        "Person" => "Особа",
        "Housing" => "Помешкання",
        "➕ New" => "➕ Створити",
        "Stays:" => "Перебування:",
        "Stays (individual):" => "Перебування (індивідуальні):",
        "➕ Add stay" => "➕ Додати перебування",
        "Add a housing and a person/group first." => "Спершу додайте помешкання та особу/групу.",
        "(no stays)" => "(немає перебувань)",
        "(group)" => "(група)",

        // Groups tab
        "No groups yet — add one." => "Груп ще немає — додайте одну.",
        "🗑 Delete group" => "🗑 Видалити групу",
        "Members:" => "Учасники:",
        "(no members)" => "(немає учасників)",
        "➕ Add existing…" => "➕ Додати наявного…",
        "➕ New person" => "➕ Нова особа",
        "Select or create a group." => "Виберіть або створіть групу.",
        "No stays for this group yet." => "Для цієї групи ще немає перебувань.",

        // Persons tab
        "No persons yet — add one." => "Осіб ще немає — додайте одну.",
        "— no group —" => "— без групи —",
        "🗑 Delete person" => "🗑 Видалити особу",
        "Select or create a person." => "Виберіть або створіть особу.",
        "No stays for this person yet." => "Для цієї особи ще немає перебувань.",

        // Housings tab
        "No housings yet — add one." => "Помешкань ще немає — додайте одне.",
        "Capacity" => "Місткість",
        "Notes:" => "Нотатки:",
        "🗑 Delete housing" => "🗑 Видалити помешкання",
        "Select or create a housing." => "Виберіть або створіть помешкання.",
        "No stays in this housing yet." => "У цьому помешканні ще немає перебувань.",

        // Timeline
        "cap" => "міст.",
        "To:" => "До:",
        "Nights:" => "Ночей:",
        "People:" => "Людей:",
        "⚠ Also booked elsewhere at the same time" => {
            "⚠ Також заброньовано в іншому місці в той самий час"
        }
        "<deleted person>" => "<видалена особа>",
        "<deleted group>" => "<видалена група>",

        // Changelog
        "📜 Changelog" => "📜 Журнал змін",
        "↩ Undo last change" => "↩ Скасувати останню зміну",
        "entries" => "записів",
        "No changes yet." => "Ще немає змін.",
        "(no group)" => "(без групи)",
        "Created housing" => "Створено помешкання",
        "Deleted housing" => "Видалено помешкання",
        "Renamed housing" => "Перейменовано помешкання",
        "Changed capacity of" => "Змінено місткість для",
        "Edited notes of" => "Відредаговано нотатки для",
        "Created group" => "Створено групу",
        "Deleted group" => "Видалено групу",
        "Renamed group" => "Перейменовано групу",
        "Changed colour of" => "Змінено колір для",
        "Added person" => "Додано особу",
        "Deleted person" => "Видалено особу",
        "Renamed person" => "Перейменовано особу",
        "Changed group of" => "Змінено групу для",
        "Added stay" => "Додано перебування",
        "Removed stay" => "Видалено перебування",
        "Moved stay" => "Переміщено перебування",
        "Changed occupant of stay" => "Змінено мешканця перебування",
        "Changed dates of stay" => "Змінено дати перебування",
        "Loaded example data" => "Завантажено приклад даних",
        "Loaded plan from file" => "Відкрито план з файлу",
        "Loaded a plan with no change history" => "Відкрито план без історії змін",
        "Undid" => "Скасовано",
        // Files / close
        "💾 Save" => "💾 Зберегти",
        "Save As…" => "Зберегти як…",
        "Unsaved changes" => "Незбережені зміни",
        "You have unsaved changes. Save before closing?" => {
            "У вас є незбережені зміни. Зберегти перед закриттям?"
        }
        "Save" => "Зберегти",
        "Discard" => "Відхилити",
        "Cancel" => "Скасувати",
        "untitled" => "без назви",
        // Fallback: English
        other => other,
    }
}

/// German translations, keyed by the English source string. Unknown strings fall
/// back to English.
fn de(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Übersicht",
        "👥 Groups" => "👥 Gruppen",
        "🧍 Persons" => "🧍 Personen",
        "🏠 Housings" => "🏠 Unterkünfte",

        // Top bar
        "From:" => "Von:",
        "Days:" => "Tage:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Oder Strg/Cmd + Scrollen (Zwei-Finger-Zoom auf dem Trackpad) über der Zeitleiste"
        }
        "Today" => "Heute",
        "Fit to stays" => "An Aufenthalte anpassen",
        "📂 Load…" => "📂 Öffnen…",
        "ℹ About" => "ℹ Über",
        "Language" => "Sprache",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Gespeichert →",
        "Loaded ←" => "Geladen ←",
        "Save failed:" => "Speichern fehlgeschlagen:",
        "Encode failed:" => "Kodierung fehlgeschlagen:",
        "Read failed:" => "Lesen fehlgeschlagen:",
        "Parse failed:" => "Analyse fehlgeschlagen:",
        "File save is not available on Android yet." => {
            "Das Speichern in Dateien ist unter Android noch nicht verfügbar."
        }
        "File load is not available on Android yet." => {
            "Das Laden aus Dateien ist unter Android noch nicht verfügbar."
        }
        "Housing Planner plan" => "Housing-Planner-Plan",

        // About window
        "About / Licenses" => "Über / Lizenzen",
        "Version" => "Version",
        "Plan who stays where, and when." => "Planen Sie, wer wann wo wohnt.",
        "📋 Copy dependency licenses" => "📋 Abhängigkeitslizenzen kopieren",
        "This application" => "Diese Anwendung",
        "Third-party dependencies" => "Drittanbieter-Abhängigkeiten",

        // Overview tab
        "Welcome to Housing Planner" => "Willkommen bei Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Fügen Sie Unterkünfte, Gruppen und Personen in den Tabs oben hinzu —"
        }
        "📋 Load example data" => "📋 Beispieldaten laden",
        "Add a housing in the Housings tab to start planning." => {
            "Fügen Sie im Tab „Unterkünfte“ eine Unterkunft hinzu, um mit der Planung zu beginnen."
        }

        // Selectors / common
        "Group" => "Gruppe",
        "Person" => "Person",
        "Housing" => "Unterkunft",
        "➕ New" => "➕ Neu",
        "Stays:" => "Aufenthalte:",
        "Stays (individual):" => "Aufenthalte (einzeln):",
        "➕ Add stay" => "➕ Aufenthalt hinzufügen",
        "Add a housing and a person/group first." => {
            "Fügen Sie zuerst eine Unterkunft und eine Person/Gruppe hinzu."
        }
        "(no stays)" => "(keine Aufenthalte)",
        "(group)" => "(Gruppe)",

        // Groups tab
        "No groups yet — add one." => "Noch keine Gruppen — fügen Sie eine hinzu.",
        "🗑 Delete group" => "🗑 Gruppe löschen",
        "Members:" => "Mitglieder:",
        "(no members)" => "(keine Mitglieder)",
        "➕ Add existing…" => "➕ Vorhandene hinzufügen…",
        "➕ New person" => "➕ Neue Person",
        "Select or create a group." => "Wählen oder erstellen Sie eine Gruppe.",
        "No stays for this group yet." => "Noch keine Aufenthalte für diese Gruppe.",

        // Persons tab
        "No persons yet — add one." => "Noch keine Personen — fügen Sie eine hinzu.",
        "— no group —" => "— keine Gruppe —",
        "🗑 Delete person" => "🗑 Person löschen",
        "Select or create a person." => "Wählen oder erstellen Sie eine Person.",
        "No stays for this person yet." => "Noch keine Aufenthalte für diese Person.",

        // Housings tab
        "No housings yet — add one." => "Noch keine Unterkünfte — fügen Sie eine hinzu.",
        "Capacity" => "Kapazität",
        "Notes:" => "Notizen:",
        "🗑 Delete housing" => "🗑 Unterkunft löschen",
        "Select or create a housing." => "Wählen oder erstellen Sie eine Unterkunft.",
        "No stays in this housing yet." => "Noch keine Aufenthalte in dieser Unterkunft.",

        // Timeline
        "cap" => "Kap.",
        "To:" => "Bis:",
        "Nights:" => "Nächte:",
        "People:" => "Personen:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Gleichzeitig auch anderswo gebucht",
        "<deleted person>" => "<gelöschte Person>",
        "<deleted group>" => "<gelöschte Gruppe>",

        // Changelog
        "📜 Changelog" => "📜 Änderungsprotokoll",
        "↩ Undo last change" => "↩ Letzte Änderung rückgängig",
        "entries" => "Einträge",
        "No changes yet." => "Noch keine Änderungen.",
        "(no group)" => "(keine Gruppe)",
        "Created housing" => "Unterkunft erstellt",
        "Deleted housing" => "Unterkunft gelöscht",
        "Renamed housing" => "Unterkunft umbenannt",
        "Changed capacity of" => "Kapazität geändert von",
        "Edited notes of" => "Notizen bearbeitet von",
        "Created group" => "Gruppe erstellt",
        "Deleted group" => "Gruppe gelöscht",
        "Renamed group" => "Gruppe umbenannt",
        "Changed colour of" => "Farbe geändert von",
        "Added person" => "Person hinzugefügt",
        "Deleted person" => "Person gelöscht",
        "Renamed person" => "Person umbenannt",
        "Changed group of" => "Gruppe geändert von",
        "Added stay" => "Aufenthalt hinzugefügt",
        "Removed stay" => "Aufenthalt entfernt",
        "Moved stay" => "Aufenthalt verschoben",
        "Changed occupant of stay" => "Belegung des Aufenthalts geändert",
        "Changed dates of stay" => "Daten des Aufenthalts geändert",
        "Loaded example data" => "Beispieldaten geladen",
        "Loaded plan from file" => "Plan aus Datei geladen",
        "Loaded a plan with no change history" => "Plan ohne Änderungsverlauf geladen",
        "Undid" => "Rückgängig gemacht",
        // Files / close
        "💾 Save" => "💾 Speichern",
        "Save As…" => "Speichern unter…",
        "Unsaved changes" => "Ungespeicherte Änderungen",
        "You have unsaved changes. Save before closing?" => {
            "Sie haben ungespeicherte Änderungen. Vor dem Schließen speichern?"
        }
        "Save" => "Speichern",
        "Discard" => "Verwerfen",
        "Cancel" => "Abbrechen",
        "untitled" => "unbenannt",
        // Fallback: English
        other => other,
    }
}

/// French translations, keyed by the English source string. Unknown strings fall
/// back to English.
fn fr(en: &'static str) -> &'static str {
    match en {
        // Tabs
        "📊 Overview" => "📊 Vue d'ensemble",
        "👥 Groups" => "👥 Groupes",
        "🧍 Persons" => "🧍 Personnes",
        "🏠 Housings" => "🏠 Logements",

        // Top bar
        "From:" => "Du :",
        "Days:" => "Jours :",
        "Zoom:" => "Zoom :",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Ou Ctrl/Cmd + défilement (pincement sur le pavé tactile) au-dessus de la chronologie"
        }
        "Today" => "Aujourd'hui",
        "Fit to stays" => "Ajuster aux séjours",
        "📂 Load…" => "📂 Ouvrir…",
        "ℹ About" => "ℹ À propos",
        "Language" => "Langue",

        // Status messages (used as prefixes before a path / error)
        "Saved →" => "Enregistré →",
        "Loaded ←" => "Chargé ←",
        "Save failed:" => "Échec de l'enregistrement :",
        "Encode failed:" => "Échec de l'encodage :",
        "Read failed:" => "Échec de la lecture :",
        "Parse failed:" => "Échec de l'analyse :",
        "File save is not available on Android yet." => {
            "L'enregistrement de fichiers n'est pas encore disponible sur Android."
        }
        "File load is not available on Android yet." => {
            "Le chargement de fichiers n'est pas encore disponible sur Android."
        }
        "Housing Planner plan" => "Plan Housing Planner",

        // About window
        "About / Licenses" => "À propos / Licences",
        "Version" => "Version",
        "Plan who stays where, and when." => "Planifiez qui loge où, et quand.",
        "📋 Copy dependency licenses" => "📋 Copier les licences des dépendances",
        "This application" => "Cette application",
        "Third-party dependencies" => "Dépendances tierces",

        // Overview tab
        "Welcome to Housing Planner" => "Bienvenue dans Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Ajoutez des logements, des groupes et des personnes dans les onglets ci-dessus —"
        }
        "📋 Load example data" => "📋 Charger des données d'exemple",
        "Add a housing in the Housings tab to start planning." => {
            "Ajoutez un logement dans l'onglet Logements pour commencer à planifier."
        }

        // Selectors / common
        "Group" => "Groupe",
        "Person" => "Personne",
        "Housing" => "Logement",
        "➕ New" => "➕ Nouveau",
        "Stays:" => "Séjours :",
        "Stays (individual):" => "Séjours (individuels) :",
        "➕ Add stay" => "➕ Ajouter un séjour",
        "Add a housing and a person/group first." => {
            "Ajoutez d'abord un logement et une personne/un groupe."
        }
        "(no stays)" => "(aucun séjour)",
        "(group)" => "(groupe)",

        // Groups tab
        "No groups yet — add one." => "Aucun groupe pour l'instant — ajoutez-en un.",
        "🗑 Delete group" => "🗑 Supprimer le groupe",
        "Members:" => "Membres :",
        "(no members)" => "(aucun membre)",
        "➕ Add existing…" => "➕ Ajouter un existant…",
        "➕ New person" => "➕ Nouvelle personne",
        "Select or create a group." => "Sélectionnez ou créez un groupe.",
        "No stays for this group yet." => "Aucun séjour pour ce groupe pour l'instant.",

        // Persons tab
        "No persons yet — add one." => "Aucune personne pour l'instant — ajoutez-en une.",
        "— no group —" => "— aucun groupe —",
        "🗑 Delete person" => "🗑 Supprimer la personne",
        "Select or create a person." => "Sélectionnez ou créez une personne.",
        "No stays for this person yet." => "Aucun séjour pour cette personne pour l'instant.",

        // Housings tab
        "No housings yet — add one." => "Aucun logement pour l'instant — ajoutez-en un.",
        "Capacity" => "Capacité",
        "Notes:" => "Notes :",
        "🗑 Delete housing" => "🗑 Supprimer le logement",
        "Select or create a housing." => "Sélectionnez ou créez un logement.",
        "No stays in this housing yet." => "Aucun séjour dans ce logement pour l'instant.",

        // Timeline
        "cap" => "cap.",
        "To:" => "Au :",
        "Nights:" => "Nuits :",
        "People:" => "Personnes :",
        "⚠ Also booked elsewhere at the same time" => {
            "⚠ Également réservé ailleurs au même moment"
        }
        "<deleted person>" => "<personne supprimée>",
        "<deleted group>" => "<groupe supprimé>",

        // Changelog
        "📜 Changelog" => "📜 Journal des modifications",
        "↩ Undo last change" => "↩ Annuler la dernière modification",
        "entries" => "entrées",
        "No changes yet." => "Aucune modification pour l'instant.",
        "(no group)" => "(aucun groupe)",
        "Created housing" => "Logement créé",
        "Deleted housing" => "Logement supprimé",
        "Renamed housing" => "Logement renommé",
        "Changed capacity of" => "Capacité modifiée pour",
        "Edited notes of" => "Notes modifiées pour",
        "Created group" => "Groupe créé",
        "Deleted group" => "Groupe supprimé",
        "Renamed group" => "Groupe renommé",
        "Changed colour of" => "Couleur modifiée pour",
        "Added person" => "Personne ajoutée",
        "Deleted person" => "Personne supprimée",
        "Renamed person" => "Personne renommée",
        "Changed group of" => "Groupe modifié pour",
        "Added stay" => "Séjour ajouté",
        "Removed stay" => "Séjour supprimé",
        "Moved stay" => "Séjour déplacé",
        "Changed occupant of stay" => "Occupant du séjour modifié",
        "Changed dates of stay" => "Dates du séjour modifiées",
        "Loaded example data" => "Données d'exemple chargées",
        "Loaded plan from file" => "Plan chargé depuis un fichier",
        "Loaded a plan with no change history" => "Plan chargé sans historique des modifications",
        "Undid" => "Annulé",
        // Files / close
        "💾 Save" => "💾 Enregistrer",
        "Save As…" => "Enregistrer sous…",
        "Unsaved changes" => "Modifications non enregistrées",
        "You have unsaved changes. Save before closing?" => {
            "Vous avez des modifications non enregistrées. Enregistrer avant de fermer ?"
        }
        "Save" => "Enregistrer",
        "Discard" => "Abandonner",
        "Cancel" => "Annuler",
        "untitled" => "sans titre",
        // Fallback: English
        other => other,
    }
}

/// Italian translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn it(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Panoramica",
        "👥 Groups" => "👥 Gruppi",
        "🧍 Persons" => "🧍 Persone",
        "🏠 Housings" => "🏠 Alloggi",
        "From:" => "Da:",
        "Days:" => "Giorni:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Oppure Ctrl/Cmd + scorrimento (pizzica sul trackpad) sulla cronologia"
        }
        "Today" => "Oggi",
        "Fit to stays" => "Adatta ai soggiorni",
        "📂 Load…" => "📂 Apri…",
        "ℹ About" => "ℹ Informazioni",
        "Language" => "Lingua",
        "Saved →" => "Salvato →",
        "Loaded ←" => "Caricato ←",
        "Save failed:" => "Salvataggio non riuscito:",
        "Encode failed:" => "Codifica non riuscita:",
        "Read failed:" => "Lettura non riuscita:",
        "Parse failed:" => "Analisi non riuscita:",
        "File save is not available on Android yet." => {
            "Il salvataggio su file non è ancora disponibile su Android."
        }
        "File load is not available on Android yet." => {
            "Il caricamento da file non è ancora disponibile su Android."
        }
        "Housing Planner plan" => "Piano di Housing Planner",
        "About / Licenses" => "Informazioni / Licenze",
        "Version" => "Versione",
        "Plan who stays where, and when." => "Pianifica chi alloggia dove e quando.",
        "📋 Copy dependency licenses" => "📋 Copia le licenze delle dipendenze",
        "This application" => "Questa applicazione",
        "Third-party dependencies" => "Dipendenze di terze parti",
        "Welcome to Housing Planner" => "Benvenuto in Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Aggiungi alloggi, gruppi e persone nelle schede in alto —"
        }
        "📋 Load example data" => "📋 Carica dati di esempio",
        "Add a housing in the Housings tab to start planning." => {
            "Aggiungi un alloggio nella scheda Alloggi per iniziare a pianificare."
        }
        "Group" => "Gruppo",
        "Person" => "Persona",
        "Housing" => "Alloggio",
        "➕ New" => "➕ Nuovo",
        "Stays:" => "Soggiorni:",
        "Stays (individual):" => "Soggiorni (individuali):",
        "➕ Add stay" => "➕ Aggiungi soggiorno",
        "Add a housing and a person/group first." => {
            "Aggiungi prima un alloggio e una persona/un gruppo."
        }
        "(no stays)" => "(nessun soggiorno)",
        "(group)" => "(gruppo)",
        "No groups yet — add one." => "Ancora nessun gruppo — aggiungine uno.",
        "🗑 Delete group" => "🗑 Elimina gruppo",
        "Members:" => "Membri:",
        "(no members)" => "(nessun membro)",
        "➕ Add existing…" => "➕ Aggiungi esistente…",
        "➕ New person" => "➕ Nuova persona",
        "Select or create a group." => "Seleziona o crea un gruppo.",
        "No stays for this group yet." => "Ancora nessun soggiorno per questo gruppo.",
        "No persons yet — add one." => "Ancora nessuna persona — aggiungine una.",
        "— no group —" => "— nessun gruppo —",
        "🗑 Delete person" => "🗑 Elimina persona",
        "Select or create a person." => "Seleziona o crea una persona.",
        "No stays for this person yet." => "Ancora nessun soggiorno per questa persona.",
        "No housings yet — add one." => "Ancora nessun alloggio — aggiungine uno.",
        "Capacity" => "Capienza",
        "Notes:" => "Note:",
        "🗑 Delete housing" => "🗑 Elimina alloggio",
        "Select or create a housing." => "Seleziona o crea un alloggio.",
        "No stays in this housing yet." => "Ancora nessun soggiorno in questo alloggio.",
        "cap" => "cap",
        "To:" => "A:",
        "Nights:" => "Notti:",
        "People:" => "Persone:",
        "⚠ Also booked elsewhere at the same time" => {
            "⚠ Prenotato anche altrove nello stesso periodo"
        }
        "<deleted person>" => "<persona eliminata>",
        "<deleted group>" => "<gruppo eliminato>",
        "📜 Changelog" => "📜 Registro modifiche",
        "↩ Undo last change" => "↩ Annulla ultima modifica",
        "entries" => "voci",
        "No changes yet." => "Ancora nessuna modifica.",
        "(no group)" => "(nessun gruppo)",
        "Created housing" => "Alloggio creato",
        "Deleted housing" => "Alloggio eliminato",
        "Renamed housing" => "Alloggio rinominato",
        "Changed capacity of" => "Capienza modificata per",
        "Edited notes of" => "Note modificate per",
        "Created group" => "Gruppo creato",
        "Deleted group" => "Gruppo eliminato",
        "Renamed group" => "Gruppo rinominato",
        "Changed colour of" => "Colore modificato per",
        "Added person" => "Persona aggiunta",
        "Deleted person" => "Persona eliminata",
        "Renamed person" => "Persona rinominata",
        "Changed group of" => "Gruppo modificato per",
        "Added stay" => "Soggiorno aggiunto",
        "Removed stay" => "Soggiorno rimosso",
        "Moved stay" => "Soggiorno spostato",
        "Changed occupant of stay" => "Occupante del soggiorno modificato",
        "Changed dates of stay" => "Date del soggiorno modificate",
        "Loaded example data" => "Dati di esempio caricati",
        "Loaded plan from file" => "Piano caricato da file",
        "Loaded a plan with no change history" => {
            "Caricato un piano senza cronologia delle modifiche"
        }
        "Undid" => "Annullato",
        "💾 Save" => "💾 Salva",
        "Save As…" => "Salva con nome…",
        "Unsaved changes" => "Modifiche non salvate",
        "You have unsaved changes. Save before closing?" => {
            "Ci sono modifiche non salvate. Salvare prima di chiudere?"
        }
        "Save" => "Salva",
        "Discard" => "Scarta",
        "Cancel" => "Annulla",
        "untitled" => "senza nome",
        // Fallback: English
        other => other,
    }
}
