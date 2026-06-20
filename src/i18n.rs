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
    Spanish,
    Dutch,
    Russian,
    Icelandic,
    Greenlandic,
    Faroese,
    Greek,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::English
    }
}

impl Lang {
    /// All languages, for building a selector.
    pub const ALL: [Lang; 17] = [
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
        Lang::Spanish,
        Lang::Dutch,
        Lang::Russian,
        Lang::Icelandic,
        Lang::Greenlandic,
        Lang::Faroese,
        Lang::Greek,
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
            Lang::Spanish => "Español",
            Lang::Dutch => "Nederlands",
            Lang::Russian => "Русский",
            Lang::Icelandic => "Íslenska",
            Lang::Greenlandic => "Kalaallisut",
            Lang::Faroese => "Føroyskt",
            Lang::Greek => "Ελληνικά",
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
        } else if v.starts_with("es") {
            Lang::Spanish
        } else if v.starts_with("nl") {
            Lang::Dutch
        } else if v.starts_with("ru") {
            Lang::Russian
        } else if v.starts_with("is") {
            Lang::Icelandic
        } else if v.starts_with("kl") {
            Lang::Greenlandic
        } else if v.starts_with("fo") {
            Lang::Faroese
        } else if v.starts_with("el") {
            Lang::Greek
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
        Lang::Spanish => es(en),
        Lang::Dutch => nl(en),
        Lang::Russian => ru(en),
        Lang::Icelandic => isl(en),
        Lang::Greenlandic => kl(en),
        Lang::Faroese => fo(en),
        Lang::Greek => el(en),
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
        "File is open in another instance." => "Filen är öppen i en annan instans.",
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
        "File is open in another instance." => "Filen er åpen i en annen instans.",
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
        "File is open in another instance." => "Fila er open i ei anna instans.",
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
        "File is open in another instance." => "Fiila lea rabas eará instánssas.",
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
        "File is open in another instance." => "Filen er åben i en anden instans.",
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
        "File is open in another instance." => "Файл відкрито в іншому екземплярі.",
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
        "File is open in another instance." => "Die Datei ist in einer anderen Instanz geöffnet.",
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
        "File is open in another instance." => "Le fichier est ouvert dans une autre instance.",
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
        "File is open in another instance." => "Il file è aperto in un'altra istanza.",
        // Fallback: English
        other => other,
    }
}

/// Spanish translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn es(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Resumen",
        "👥 Groups" => "👥 Grupos",
        "🧍 Persons" => "🧍 Personas",
        "🏠 Housings" => "🏠 Alojamientos",
        "From:" => "Desde:",
        "Days:" => "Días:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "O Ctrl/Cmd + desplazamiento (pellizco en el panel táctil) sobre la línea de tiempo"
        }
        "Today" => "Hoy",
        "Fit to stays" => "Ajustar a las estancias",
        "📂 Load…" => "📂 Abrir…",
        "ℹ About" => "ℹ Acerca de",
        "Language" => "Idioma",
        "Saved →" => "Guardado →",
        "Loaded ←" => "Cargado ←",
        "Save failed:" => "Error al guardar:",
        "Encode failed:" => "Error al codificar:",
        "Read failed:" => "Error al leer:",
        "Parse failed:" => "Error al analizar:",
        "File save is not available on Android yet." => {
            "Guardar en archivo aún no está disponible en Android."
        }
        "File load is not available on Android yet." => {
            "Cargar desde archivo aún no está disponible en Android."
        }
        "Housing Planner plan" => "Plan de Housing Planner",
        "About / Licenses" => "Acerca de / Licencias",
        "Version" => "Versión",
        "Plan who stays where, and when." => "Planifica quién se aloja dónde y cuándo.",
        "📋 Copy dependency licenses" => "📋 Copiar licencias de dependencias",
        "This application" => "Esta aplicación",
        "Third-party dependencies" => "Dependencias de terceros",
        "Welcome to Housing Planner" => "Bienvenido a Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Añade alojamientos, grupos y personas en las pestañas de arriba —"
        }
        "📋 Load example data" => "📋 Cargar datos de ejemplo",
        "Add a housing in the Housings tab to start planning." => {
            "Añade un alojamiento en la pestaña Alojamientos para empezar a planificar."
        }
        "Group" => "Grupo",
        "Person" => "Persona",
        "Housing" => "Alojamiento",
        "➕ New" => "➕ Nuevo",
        "Stays:" => "Estancias:",
        "Stays (individual):" => "Estancias (individuales):",
        "➕ Add stay" => "➕ Añadir estancia",
        "Add a housing and a person/group first." => {
            "Añade primero un alojamiento y una persona/un grupo."
        }
        "(no stays)" => "(sin estancias)",
        "(group)" => "(grupo)",
        "No groups yet — add one." => "Aún no hay grupos — añade uno.",
        "🗑 Delete group" => "🗑 Eliminar grupo",
        "Members:" => "Miembros:",
        "(no members)" => "(sin miembros)",
        "➕ Add existing…" => "➕ Añadir existente…",
        "➕ New person" => "➕ Nueva persona",
        "Select or create a group." => "Selecciona o crea un grupo.",
        "No stays for this group yet." => "Aún no hay estancias para este grupo.",
        "No persons yet — add one." => "Aún no hay personas — añade una.",
        "— no group —" => "— sin grupo —",
        "🗑 Delete person" => "🗑 Eliminar persona",
        "Select or create a person." => "Selecciona o crea una persona.",
        "No stays for this person yet." => "Aún no hay estancias para esta persona.",
        "No housings yet — add one." => "Aún no hay alojamientos — añade uno.",
        "Capacity" => "Capacidad",
        "Notes:" => "Notas:",
        "🗑 Delete housing" => "🗑 Eliminar alojamiento",
        "Select or create a housing." => "Selecciona o crea un alojamiento.",
        "No stays in this housing yet." => "Aún no hay estancias en este alojamiento.",
        "cap" => "cap",
        "To:" => "Hasta:",
        "Nights:" => "Noches:",
        "People:" => "Personas:",
        "⚠ Also booked elsewhere at the same time" => {
            "⚠ También reservado en otro lugar al mismo tiempo"
        }
        "<deleted person>" => "<persona eliminada>",
        "<deleted group>" => "<grupo eliminado>",
        "📜 Changelog" => "📜 Registro de cambios",
        "↩ Undo last change" => "↩ Deshacer último cambio",
        "entries" => "entradas",
        "No changes yet." => "Aún no hay cambios.",
        "(no group)" => "(sin grupo)",
        "Created housing" => "Alojamiento creado",
        "Deleted housing" => "Alojamiento eliminado",
        "Renamed housing" => "Alojamiento renombrado",
        "Changed capacity of" => "Capacidad modificada de",
        "Edited notes of" => "Notas editadas de",
        "Created group" => "Grupo creado",
        "Deleted group" => "Grupo eliminado",
        "Renamed group" => "Grupo renombrado",
        "Changed colour of" => "Color modificado de",
        "Added person" => "Persona añadida",
        "Deleted person" => "Persona eliminada",
        "Renamed person" => "Persona renombrada",
        "Changed group of" => "Grupo modificado de",
        "Added stay" => "Estancia añadida",
        "Removed stay" => "Estancia eliminada",
        "Moved stay" => "Estancia movida",
        "Changed occupant of stay" => "Ocupante de la estancia modificado",
        "Changed dates of stay" => "Fechas de la estancia modificadas",
        "Loaded example data" => "Datos de ejemplo cargados",
        "Loaded plan from file" => "Plan cargado desde archivo",
        "Loaded a plan with no change history" => "Se cargó un plan sin historial de cambios",
        "Undid" => "Deshecho",
        "💾 Save" => "💾 Guardar",
        "Save As…" => "Guardar como…",
        "Unsaved changes" => "Cambios sin guardar",
        "You have unsaved changes. Save before closing?" => {
            "Tienes cambios sin guardar. ¿Guardar antes de cerrar?"
        }
        "Save" => "Guardar",
        "Discard" => "Descartar",
        "Cancel" => "Cancelar",
        "untitled" => "sin título",
        "File is open in another instance." => "El archivo está abierto en otra instancia.",
        // Fallback: English
        other => other,
    }
}

