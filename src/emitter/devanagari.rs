/// Varna → Devanagari emitter.
///
/// Unicode Devanagari rules:
/// - Consonants carry inherent 'a': क = ka
/// - Virama (्, U+094D) suppresses inherent vowel: क् = k
/// - Vowels after consonants use dependent matra forms: कि = ki
/// - Vowels at start or after another vowel use independent forms: अ इ
/// - Anusvara (ं) and visarga (ः) attach after the akshara
/// - Vedic accents use U+0951 (svarita) and U+0952 (anudatta)

use crate::varna::*;

/// Emit a sequence of Varnas as a Devanagari Unicode string.
pub fn emit(varnas: &[Varna]) -> String {
    let mut out = String::with_capacity(varnas.len() * 3);
    let len = varnas.len();

    let mut i = 0;
    while i < len {
        match &varnas[i] {
            Varna::Vyanjana { .. } => {
                // Emit consonant base
                out.push_str(vyanjana_char(&varnas[i]));

                // Check what follows
                if i + 1 < len {
                    match &varnas[i + 1] {
                        Varna::Svara { pitch, .. } => {
                            // Consonant + vowel: emit dependent matra (unless 'a')
                            let matra = svara_matra(&varnas[i + 1]);
                            if let Some(m) = matra {
                                out.push_str(m);
                            }
                            i += 2;
                            // Emit any following ayogavaha BEFORE the accent mark
                            let pitch_val = *pitch;
                            while i < len {
                                if let Varna::Ayogavaha(typ) = &varnas[i] {
                                    out.push_str(ayogavaha_str(typ));
                                    i += 1;
                                } else {
                                    break;
                                }
                            }
                            emit_pitch(&mut out, pitch_val);
                            continue;
                        }
                        Varna::Vyanjana { .. } => {
                            // Consonant + consonant: emit virama for conjunct
                            out.push('\u{094D}');
                        }
                        Varna::Ayogavaha(_) => {
                            // Consonant + ayogavaha: inherent 'a' stays
                        }
                        Varna::Passthrough(_) => {
                            // Consonant before space/punctuation: virama
                            out.push('\u{094D}');
                        }
                    }
                } else {
                    // Final consonant: emit virama (no inherent vowel at end)
                    out.push('\u{094D}');
                }
                i += 1;
            }

            Varna::Svara { pitch, .. } => {
                // Independent vowel form (start of word or after another vowel)
                out.push_str(svara_independent(&varnas[i]));
                i += 1;
                // Emit any following ayogavaha BEFORE the accent mark
                let pitch_val = *pitch;
                while i < len {
                    if let Varna::Ayogavaha(typ) = &varnas[i] {
                        out.push_str(ayogavaha_str(typ));
                        i += 1;
                    } else {
                        break;
                    }
                }
                emit_pitch(&mut out, pitch_val);
                continue;
            }

            Varna::Ayogavaha(typ) => {
                out.push_str(match typ {
                    AyogavahaType::Anusvara => "\u{0902}",     // ं
                    AyogavahaType::Visarga => "\u{0903}",      // ः
                    AyogavahaType::Chandrabindu => "\u{0901}",  // ँ
                    AyogavahaType::Jihvamuliya => "\u{0CF1}",   // rare, use Kannada sign
                    AyogavahaType::Upadhmaniya => "\u{0CF2}",   // rare
                });
                i += 1;
            }

            Varna::Passthrough(c) => {
                let ch = match c {
                    '|' => '।',
                    _ => *c,
                };
                // Collapse consecutive spaces
                if ch == ' ' && out.ends_with(' ') {
                    // skip duplicate space
                } else {
                    out.push(ch);
                }
                i += 1;
            }
        }
    }

    out
}

fn ayogavaha_str(typ: &AyogavahaType) -> &'static str {
    match typ {
        AyogavahaType::Anusvara => "\u{0902}",     // ं
        AyogavahaType::Visarga => "\u{0903}",       // ः
        AyogavahaType::Chandrabindu => "\u{0901}",  // ँ
        AyogavahaType::Jihvamuliya => "\u{0CF1}",
        AyogavahaType::Upadhmaniya => "\u{0CF2}",
    }
}

/// Emit Vedic accent marks in Devanagari (RV convention).
fn emit_pitch(out: &mut String, pitch: Option<SvaraPitch>) {
    if let Some(p) = pitch {
        match p {
            // RV convention: udatta unmarked, anudatta = underbar, svarita = overbar
            SvaraPitch::Anudatta => out.push('\u{0952}'),   // ॒
            SvaraPitch::Svarita |
            SvaraPitch::DependentSvarita |
            SvaraPitch::DirghaSvarita |
            SvaraPitch::Kshaipra |
            SvaraPitch::Prashlishta |
            SvaraPitch::Abhinihita |
            SvaraPitch::Tairovyanjana => out.push('\u{0951}'), // ॑
            SvaraPitch::Udatta => {} // unmarked in RV Devanagari
            SvaraPitch::Pracaya => {} // unmarked (same as anudatta visually in most editions)
        }
    }
}

