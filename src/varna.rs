/// Paniniya Shiksha Serialization — core types.
///
/// All types derive from the five-fold classification (PS.10):
/// "svarataḥ kālataḥ sthānāt prayatnānupradānataḥ"

/// Place of articulation (PS.13, PS.17-18).
///
/// "aṣṭausthānāni varṇānāmuraḥ kaṇṭhaḥ śirastathā
///  jihvāmūlaṃ cha dantāścha nāsikōṣṭhaucha tālu cha"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sthana {
    /// Guttural — a, ka-varga, ha, visarga
    /// PS.17: "kaṇṭhyāvahāv"
    Kantha,
    /// Palatal — i, ca-varga, ya, śa
    /// PS.17: "ichuyaśāstālavyā"
    Talu,
    /// Retroflex — ṛ, ṭa-varga, ra, ṣa
    /// PS.17: "syurmūrdhanyā ṛṭuraṣā"
    Murdha,
    /// Dental — ḷ, ta-varga, la, sa
    /// PS.17: "dantyā ḷitulasāḥ smṛtāḥ"
    Danta,
    /// Labial — u, pa-varga
    /// PS.17: "ōṣṭhajāvupū"
    Oshtha,
    /// Guttural-palatal — e, ai
    /// PS.18: "ēai tu kaṇṭhatālavyā"
    KanthaTalu,
    /// Guttural-labial — o, au
    /// PS.18: "ōau kaṇṭhōṣṭhajau smṛtau"
    KanthaOshtha,
    /// Dental-labial — va
    /// PS.18: "dantyōṣṭhyō vaḥ smṛtō budhaiḥ"
    DantaOshtha,
}

/// Articulatory effort (PS.38).
///
/// "achō'spṛṣṭā yaṇastvīṣannēmaspṛṣṭāḥ śalaḥ smṛtāḥ
///  śēṣāḥ spṛṣṭā halaḥ prōktā"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prayatna {
    /// Full contact — sparsha (stops: ka-varga through pa-varga)
    Sprshta,
    /// Slight contact — antahstha (semivowels: ya va ra la)
    IshatSprshta,
    /// Open — ushman (sibilants: śa ṣa sa ha)
    Vivrita,
}

/// Voicing (PS.39).
///
/// Derived from nadin/ishannadin (saghosha) vs shvasin (aghosha).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ghosha {
    /// Unvoiced — 1st/2nd of varga, sibilants
    Aghosha,
    /// Voiced — 3rd/4th/5th of varga, semivowels, vowels
    Saghosha,
}

/// Aspiration (PS.39).
///
/// Derived from shvasin/nadin (mahaprana) vs others (alpaprana).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prana {
    /// Unaspirated — 1st, 3rd, 5th of varga
    Alpaprana,
    /// Aspirated — 2nd, 4th of varga
    Mahaprana,
}

/// Vowel duration (PS.11).
///
/// "hrasvō dīrghaḥ pluta iti kālatō niyamā achi"
/// PS.49: "chāṣastu vadatē mātrāṃ dvimātraṃ chaiva vāyasaḥ
///         śikhī rauti trimātraṃ tu nakulastvardhamātrakam"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kala {
    /// 1 matra (chataka bird, PS.49)
    Hrasva,
    /// 2 matras (crow, PS.49)
    Dirgha,
    /// 3 matras (peacock, PS.49)
    Pluta,
}

/// Degree of opening for compound vowels (PS.21).
///
/// "svarāṇāmūṣmaṇāṃ chaiva vivṛtaṃ karaṇaṃ smṛtam
///  tēbhyō'pi vivṛtāvēṅau tābhyāmaichau tathaiva cha"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vivrti {
    /// Standard compound opening — e, o (guna)
    Vivrita,
    /// Greater compound opening — ai, au (vrddhi)
    /// PS.21: "tābhyāmaichau tathaiva cha"
    Ativivrita,
}

