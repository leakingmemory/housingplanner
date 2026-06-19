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
}

impl Default for Lang {
    fn default() -> Self {
        Lang::English
    }
}

impl Lang {
    /// All languages, for building a selector.
    pub const ALL: [Lang; 8] = [
        Lang::English,
        Lang::Swedish,
        Lang::Norwegian,
        Lang::NorwegianNynorsk,
        Lang::NorthernSami,
        Lang::Danish,
        Lang::Ukrainian,
        Lang::German,
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
        "💾 Save…" => "💾 Lagre…",
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
        "💾 Save…" => "💾 Vurke…",
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
        "💾 Save…" => "💾 Gem…",
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
        "💾 Save…" => "💾 Зберегти…",
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
        "💾 Save…" => "💾 Speichern…",
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

        // Fallback: English
        other => other,
    }
}