/// Dutch translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn nl(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Overzicht",
        "👥 Groups" => "👥 Groepen",
        "🧍 Persons" => "🧍 Personen",
        "🏠 Housings" => "🏠 Accommodaties",
        "From:" => "Van:",
        "Days:" => "Dagen:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Of Ctrl/Cmd + scrollen (knijpen op trackpad) over de tijdlijn"
        }
        "Today" => "Vandaag",
        "Fit to stays" => "Aanpassen aan verblijven",
        "📂 Load…" => "📂 Openen…",
        "ℹ About" => "ℹ Over",
        "Language" => "Taal",
        "Saved →" => "Opgeslagen →",
        "Loaded ←" => "Geladen ←",
        "Save failed:" => "Opslaan mislukt:",
        "Encode failed:" => "Coderen mislukt:",
        "Read failed:" => "Lezen mislukt:",
        "Parse failed:" => "Verwerken mislukt:",
        "File save is not available on Android yet." => {
            "Opslaan naar bestand is nog niet beschikbaar op Android."
        }
        "File load is not available on Android yet." => {
            "Laden uit bestand is nog niet beschikbaar op Android."
        }
        "Housing Planner plan" => "Housing Planner-plan",
        "About / Licenses" => "Over / Licenties",
        "Version" => "Versie",
        "Plan who stays where, and when." => "Plan wie waar verblijft, en wanneer.",
        "📋 Copy dependency licenses" => "📋 Afhankelijkheidslicenties kopiëren",
        "This application" => "Deze applicatie",
        "Third-party dependencies" => "Afhankelijkheden van derden",
        "Welcome to Housing Planner" => "Welkom bij Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Voeg accommodaties, groepen en personen toe in de tabbladen hierboven —"
        }
        "📋 Load example data" => "📋 Voorbeeldgegevens laden",
        "Add a housing in the Housings tab to start planning." => {
            "Voeg een accommodatie toe op het tabblad Accommodaties om te beginnen met plannen."
        }
        "Group" => "Groep",
        "Person" => "Persoon",
        "Housing" => "Accommodatie",
        "➕ New" => "➕ Nieuw",
        "Stays:" => "Verblijven:",
        "Stays (individual):" => "Verblijven (individueel):",
        "➕ Add stay" => "➕ Verblijf toevoegen",
        "Add a housing and a person/group first." => {
            "Voeg eerst een accommodatie en een persoon/groep toe."
        }
        "(no stays)" => "(geen verblijven)",
        "(group)" => "(groep)",
        "No groups yet — add one." => "Nog geen groepen — voeg er een toe.",
        "🗑 Delete group" => "🗑 Groep verwijderen",
        "Members:" => "Leden:",
        "(no members)" => "(geen leden)",
        "➕ Add existing…" => "➕ Bestaande toevoegen…",
        "➕ New person" => "➕ Nieuwe persoon",
        "Select or create a group." => "Selecteer of maak een groep.",
        "No stays for this group yet." => "Nog geen verblijven voor deze groep.",
        "No persons yet — add one." => "Nog geen personen — voeg er een toe.",
        "— no group —" => "— geen groep —",
        "🗑 Delete person" => "🗑 Persoon verwijderen",
        "Select or create a person." => "Selecteer of maak een persoon.",
        "No stays for this person yet." => "Nog geen verblijven voor deze persoon.",
        "No housings yet — add one." => "Nog geen accommodaties — voeg er een toe.",
        "Capacity" => "Capaciteit",
        "Notes:" => "Notities:",
        "🗑 Delete housing" => "🗑 Accommodatie verwijderen",
        "Select or create a housing." => "Selecteer of maak een accommodatie.",
        "No stays in this housing yet." => "Nog geen verblijven in deze accommodatie.",
        "cap" => "cap",
        "To:" => "Tot:",
        "Nights:" => "Nachten:",
        "People:" => "Personen:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Tegelijkertijd ook elders geboekt",
        "<deleted person>" => "<verwijderde persoon>",
        "<deleted group>" => "<verwijderde groep>",
        "📜 Changelog" => "📜 Wijzigingslogboek",
        "↩ Undo last change" => "↩ Laatste wijziging ongedaan maken",
        "entries" => "items",
        "No changes yet." => "Nog geen wijzigingen.",
        "(no group)" => "(geen groep)",
        "Created housing" => "Accommodatie aangemaakt",
        "Deleted housing" => "Accommodatie verwijderd",
        "Renamed housing" => "Accommodatie hernoemd",
        "Changed capacity of" => "Capaciteit gewijzigd van",
        "Edited notes of" => "Notities bewerkt van",
        "Created group" => "Groep aangemaakt",
        "Deleted group" => "Groep verwijderd",
        "Renamed group" => "Groep hernoemd",
        "Changed colour of" => "Kleur gewijzigd van",
        "Added person" => "Persoon toegevoegd",
        "Deleted person" => "Persoon verwijderd",
        "Renamed person" => "Persoon hernoemd",
        "Changed group of" => "Groep gewijzigd van",
        "Added stay" => "Verblijf toegevoegd",
        "Removed stay" => "Verblijf verwijderd",
        "Moved stay" => "Verblijf verplaatst",
        "Changed occupant of stay" => "Bewoner van verblijf gewijzigd",
        "Changed dates of stay" => "Datums van verblijf gewijzigd",
        "Loaded example data" => "Voorbeeldgegevens geladen",
        "Loaded plan from file" => "Plan geladen uit bestand",
        "Loaded a plan with no change history" => "Een plan zonder wijzigingsgeschiedenis geladen",
        "Undid" => "Ongedaan gemaakt",
        "💾 Save" => "💾 Opslaan",
        "Save As…" => "Opslaan als…",
        "Unsaved changes" => "Niet-opgeslagen wijzigingen",
        "You have unsaved changes. Save before closing?" => {
            "Je hebt niet-opgeslagen wijzigingen. Opslaan voordat je sluit?"
        }
        "Save" => "Opslaan",
        "Discard" => "Verwerpen",
        "Cancel" => "Annuleren",
        "untitled" => "naamloos",
        "File is open in another instance." => "Het bestand is geopend in een ander exemplaar.",
        // Fallback: English
        other => other,
    }
}