/// Pitch accent — full Vedic system.
///
/// Basic three (PS.11):
/// "udāttaśchānudāttaścha svaritaścha svarāstrayaḥ"
///
/// Nava-pada-shayyā (PS.45):
/// "antōdāttamādyudāttamudāttamanudāttaṃ nīchasvaritam
///  madhyōdāttaṃ svaritaṃ dvyudāttaṃ tryudāttamiti"
///
/// PS.48: "anudāttō hṛdi jñēyō mūrdhnyudātta udāhṛtaḥ
///         svaritaḥ karṇamūlīyaḥ sarvāsyē prachayaḥ smṛtaḥ"
///
/// Svarita subtypes from the Pratishakhyas (RPr, TPr):
/// The five types of svarita arise from different sandhi contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvaraPitch {
    // === Basic three (PS.11) ===

    /// Raised pitch — "mūrdhni" (PS.48)
    Udatta,
    /// Lowered pitch — "hṛdi" (PS.48)
    Anudatta,
    /// Independent/natural svarita (jātya) — "karṇamūlīya" (PS.48)
    /// On e, o, ai, au arising from sandhi. A true falling pitch.
    Svarita,

    // === Extended accents (PS.45, PS.48) ===

    /// Leveled continuation after svarita — "sarvāsyē" (PS.48)
    /// Distinct from anudatta: not a marked low, but absence of movement.
    Pracaya,

    // === Svarita subtypes (Pratishakhyas) ===

    /// Dependent svarita — on syllable immediately following udatta.
    /// Automatic, not independently marked in most texts.
    DependentSvarita,
    /// Dirgha svarita — svarita on a long (dirgha) vowel.
    /// PS.29-30: extended falling contour, often with kampa.
    DirghaSvarita,
    /// Kṣaipra — "quick" svarita. Occurs when an udatta-final short
    /// vowel is followed by a vowel-initial word. The udatta contracts
    /// and the resulting syllable gets kṣaipra svarita.
    Kshaipra,
    /// Praśliṣṭa — svarita from vowel coalescence (sandhi) across
    /// a word boundary where both vowels merge.
    Prashlishta,
    /// Abhinihita — svarita when an udatta vowel at word-end is
    /// followed by a vowel-initial word and the two coalesce into
    /// one syllable with avagraha marking the elision.
    Abhinihita,
    /// Tairovyañjana — svarita that occurs when a consonant separates
    /// an udatta syllable from the following syllable that would
    /// otherwise receive dependent svarita.
    Tairovyanjana,
}

impl SvaraPitch {
    /// Musical note mapping per PS.12.
    ///
    /// "udāttē niṣādagāndhārāvanudātta ṛṣabhadhaivatau
    ///  svaritaprabhavā hyētē ṣaḍjamadhyamapañchamāḥ"
    pub fn musical_notes(&self) -> &'static [&'static str] {
        match self {
            SvaraPitch::Udatta => &["niṣāda", "gāndhāra"],
            SvaraPitch::Anudatta | SvaraPitch::Pracaya => &["ṛṣabha", "dhaivata"],
            _ => &["ṣaḍja", "madhyama", "pañchama"],  // svarita-derived
        }
    }

    /// Returns true if this is any kind of svarita.
    pub fn is_svarita(&self) -> bool {
        matches!(self,
            SvaraPitch::Svarita |
            SvaraPitch::DependentSvarita |
            SvaraPitch::DirghaSvarita |
            SvaraPitch::Kshaipra |
            SvaraPitch::Prashlishta |
            SvaraPitch::Abhinihita |
            SvaraPitch::Tairovyanjana
        )
    }
}

/// Recitation modifiers for a svara (PS.26-30).
///
/// These are performance features that co-occur with pitch accent,
/// not independent accent types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SvaraModifiers {
    /// Kampa (tremolo) — PS.29-30.
    /// "hṛdayādutkarē tiṣṭhankāṃsyēna samanusvaran
    ///  mārdavaṃ cha dvimātraṃ cha jaghanvāँ iti nidarśanam"
    /// PS.30: "madhyē tu kampayētkampam"
    /// A wavering of pitch on certain svarita syllables.
    pub kampa: bool,

    /// Ranga (nasal resonance) — PS.26-28.
    /// PS.26: "yathā saurāṣṭrikā nārī takraँ ityabhibhāṣatē
    ///         ēvaṃ raṅgāḥ prayōktavyāḥ"
    /// PS.28: "hṛdayē chaikamātrastvarddhamātrastu mūrddhani
    ///         nāsikāyāṃ tathārddhaṃ cha raṅgasyaivaṃ dvimātratā"
    /// Total duration: 2 matras (1 in chest, ½ in head, ½ in nose).
    pub ranga: bool,
}