/// Get the Devanagari consonant character for a vyanjana.
fn vyanjana_char(v: &Varna) -> &'static str {
    match v {
        Varna::Vyanjana { sthana, prayatna, ghosha, prana, nasika } => {
            match prayatna {
                Prayatna::Sprshta => {
                    if *nasika {
                        match sthana {
                            Sthana::Kantha => "ङ",
                            Sthana::Talu => "ञ",
                            Sthana::Murdha => "ण",
                            Sthana::Danta => "न",
                            Sthana::Oshtha => "म",
                            _ => "",
                        }
                    } else {
                        match (sthana, ghosha, prana) {
                            (Sthana::Kantha, Ghosha::Aghosha, Prana::Alpaprana) => "क",
                            (Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana) => "ख",
                            (Sthana::Kantha, Ghosha::Saghosha, Prana::Alpaprana) => "ग",
                            (Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana) => "घ",
                            (Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana) => "च",
                            (Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana) => "छ",
                            (Sthana::Talu, Ghosha::Saghosha, Prana::Alpaprana) => "ज",
                            (Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana) => "झ",
                            (Sthana::Murdha, Ghosha::Aghosha, Prana::Alpaprana) => "ट",
                            (Sthana::Murdha, Ghosha::Aghosha, Prana::Mahaprana) => "ठ",
                            (Sthana::Murdha, Ghosha::Saghosha, Prana::Alpaprana) => "ड",
                            (Sthana::Murdha, Ghosha::Saghosha, Prana::Mahaprana) => "ढ",
                            (Sthana::Danta, Ghosha::Aghosha, Prana::Alpaprana) => "त",
                            (Sthana::Danta, Ghosha::Aghosha, Prana::Mahaprana) => "थ",
                            (Sthana::Danta, Ghosha::Saghosha, Prana::Alpaprana) => "द",
                            (Sthana::Danta, Ghosha::Saghosha, Prana::Mahaprana) => "ध",
                            (Sthana::Oshtha, Ghosha::Aghosha, Prana::Alpaprana) => "प",
                            (Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana) => "फ",
                            (Sthana::Oshtha, Ghosha::Saghosha, Prana::Alpaprana) => "ब",
                            (Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana) => "भ",
                            _ => "",
                        }
                    }
                }
                Prayatna::IshatSprshta => match sthana {
                    Sthana::Talu => "य",
                    Sthana::Murdha => "र",
                    Sthana::Danta => "ल",
                    Sthana::DantaOshtha => "व",
                    _ => "ळ",  // ḷa
                },
                Prayatna::Vivrita => match sthana {
                    Sthana::Talu => "श",
                    Sthana::Murdha => "ष",
                    Sthana::Danta => "स",
                    Sthana::Kantha => "ह",
                    _ => "",
                },
            }
        }
        _ => "",
    }
}

/// Get the independent (standalone) Devanagari vowel form.
fn svara_independent(v: &Varna) -> &'static str {
    match v {
        Varna::Svara { sthana, kala, vivrti, .. } => {
            match (sthana, vivrti) {
                (Sthana::KanthaTalu, Some(Vivrti::Ativivrita)) => "ऐ",
                (Sthana::KanthaTalu, _) => "ए",
                (Sthana::KanthaOshtha, Some(Vivrti::Ativivrita)) => "औ",
                (Sthana::KanthaOshtha, _) => "ओ",
                _ => match (sthana, kala) {
                    (Sthana::Kantha, Kala::Hrasva) => "अ",
                    (Sthana::Kantha, _) => "आ",
                    (Sthana::Talu, Kala::Hrasva) => "इ",
                    (Sthana::Talu, _) => "ई",
                    (Sthana::Oshtha, Kala::Hrasva) => "उ",
                    (Sthana::Oshtha, _) => "ऊ",
                    (Sthana::Murdha, Kala::Hrasva) => "ऋ",
                    (Sthana::Murdha, _) => "ॠ",
                    (Sthana::Danta, Kala::Hrasva) => "ऌ",
                    (Sthana::Danta, _) => "ॡ",
                    _ => "",
                },
            }
        }
        _ => "",
    }
}