/// Russian translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn ru(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Обзор",
        "👥 Groups" => "👥 Группы",
        "🧍 Persons" => "🧍 Люди",
        "🏠 Housings" => "🏠 Жильё",
        "From:" => "С:",
        "Days:" => "Дни:",
        "Zoom:" => "Масштаб:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Или Ctrl/Cmd + прокрутка (щипок на тачпаде) над шкалой времени"
        }
        "Today" => "Сегодня",
        "Fit to stays" => "Подогнать под проживания",
        "📂 Load…" => "📂 Открыть…",
        "ℹ About" => "ℹ О программе",
        "Language" => "Язык",
        "Saved →" => "Сохранено →",
        "Loaded ←" => "Загружено ←",
        "Save failed:" => "Не удалось сохранить:",
        "Encode failed:" => "Не удалось закодировать:",
        "Read failed:" => "Не удалось прочитать:",
        "Parse failed:" => "Не удалось разобрать:",
        "File save is not available on Android yet." => {
            "Сохранение в файл пока недоступно на Android."
        }
        "File load is not available on Android yet." => {
            "Загрузка из файла пока недоступна на Android."
        }
        "Housing Planner plan" => "План Housing Planner",
        "About / Licenses" => "О программе / Лицензии",
        "Version" => "Версия",
        "Plan who stays where, and when." => "Планируйте, кто где живёт и когда.",
        "📋 Copy dependency licenses" => "📋 Скопировать лицензии зависимостей",
        "This application" => "Это приложение",
        "Third-party dependencies" => "Сторонние зависимости",
        "Welcome to Housing Planner" => "Добро пожаловать в Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Добавляйте жильё, группы и людей на вкладках выше —"
        }
        "📋 Load example data" => "📋 Загрузить пример данных",
        "Add a housing in the Housings tab to start planning." => {
            "Добавьте жильё на вкладке «Жильё», чтобы начать планирование."
        }
        "Group" => "Группа",
        "Person" => "Человек",
        "Housing" => "Жильё",
        "➕ New" => "➕ Создать",
        "Stays:" => "Проживания:",
        "Stays (individual):" => "Проживания (индивидуальные):",
        "➕ Add stay" => "➕ Добавить проживание",
        "Add a housing and a person/group first." => "Сначала добавьте жильё и человека/группу.",
        "(no stays)" => "(нет проживаний)",
        "(group)" => "(группа)",
        "No groups yet — add one." => "Групп пока нет — добавьте одну.",
        "🗑 Delete group" => "🗑 Удалить группу",
        "Members:" => "Участники:",
        "(no members)" => "(нет участников)",
        "➕ Add existing…" => "➕ Добавить существующего…",
        "➕ New person" => "➕ Новый человек",
        "Select or create a group." => "Выберите или создайте группу.",
        "No stays for this group yet." => "Для этой группы пока нет проживаний.",
        "No persons yet — add one." => "Людей пока нет — добавьте одного.",
        "— no group —" => "— без группы —",
        "🗑 Delete person" => "🗑 Удалить человека",
        "Select or create a person." => "Выберите или создайте человека.",
        "No stays for this person yet." => "Для этого человека пока нет проживаний.",
        "No housings yet — add one." => "Жилья пока нет — добавьте.",
        "Capacity" => "Вместимость",
        "Notes:" => "Заметки:",
        "🗑 Delete housing" => "🗑 Удалить жильё",
        "Select or create a housing." => "Выберите или создайте жильё.",
        "No stays in this housing yet." => "В этом жилье пока нет проживаний.",
        "cap" => "вмест.",
        "To:" => "По:",
        "Nights:" => "Ночей:",
        "People:" => "Людей:",
        "⚠ Also booked elsewhere at the same time" => {
            "⚠ Также забронировано в другом месте в это же время"
        }
        "<deleted person>" => "<удалённый человек>",
        "<deleted group>" => "<удалённая группа>",
        "📜 Changelog" => "📜 Журнал изменений",
        "↩ Undo last change" => "↩ Отменить последнее изменение",
        "entries" => "записей",
        "No changes yet." => "Изменений пока нет.",
        "(no group)" => "(без группы)",
        "Created housing" => "Создано жильё",
        "Deleted housing" => "Удалено жильё",
        "Renamed housing" => "Переименовано жильё",
        "Changed capacity of" => "Изменена вместимость для",
        "Edited notes of" => "Отредактированы заметки для",
        "Created group" => "Создана группа",
        "Deleted group" => "Удалена группа",
        "Renamed group" => "Переименована группа",
        "Changed colour of" => "Изменён цвет для",
        "Added person" => "Добавлен человек",
        "Deleted person" => "Удалён человек",
        "Renamed person" => "Переименован человек",
        "Changed group of" => "Изменена группа для",
        "Added stay" => "Добавлено проживание",
        "Removed stay" => "Удалено проживание",
        "Moved stay" => "Перемещено проживание",
        "Changed occupant of stay" => "Изменён жилец проживания",
        "Changed dates of stay" => "Изменены даты проживания",
        "Loaded example data" => "Загружен пример данных",
        "Loaded plan from file" => "План загружен из файла",
        "Loaded a plan with no change history" => "Загружен план без истории изменений",
        "Undid" => "Отменено",
        "💾 Save" => "💾 Сохранить",
        "Save As…" => "Сохранить как…",
        "Unsaved changes" => "Несохранённые изменения",
        "You have unsaved changes. Save before closing?" => {
            "У вас есть несохранённые изменения. Сохранить перед закрытием?"
        }
        "Save" => "Сохранить",
        "Discard" => "Не сохранять",
        "Cancel" => "Отмена",
        "untitled" => "без названия",
        "File is open in another instance." => "Файл открыт в другом экземпляре.",
        // Fallback: English
        other => other,
    }
}