/// Samaveda svara notation — 7-note system.
///
/// The Samaveda uses a completely different tonal system from the
/// 3-accent RV/YV system. Seven notes (svaras) are denoted by
/// numerals 1-7 in printed editions.
///
/// PS.12 maps the basic three accents to these musical notes:
/// "svaritaprabhavā hyētē ṣaḍjamadhyamapañchamāḥ"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SamaGana {
    /// 1 — krushta (highest)
    Krushta = 1,
    /// 2 — prathama
    Prathama = 2,
    /// 3 — dvitiya
    Dvitiya = 3,
    /// 4 — tritiya
    Tritiya = 4,
    /// 5 — chaturtha
    Chaturtha = 5,
    /// 6 — mandra
    Mandra = 6,
    /// 7 — atisvarya (lowest)
    Atisvarya = 7,
}

/// Dependent phonemes (PS.5, PS.22).
///
/// "anusvārō visargaścha ka pau chāpi parāśritau"
/// PS.22: "ayōgavāhā vijñēyā āśrayasthānabhāginaḥ"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AyogavahaType {
    Anusvara,
    Visarga,
    /// PS.5: "ka" — visarga variant before ka-varga
    Jihvamuliya,
    /// PS.5: "pau" — visarga variant before pa-varga
    Upadhmaniya,
    Chandrabindu,
}

/// A single Sanskrit phoneme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Varna {
    /// Vowel (PS.4: "svarāviṃśatirēkaścha")
    Svara {
        sthana: Sthana,
        kala: Kala,
        vivrti: Option<Vivrti>,
        pitch: Option<SvaraPitch>,
        modifiers: SvaraModifiers,
    },

    /// Consonant (PS.4: "sparśānāṃ pañchaviṃśatiḥ" + "yādayaścha smṛtā hyaṣṭau")
    Vyanjana {
        sthana: Sthana,
        prayatna: Prayatna,
        ghosha: Ghosha,
        prana: Prana,
        nasika: bool,
    },

    /// Dependent phoneme (PS.5)
    Ayogavaha(AyogavahaType),
}

impl Varna {
    /// Returns the sthana of this varna, if applicable.
    pub fn sthana(&self) -> Option<Sthana> {
        match self {
            Varna::Svara { sthana, .. } => Some(*sthana),
            Varna::Vyanjana { sthana, .. } => Some(*sthana),
            Varna::Ayogavaha(_) => None,
        }
    }

    /// Returns true if this varna is a svara (vowel).
    pub fn is_svara(&self) -> bool {
        matches!(self, Varna::Svara { .. })
    }

    /// Returns true if this varna is a vyanjana (consonant).
    pub fn is_vyanjana(&self) -> bool {
        matches!(self, Varna::Vyanjana { .. })
    }

    /// Returns true if two svaras are savarna (same sthana).
    pub fn is_savarna(&self, other: &Varna) -> bool {
        match (self, other) {
            (Varna::Svara { sthana: s1, .. }, Varna::Svara { sthana: s2, .. }) => s1 == s2,
            _ => false,
        }
    }

    /// Returns true if this is a sparsha (stop consonant, PS.38 sprshta).
    pub fn is_sparsha(&self) -> bool {
        matches!(self, Varna::Vyanjana { prayatna: Prayatna::Sprshta, .. })
    }

    /// Returns true if this is an antahstha (semivowel, PS.38 ishat-sprshta).
    pub fn is_antahstha(&self) -> bool {
        matches!(self, Varna::Vyanjana { prayatna: Prayatna::IshatSprshta, .. })
    }

    /// Returns true if this is an ushman (sibilant/ha, PS.38 vivrita).
    pub fn is_ushman(&self) -> bool {
        matches!(self, Varna::Vyanjana { prayatna: Prayatna::Vivrita, .. })
    }

    /// Returns true if this is a nasal (5th of varga, PS.39 anunasika).
    pub fn is_anunasika(&self) -> bool {
        matches!(self, Varna::Vyanjana { nasika: true, .. })
    }

    /// Returns the matra count for a svara (PS.11, PS.49).
    pub fn matra_count(&self) -> Option<u8> {
        match self {
            Varna::Svara { kala, .. } => Some(match kala {
                Kala::Hrasva => 1,
                Kala::Dirgha => 2,
                Kala::Pluta => 3,
            }),
            _ => None,
        }
    }
}
