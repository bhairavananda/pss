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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kala {
    /// 1 matra
    Hrasva,
    /// 2 matras
    Dirgha,
    /// 3 matras
    Pluta,
}

/// Degree of opening for compound vowels (PS.21).
///
/// "svarāṇāmūṣmaṇāṃ chaiva vivṛtaṃ karaṇaṃ smṛtam
///  tēbhyō'pi vivṛtāvēṅau tābhyāmaichau tathaiva cha"
///
/// All vowels are vivrita. Among compound vowels, e/o have standard
/// opening (vivrita), while ai/au have greater opening (ativivrita).
/// Simple vowels (a, i, u, etc.) have no compound opening — this field
/// is None for them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vivrti {
    /// Standard compound opening — e, o (guna)
    Vivrita,
    /// Greater compound opening — ai, au (vrddhi)
    /// PS.21: "tābhyāmaichau tathaiva cha"
    Ativivrita,
}

/// Pitch accent (PS.11, PS.45, PS.48).
///
/// The basic three (PS.11):
/// "udāttaśchānudāttaścha svaritaścha svarāstrayaḥ"
///
/// The full RV system (PS.45 — nava-pada-shayyā):
/// "antōdāttamādyudāttamudāttamanudāttaṃ nīchasvaritam
///  madhyōdāttaṃ svaritaṃ dvyudāttaṃ tryudāttamiti navapadaśayyā"
///
/// PS.48 locates each in the body:
/// "anudāttō hṛdi jñēyō mūrdhnyudātta udāhṛtaḥ
///  svaritaḥ karṇamūlīyaḥ sarvāsyē prachayaḥ smṛtaḥ"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvaraPitch {
    /// Raised pitch — "mūrdhni" (PS.48)
    Udatta,
    /// Lowered pitch — "hṛdi" (PS.48)
    Anudatta,
    /// Independent/natural falling pitch (jātya) — "karṇamūlīya" (PS.48)
    /// Occurs on e, o, ai, au arising from sandhi, and on certain
    /// inherently svarita syllables.
    Svarita,
    /// Dependent svarita — automatically placed on the syllable
    /// immediately following an udatta. Not independently marked in
    /// most texts but phonologically distinct from jātya svarita.
    DependentSvarita,
    /// Dirgha svarita — svarita on a long (dirgha) vowel.
    /// PS.29-30: "hṛdayādutkarē tiṣṭhan" ... "madhyē tu kampayētkampam"
    /// Has an extended falling contour, often with kampa (tremolo).
    DirghaSvarita,
    /// Leveled/monotone continuation after svarita — "sarvāsyē" (PS.48)
    /// PS.45: "prachayam" in the nava-pada-shayyā.
    /// Phonologically distinct from true anudatta: anudatta is a marked
    /// low tone, pracaya is the absence of tonal movement.
    Pracaya,
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
    /// Used for savarna-dirgha sandhi.
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

    /// Returns the matra count for a svara (PS.11).
    /// PS.49: "chāṣastu vadatē mātrāṃ dvimātraṃ chaiva vāyasaḥ
    ///         śikhī rauti trimātraṃ tu"
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