/// Icelandic translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn isl(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Yfirlit",
        "👥 Groups" => "👥 Hópar",
        "🧍 Persons" => "🧍 Einstaklingar",
        "🏠 Housings" => "🏠 Húsnæði",
        "From:" => "Frá:",
        "Days:" => "Dagar:",
        "Zoom:" => "Aðdráttur:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Eða Ctrl/Cmd + skrun (klíptu á snertiplatta) yfir tímalínuna"
        }
        "Today" => "Í dag",
        "Fit to stays" => "Laga að dvölum",
        "📂 Load…" => "📂 Opna…",
        "ℹ About" => "ℹ Um",
        "Language" => "Tungumál",
        "Saved →" => "Vistað →",
        "Loaded ←" => "Hlaðið ←",
        "Save failed:" => "Vistun mistókst:",
        "Encode failed:" => "Kóðun mistókst:",
        "Read failed:" => "Lestur mistókst:",
        "Parse failed:" => "Þáttun mistókst:",
        "File save is not available on Android yet." => {
            "Vistun í skrá er ekki enn í boði á Android."
        }
        "File load is not available on Android yet." => {
            "Hleðsla úr skrá er ekki enn í boði á Android."
        }
        "Housing Planner plan" => "Housing Planner-áætlun",
        "About / Licenses" => "Um / Leyfi",
        "Version" => "Útgáfa",
        "Plan who stays where, and when." => "Skipuleggðu hver dvelur hvar og hvenær.",
        "📋 Copy dependency licenses" => "📋 Afrita leyfi háðra pakka",
        "This application" => "Þetta forrit",
        "Third-party dependencies" => "Pakkar þriðja aðila",
        "Welcome to Housing Planner" => "Velkomin í Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Bættu við húsnæði, hópum og fólki í flipunum að ofan —"
        }
        "📋 Load example data" => "📋 Hlaða dæmigögnum",
        "Add a housing in the Housings tab to start planning." => {
            "Bættu við húsnæði í Húsnæði-flipanum til að byrja að skipuleggja."
        }
        "Group" => "Hópur",
        "Person" => "Einstaklingur",
        "Housing" => "Húsnæði",
        "➕ New" => "➕ Nýtt",
        "Stays:" => "Dvalir:",
        "Stays (individual):" => "Dvalir (einstakar):",
        "➕ Add stay" => "➕ Bæta við dvöl",
        "Add a housing and a person/group first." => {
            "Bættu fyrst við húsnæði og einstaklingi/hópi."
        }
        "(no stays)" => "(engar dvalir)",
        "(group)" => "(hópur)",
        "No groups yet — add one." => "Engir hópar enn — bættu við einum.",
        "🗑 Delete group" => "🗑 Eyða hópi",
        "Members:" => "Meðlimir:",
        "(no members)" => "(engir meðlimir)",
        "➕ Add existing…" => "➕ Bæta við fyrirliggjandi…",
        "➕ New person" => "➕ Nýr einstaklingur",
        "Select or create a group." => "Veldu eða búðu til hóp.",
        "No stays for this group yet." => "Engar dvalir fyrir þennan hóp enn.",
        "No persons yet — add one." => "Engir einstaklingar enn — bættu við einum.",
        "— no group —" => "— enginn hópur —",
        "🗑 Delete person" => "🗑 Eyða einstaklingi",
        "Select or create a person." => "Veldu eða búðu til einstakling.",
        "No stays for this person yet." => "Engar dvalir fyrir þennan einstakling enn.",
        "No housings yet — add one." => "Ekkert húsnæði enn — bættu við einu.",
        "Capacity" => "Rými",
        "Notes:" => "Athugasemdir:",
        "🗑 Delete housing" => "🗑 Eyða húsnæði",
        "Select or create a housing." => "Veldu eða búðu til húsnæði.",
        "No stays in this housing yet." => "Engar dvalir í þessu húsnæði enn.",
        "cap" => "rými",
        "To:" => "Til:",
        "Nights:" => "Nætur:",
        "People:" => "Fólk:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Einnig bókað annars staðar á sama tíma",
        "<deleted person>" => "<eyddur einstaklingur>",
        "<deleted group>" => "<eyddur hópur>",
        "📜 Changelog" => "📜 Breytingaskrá",
        "↩ Undo last change" => "↩ Afturkalla síðustu breytingu",
        "entries" => "færslur",
        "No changes yet." => "Engar breytingar enn.",
        "(no group)" => "(enginn hópur)",
        "Created housing" => "Húsnæði búið til",
        "Deleted housing" => "Húsnæði eytt",
        "Renamed housing" => "Húsnæði endurnefnt",
        "Changed capacity of" => "Rými breytt fyrir",
        "Edited notes of" => "Athugasemdir breyttar fyrir",
        "Created group" => "Hópur búinn til",
        "Deleted group" => "Hópi eytt",
        "Renamed group" => "Hópur endurnefndur",
        "Changed colour of" => "Litur breyttur fyrir",
        "Added person" => "Einstaklingi bætt við",
        "Deleted person" => "Einstaklingi eytt",
        "Renamed person" => "Einstaklingur endurnefndur",
        "Changed group of" => "Hópi breytt fyrir",
        "Added stay" => "Dvöl bætt við",
        "Removed stay" => "Dvöl fjarlægð",
        "Moved stay" => "Dvöl færð",
        "Changed occupant of stay" => "Íbúa dvalar breytt",
        "Changed dates of stay" => "Dagsetningum dvalar breytt",
        "Loaded example data" => "Dæmigögn hlaðin",
        "Loaded plan from file" => "Áætlun hlaðin úr skrá",
        "Loaded a plan with no change history" => "Hlóð áætlun án breytingasögu",
        "Undid" => "Afturkallað",
        "💾 Save" => "💾 Vista",
        "Save As…" => "Vista sem…",
        "Unsaved changes" => "Óvistaðar breytingar",
        "You have unsaved changes. Save before closing?" => {
            "Þú ert með óvistaðar breytingar. Vista áður en lokað er?"
        }
        "Save" => "Vista",
        "Discard" => "Henda",
        "Cancel" => "Hætta við",
        "untitled" => "ónefnt",
        "File is open in another instance." => "Skráin er opin í öðru tilviki.",
        // Fallback: English
        other => other,
    }
}