/// Get the dependent matra form for a vowel following a consonant.
/// Returns None for 'a' (inherent vowel — no matra needed).
fn svara_matra(v: &Varna) -> Option<&'static str> {
    match v {
        Varna::Svara { sthana, kala, vivrti, .. } => {
            match (sthana, vivrti) {
                (Sthana::KanthaTalu, Some(Vivrti::Ativivrita)) => Some("\u{0948}"),  // ै
                (Sthana::KanthaTalu, _) => Some("\u{0947}"),                          // े
                (Sthana::KanthaOshtha, Some(Vivrti::Ativivrita)) => Some("\u{094C}"), // ौ
                (Sthana::KanthaOshtha, _) => Some("\u{094B}"),                        // ो
                _ => match (sthana, kala) {
                    (Sthana::Kantha, Kala::Hrasva) => None,                     // inherent a
                    (Sthana::Kantha, _) => Some("\u{093E}"),                     // ा
                    (Sthana::Talu, Kala::Hrasva) => Some("\u{093F}"),            // ि
                    (Sthana::Talu, _) => Some("\u{0940}"),                       // ी
                    (Sthana::Oshtha, Kala::Hrasva) => Some("\u{0941}"),          // ु
                    (Sthana::Oshtha, _) => Some("\u{0942}"),                     // ू
                    (Sthana::Murdha, Kala::Hrasva) => Some("\u{0943}"),          // ृ
                    (Sthana::Murdha, _) => Some("\u{0944}"),                     // ॄ
                    (Sthana::Danta, Kala::Hrasva) => Some("\u{0962}"),           // ॢ
                    (Sthana::Danta, _) => Some("\u{0963}"),                      // ॣ
                    _ => None,
                },
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::iast as iast_parser;
    use crate::parser::baraha as baraha_parser;

    #[test]
    fn test_simple_consonant_vowel() {
        let v = iast_parser::parse("ka");
        assert_eq!(emit(&v), "क");

        let v = iast_parser::parse("ki");
        assert_eq!(emit(&v), "कि");

        let v = iast_parser::parse("ku");
        assert_eq!(emit(&v), "कु");

        let v = iast_parser::parse("kā");
        assert_eq!(emit(&v), "का");

        let v = iast_parser::parse("kī");
        assert_eq!(emit(&v), "की");

        let v = iast_parser::parse("kū");
        assert_eq!(emit(&v), "कू");
    }

    #[test]
    fn test_independent_vowels() {
        let v = iast_parser::parse("a");
        assert_eq!(emit(&v), "अ");

        let v = iast_parser::parse("ā");
        assert_eq!(emit(&v), "आ");

        let v = iast_parser::parse("i");
        assert_eq!(emit(&v), "इ");

        let v = iast_parser::parse("u");
        assert_eq!(emit(&v), "उ");

        let v = iast_parser::parse("e");
        assert_eq!(emit(&v), "ए");

        let v = iast_parser::parse("ai");
        assert_eq!(emit(&v), "ऐ");

        let v = iast_parser::parse("o");
        assert_eq!(emit(&v), "ओ");

        let v = iast_parser::parse("au");
        assert_eq!(emit(&v), "औ");
    }

    #[test]
    fn test_conjuncts() {
        // Two consonants + vowel = virama between them
        let v = iast_parser::parse("kra");
        assert_eq!(emit(&v), "क्र");

        let v = iast_parser::parse("sta");
        assert_eq!(emit(&v), "स्त");
    }

    #[test]
    fn test_anusvara_visarga() {
        let v = iast_parser::parse("aṃ");
        assert_eq!(emit(&v), "अं");

        let v = iast_parser::parse("namaḥ");
        assert_eq!(emit(&v), "नमः");
    }

    #[test]
    fn test_krsna() {
        let v = iast_parser::parse("kṛṣṇa");
        assert_eq!(emit(&v), "कृष्ण");
    }

    #[test]
    fn test_dharma() {
        let v = iast_parser::parse("dharma");
        assert_eq!(emit(&v), "धर्म");
    }

    #[test]
    fn test_siva() {
        let v = iast_parser::parse("śiva");
        assert_eq!(emit(&v), "शिव");
    }

    #[test]
    fn test_rama() {
        let v = iast_parser::parse("rāma");
        assert_eq!(emit(&v), "राम");
    }

    #[test]
    fn test_bhagavadgita() {
        let v = iast_parser::parse("bhagavadgītā");
        assert_eq!(emit(&v), "भगवद्गीता");
    }

    #[test]
    fn test_baraha_to_devanagari() {
        let v = baraha_parser::parse("kRuShNa");
        assert_eq!(emit(&v), "कृष्ण");

        let v = baraha_parser::parse("shiva");
        assert_eq!(emit(&v), "शिव");

        let v = baraha_parser::parse("raama");
        // Note: "raama" = ra + aa + ma + a = राम (not रामा, 'a' after 'm' is inherent)
        // Actually: r=र, aa=आ(matra), m=म, a=अ... hmm
        // In Baraha "raama" parses as: r a aa m a
        // Wait: r, then 'a' (single), then 'aa' (digraph), then 'm', then 'a'
        // = ra + ā + ma = रामा? No...
        // Actually longest match: r, aa, m, a = r + ā + m + a = राम
        let v = baraha_parser::parse("raama");
        assert_eq!(emit(&v), "राम");
    }

    #[test]
    fn test_agnim() {
        let v = iast_parser::parse("agnim");
        let dev = emit(&v);
        assert_eq!(dev, "अग्निम्");
    }

    #[test]
    fn test_purohitam() {
        let v = iast_parser::parse("purohitam");
        let dev = emit(&v);
        assert_eq!(dev, "पुरोहितम्");
    }

    #[test]
    fn test_final_consonant() {
        // Word ending in consonant should have virama
        let v = iast_parser::parse("tat");
        assert_eq!(emit(&v), "तत्");
    }

    #[test]
    fn test_samskrtam() {
        let v = iast_parser::parse("saṃskṛtam");
        assert_eq!(emit(&v), "संस्कृतम्");
    }
}