/// Greenlandic (Kalaallisut) translations.
///
/// STUB / best-effort: only a few high-confidence strings are translated; the
/// rest intentionally equal English. Needs review/completion by a Kalaallisut
/// speaker.
fn kl(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Overview",
        "👥 Groups" => "👥 Groups",
        "🧍 Persons" => "🧍 Inuit",
        "🏠 Housings" => "🏠 Illut",
        "From:" => "From:",
        "Days:" => "Ullut:",
        "Zoom:" => "Zoom:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline"
        }
        "Today" => "Ullumi",
        "Fit to stays" => "Fit to stays",
        "📂 Load…" => "📂 Load…",
        "ℹ About" => "ℹ Pillugu",
        "Language" => "Oqaatsit",
        "Saved →" => "Saved →",
        "Loaded ←" => "Loaded ←",
        "Save failed:" => "Save failed:",
        "Encode failed:" => "Encode failed:",
        "Read failed:" => "Read failed:",
        "Parse failed:" => "Parse failed:",
        "File save is not available on Android yet." => {
            "File save is not available on Android yet."
        }
        "File load is not available on Android yet." => {
            "File load is not available on Android yet."
        }
        "Housing Planner plan" => "Housing Planner plan",
        "About / Licenses" => "About / Licenses",
        "Version" => "Version",
        "Plan who stays where, and when." => "Plan who stays where, and when.",
        "📋 Copy dependency licenses" => "📋 Copy dependency licenses",
        "This application" => "This application",
        "Third-party dependencies" => "Third-party dependencies",
        "Welcome to Housing Planner" => "Tikilluarit Housing Planner-mut",
        "Add housings, groups and people in the tabs above —" => {
            "Add housings, groups and people in the tabs above —"
        }
        "📋 Load example data" => "📋 Load example data",
        "Add a housing in the Housings tab to start planning." => {
            "Add a housing in the Housings tab to start planning."
        }
        "Group" => "Group",
        "Person" => "Inuk",
        "Housing" => "Illu",
        "➕ New" => "➕ New",
        "Stays:" => "Stays:",
        "Stays (individual):" => "Stays (individual):",
        "➕ Add stay" => "➕ Add stay",
        "Add a housing and a person/group first." => "Add a housing and a person/group first.",
        "(no stays)" => "(no stays)",
        "(group)" => "(group)",
        "No groups yet — add one." => "No groups yet — add one.",
        "🗑 Delete group" => "🗑 Delete group",
        "Members:" => "Members:",
        "(no members)" => "(no members)",
        "➕ Add existing…" => "➕ Add existing…",
        "➕ New person" => "➕ New person",
        "Select or create a group." => "Select or create a group.",
        "No stays for this group yet." => "No stays for this group yet.",
        "No persons yet — add one." => "No persons yet — add one.",
        "— no group —" => "— no group —",
        "🗑 Delete person" => "🗑 Delete person",
        "Select or create a person." => "Select or create a person.",
        "No stays for this person yet." => "No stays for this person yet.",
        "No housings yet — add one." => "No housings yet — add one.",
        "Capacity" => "Capacity",
        "Notes:" => "Notes:",
        "🗑 Delete housing" => "🗑 Delete housing",
        "Select or create a housing." => "Select or create a housing.",
        "No stays in this housing yet." => "No stays in this housing yet.",
        "cap" => "cap",
        "To:" => "To:",
        "Nights:" => "Unnuat:",
        "People:" => "Inuit:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Also booked elsewhere at the same time",
        "<deleted person>" => "<deleted person>",
        "<deleted group>" => "<deleted group>",
        "📜 Changelog" => "📜 Changelog",
        "↩ Undo last change" => "↩ Undo last change",
        "entries" => "entries",
        "No changes yet." => "No changes yet.",
        "(no group)" => "(no group)",
        "Created housing" => "Created housing",
        "Deleted housing" => "Deleted housing",
        "Renamed housing" => "Renamed housing",
        "Changed capacity of" => "Changed capacity of",
        "Edited notes of" => "Edited notes of",
        "Created group" => "Created group",
        "Deleted group" => "Deleted group",
        "Renamed group" => "Renamed group",
        "Changed colour of" => "Changed colour of",
        "Added person" => "Added person",
        "Deleted person" => "Deleted person",
        "Renamed person" => "Renamed person",
        "Changed group of" => "Changed group of",
        "Added stay" => "Added stay",
        "Removed stay" => "Removed stay",
        "Moved stay" => "Moved stay",
        "Changed occupant of stay" => "Changed occupant of stay",
        "Changed dates of stay" => "Changed dates of stay",
        "Loaded example data" => "Loaded example data",
        "Loaded plan from file" => "Loaded plan from file",
        "Loaded a plan with no change history" => "Loaded a plan with no change history",
        "Undid" => "Undid",
        "💾 Save" => "💾 Save",
        "Save As…" => "Save As…",
        "Unsaved changes" => "Unsaved changes",
        "You have unsaved changes. Save before closing?" => {
            "You have unsaved changes. Save before closing?"
        }
        "Save" => "Save",
        "Discard" => "Discard",
        "Cancel" => "Cancel",
        "untitled" => "untitled",
        "File is open in another instance." => "File is open in another instance.",
        // Fallback: English
        other => other,
    }
}

/// Faroese (Føroyskt) translations, keyed by the English source string.
/// Best-effort; should be reviewed by a fluent speaker. Unknown strings fall
/// back to English.
fn fo(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Yvirlit",
        "👥 Groups" => "👥 Bólkar",
        "🧍 Persons" => "🧍 Persónar",
        "🏠 Housings" => "🏠 Bústaðir",
        "From:" => "Frá:",
        "Days:" => "Dagar:",
        "Zoom:" => "Atdráttur:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Ella Ctrl/Cmd + rulling (kníp á flatuni) yvir tíðarlinjuni"
        }
        "Today" => "Í dag",
        "Fit to stays" => "Tillaga til uppihald",
        "📂 Load…" => "📂 Opna…",
        "ℹ About" => "ℹ Um",
        "Language" => "Mál",
        "Saved →" => "Goymt →",
        "Loaded ←" => "Innlisið ←",
        "Save failed:" => "Goyming miseydnaðist:",
        "Encode failed:" => "Koding miseydnaðist:",
        "Read failed:" => "Lesing miseydnaðist:",
        "Parse failed:" => "Tulking miseydnaðist:",
        "File save is not available on Android yet." => {
            "Goyming til fílu er enn ikki tøk á Android."
        }
        "File load is not available on Android yet." => {
            "Innlesing úr fílu er enn ikki tøk á Android."
        }
        "Housing Planner plan" => "Housing Planner-ætlan",
        "About / Licenses" => "Um / Loyvir",
        "Version" => "Útgáva",
        "Plan who stays where, and when." => "Ætla hvør býr hvar, og nær.",
        "📋 Copy dependency licenses" => "📋 Avrita loyvir fyri avhongdar pakkar",
        "This application" => "Hetta forritið",
        "Third-party dependencies" => "Triðjaparts-avhongd",
        "Welcome to Housing Planner" => "Vælkomin til Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Legg afturat bústøðum, bólkum og fólki í teigunum omanfyri —"
        }
        "📋 Load example data" => "📋 Innles dømisdátur",
        "Add a housing in the Housings tab to start planning." => {
            "Legg ein bústað afturat í Bústaðir-teiginum fyri at byrja at ætla."
        }
        "Group" => "Bólkur",
        "Person" => "Persónur",
        "Housing" => "Bústaður",
        "➕ New" => "➕ Nýtt",
        "Stays:" => "Uppihøld:",
        "Stays (individual):" => "Uppihøld (einstøk):",
        "➕ Add stay" => "➕ Legg uppihald afturat",
        "Add a housing and a person/group first." => {
            "Legg fyrst ein bústað og ein persón/bólk afturat."
        }
        "(no stays)" => "(eingi uppihøld)",
        "(group)" => "(bólkur)",
        "No groups yet — add one." => "Eingir bólkar enn — legg ein afturat.",
        "🗑 Delete group" => "🗑 Strika bólk",
        "Members:" => "Limir:",
        "(no members)" => "(eingir limir)",
        "➕ Add existing…" => "➕ Legg verandi afturat…",
        "➕ New person" => "➕ Nýggjur persónur",
        "Select or create a group." => "Vel ella stovna ein bólk.",
        "No stays for this group yet." => "Eingi uppihøld fyri henda bólk enn.",
        "No persons yet — add one." => "Eingir persónar enn — legg ein afturat.",
        "— no group —" => "— eingin bólkur —",
        "🗑 Delete person" => "🗑 Strika persón",
        "Select or create a person." => "Vel ella stovna ein persón.",
        "No stays for this person yet." => "Eingi uppihøld fyri henda persón enn.",
        "No housings yet — add one." => "Eingir bústaðir enn — legg ein afturat.",
        "Capacity" => "Pláss",
        "Notes:" => "Viðmerkingar:",
        "🗑 Delete housing" => "🗑 Strika bústað",
        "Select or create a housing." => "Vel ella stovna ein bústað.",
        "No stays in this housing yet." => "Eingi uppihøld í hesum bústaði enn.",
        "cap" => "pláss",
        "To:" => "Til:",
        "Nights:" => "Nætur:",
        "People:" => "Fólk:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Eisini bílagt aðrastaðni á sama tíð",
        "<deleted person>" => "<strikaður persónur>",
        "<deleted group>" => "<strikaður bólkur>",
        "📜 Changelog" => "📜 Broytingarlogg",
        "↩ Undo last change" => "↩ Angra seinastu broyting",
        "entries" => "innføringar",
        "No changes yet." => "Ongar broytingar enn.",
        "(no group)" => "(eingin bólkur)",
        "Created housing" => "Bústaður stovnaður",
        "Deleted housing" => "Bústaður strikaður",
        "Renamed housing" => "Bústaður umnevndur",
        "Changed capacity of" => "Pláss broytt fyri",
        "Edited notes of" => "Viðmerkingar broyttar fyri",
        "Created group" => "Bólkur stovnaður",
        "Deleted group" => "Bólkur strikaður",
        "Renamed group" => "Bólkur umnevndur",
        "Changed colour of" => "Litur broyttur fyri",
        "Added person" => "Persónur lagdur afturat",
        "Deleted person" => "Persónur strikaður",
        "Renamed person" => "Persónur umnevndur",
        "Changed group of" => "Bólkur broyttur fyri",
        "Added stay" => "Uppihald lagt afturat",
        "Removed stay" => "Uppihald strikað",
        "Moved stay" => "Uppihald flutt",
        "Changed occupant of stay" => "Íbúgvi í uppihaldi broyttur",
        "Changed dates of stay" => "Dagfestingar í uppihaldi broyttar",
        "Loaded example data" => "Dømisdátur innlisnar",
        "Loaded plan from file" => "Ætlan innlisin úr fílu",
        "Loaded a plan with no change history" => "Innlas eina ætlan uttan broytingarsøgu",
        "Undid" => "Angrað",
        "💾 Save" => "💾 Goym",
        "Save As…" => "Goym sum…",
        "Unsaved changes" => "Ógoymdar broytingar",
        "You have unsaved changes. Save before closing?" => {
            "Tú hevur ógoymdar broytingar. Goyma áðrenn tú letur aftur?"
        }
        "Save" => "Goym",
        "Discard" => "Vraka",
        "Cancel" => "Angra",
        "untitled" => "ónevnt",
        "File is open in another instance." => "Fílan er opin í øðrum tilviki.",
        // Fallback: English
        other => other,
    }
}

/// Greek translations, keyed by the English source string. Unknown strings
/// fall back to English.
fn el(en: &'static str) -> &'static str {
    match en {
        "📊 Overview" => "📊 Επισκόπηση",
        "👥 Groups" => "👥 Ομάδες",
        "🧍 Persons" => "🧍 Άτομα",
        "🏠 Housings" => "🏠 Καταλύματα",
        "From:" => "Από:",
        "Days:" => "Ημέρες:",
        "Zoom:" => "Ζουμ:",
        "Or Ctrl/Cmd + scroll (pinch on trackpad) over the timeline" => {
            "Ή Ctrl/Cmd + κύλιση (τσίμπημα στην επιφάνεια αφής) πάνω από τη γραμμή χρόνου"
        }
        "Today" => "Σήμερα",
        "Fit to stays" => "Προσαρμογή στις διαμονές",
        "📂 Load…" => "📂 Άνοιγμα…",
        "ℹ About" => "ℹ Σχετικά",
        "Language" => "Γλώσσα",
        "Saved →" => "Αποθηκεύτηκε →",
        "Loaded ←" => "Φορτώθηκε ←",
        "Save failed:" => "Η αποθήκευση απέτυχε:",
        "Encode failed:" => "Η κωδικοποίηση απέτυχε:",
        "Read failed:" => "Η ανάγνωση απέτυχε:",
        "Parse failed:" => "Η ανάλυση απέτυχε:",
        "File save is not available on Android yet." => {
            "Η αποθήκευση σε αρχείο δεν είναι ακόμη διαθέσιμη στο Android."
        }
        "File load is not available on Android yet." => {
            "Η φόρτωση από αρχείο δεν είναι ακόμη διαθέσιμη στο Android."
        }
        "Housing Planner plan" => "Σχέδιο Housing Planner",
        "About / Licenses" => "Σχετικά / Άδειες",
        "Version" => "Έκδοση",
        "Plan who stays where, and when." => "Σχεδιάστε ποιος μένει πού και πότε.",
        "📋 Copy dependency licenses" => "📋 Αντιγραφή αδειών εξαρτήσεων",
        "This application" => "Αυτή η εφαρμογή",
        "Third-party dependencies" => "Εξαρτήσεις τρίτων",
        "Welcome to Housing Planner" => "Καλώς ορίσατε στο Housing Planner",
        "Add housings, groups and people in the tabs above —" => {
            "Προσθέστε καταλύματα, ομάδες και άτομα στις παραπάνω καρτέλες —"
        }
        "📋 Load example data" => "📋 Φόρτωση δεδομένων παραδείγματος",
        "Add a housing in the Housings tab to start planning." => {
            "Προσθέστε ένα κατάλυμα στην καρτέλα Καταλύματα για να ξεκινήσετε τον σχεδιασμό."
        }
        "Group" => "Ομάδα",
        "Person" => "Άτομο",
        "Housing" => "Κατάλυμα",
        "➕ New" => "➕ Νέο",
        "Stays:" => "Διαμονές:",
        "Stays (individual):" => "Διαμονές (ατομικές):",
        "➕ Add stay" => "➕ Προσθήκη διαμονής",
        "Add a housing and a person/group first." => {
            "Προσθέστε πρώτα ένα κατάλυμα και ένα άτομο/μια ομάδα."
        }
        "(no stays)" => "(καμία διαμονή)",
        "(group)" => "(ομάδα)",
        "No groups yet — add one." => "Καμία ομάδα ακόμη — προσθέστε μία.",
        "🗑 Delete group" => "🗑 Διαγραφή ομάδας",
        "Members:" => "Μέλη:",
        "(no members)" => "(κανένα μέλος)",
        "➕ Add existing…" => "➕ Προσθήκη υπάρχοντος…",
        "➕ New person" => "➕ Νέο άτομο",
        "Select or create a group." => "Επιλέξτε ή δημιουργήστε μια ομάδα.",
        "No stays for this group yet." => "Καμία διαμονή για αυτή την ομάδα ακόμη.",
        "No persons yet — add one." => "Κανένα άτομο ακόμη — προσθέστε ένα.",
        "— no group —" => "— καμία ομάδα —",
        "🗑 Delete person" => "🗑 Διαγραφή ατόμου",
        "Select or create a person." => "Επιλέξτε ή δημιουργήστε ένα άτομο.",
        "No stays for this person yet." => "Καμία διαμονή για αυτό το άτομο ακόμη.",
        "No housings yet — add one." => "Κανένα κατάλυμα ακόμη — προσθέστε ένα.",
        "Capacity" => "Χωρητικότητα",
        "Notes:" => "Σημειώσεις:",
        "🗑 Delete housing" => "🗑 Διαγραφή καταλύματος",
        "Select or create a housing." => "Επιλέξτε ή δημιουργήστε ένα κατάλυμα.",
        "No stays in this housing yet." => "Καμία διαμονή σε αυτό το κατάλυμα ακόμη.",
        "cap" => "χωρ.",
        "To:" => "Έως:",
        "Nights:" => "Νύχτες:",
        "People:" => "Άτομα:",
        "⚠ Also booked elsewhere at the same time" => "⚠ Κρατημένο και αλλού την ίδια περίοδο",
        "<deleted person>" => "<διαγραμμένο άτομο>",
        "<deleted group>" => "<διαγραμμένη ομάδα>",
        "📜 Changelog" => "📜 Ιστορικό αλλαγών",
        "↩ Undo last change" => "↩ Αναίρεση τελευταίας αλλαγής",
        "entries" => "καταχωρίσεις",
        "No changes yet." => "Καμία αλλαγή ακόμη.",
        "(no group)" => "(καμία ομάδα)",
        "Created housing" => "Δημιουργήθηκε κατάλυμα",
        "Deleted housing" => "Διαγράφηκε κατάλυμα",
        "Renamed housing" => "Μετονομάστηκε κατάλυμα",
        "Changed capacity of" => "Αλλαγή χωρητικότητας για",
        "Edited notes of" => "Επεξεργασία σημειώσεων για",
        "Created group" => "Δημιουργήθηκε ομάδα",
        "Deleted group" => "Διαγράφηκε ομάδα",
        "Renamed group" => "Μετονομάστηκε ομάδα",
        "Changed colour of" => "Αλλαγή χρώματος για",
        "Added person" => "Προστέθηκε άτομο",
        "Deleted person" => "Διαγράφηκε άτομο",
        "Renamed person" => "Μετονομάστηκε άτομο",
        "Changed group of" => "Αλλαγή ομάδας για",
        "Added stay" => "Προστέθηκε διαμονή",
        "Removed stay" => "Αφαιρέθηκε διαμονή",
        "Moved stay" => "Μετακινήθηκε διαμονή",
        "Changed occupant of stay" => "Αλλαγή ενοίκου της διαμονής",
        "Changed dates of stay" => "Αλλαγή ημερομηνιών της διαμονής",
        "Loaded example data" => "Φορτώθηκαν δεδομένα παραδείγματος",
        "Loaded plan from file" => "Φορτώθηκε σχέδιο από αρχείο",
        "Loaded a plan with no change history" => "Φορτώθηκε σχέδιο χωρίς ιστορικό αλλαγών",
        "Undid" => "Αναιρέθηκε",
        "💾 Save" => "💾 Αποθήκευση",
        "Save As…" => "Αποθήκευση ως…",
        "Unsaved changes" => "Μη αποθηκευμένες αλλαγές",
        "You have unsaved changes. Save before closing?" => {
            "Έχετε μη αποθηκευμένες αλλαγές. Αποθήκευση πριν το κλείσιμο;"
        }
        "Save" => "Αποθήκευση",
        "Discard" => "Απόρριψη",
        "Cancel" => "Άκυρο",
        "untitled" => "χωρίς τίτλο",
        "File is open in another instance." => "Το αρχείο είναι ανοιχτό σε άλλο στιγμιότυπο.",
        // Fallback: English
        other => other,
    }
}
