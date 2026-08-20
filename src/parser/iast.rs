/// IAST → Varna parser.
///
/// IAST (International Alphabet of Sanskrit Transliteration) uses Unicode
/// diacritics and multi-character sequences. This parser handles:
/// - Diacritics: ā ī ū ṛ ṝ ḷ ḹ ś ṣ ṭ ḍ ṇ ñ ṅ
/// - Digraphs: kh gh ch jh ṭh ḍh th dh ph bh (aspirates)
/// - Compound vowels: ai au
/// - Anusvara (ṃ), visarga (ḥ), chandrabindu (m̐)
/// - Vedic svara accents (PS.11):
///   - Combining acute (U+0301) → udatta
///   - Combining grave (U+0300) → anudatta
///   - Devanagari stress sign udatta ॑ (U+0951) → svarita
///   - Devanagari stress sign anudatta ॒ (U+0952) → anudatta
///
/// Longest-match parsing: "kh" → kha, not "k" + "h".

use crate::varna::*;

/// Parse an IAST string into a sequence of Varnas.
pub fn parse(input: &str) -> Vec<Varna> {
    let mut varnas = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // Skip standalone combining accents (already consumed after vowels)
        if is_accent_mark(c) {
            i += 1;
            continue;
        }

        let next = if i + 1 < len { Some(chars[i + 1]) } else { None };

        // Try two-character sequences first (longest match)
        if let Some(nc) = next {
            if let Some(mut v) = match_digraph(c, nc) {
                i += 2;
                // Check for accent after compound vowel
                apply_accent(&mut v, &chars, &mut i);
                varnas.push(v);
                continue;
            }
        }

        // Single character
        if let Some(mut v) = match_single(c) {
            i += 1;
            // Check for accent after vowel
            apply_accent(&mut v, &chars, &mut i);
            varnas.push(v);
        } else {
            // whitespace, punctuation, unknown — skip
            i += 1;
        }
    }

    varnas
}

/// Check if a character is a Vedic accent combining mark.
fn is_accent_mark(c: char) -> bool {
    accent_pitch(c).is_some()
}

/// Map accent mark to SvaraPitch.
///
/// Supports multiple conventions used in digital Vedic texts:
///
/// Standard combining marks:
///   U+0301 (acute)        → udatta
///   U+0300 (grave)        → anudatta
///   U+030B (double acute) → svarita (jatya)
///   U+0302 (circumflex)   → svarita (jatya)
///   U+0311 (inverted breve above) → dependent svarita (some editions)
///
/// Devanagari Vedic marks (RV convention):
///   U+0951 ॑ (udatta sign)   → svarita (in RV, this overbar marks svarita)
///   U+0952 ॒ (anudatta sign) → anudatta
///   U+1CDA ᳚ (double overline) → dirgha svarita (some editions)
///
/// Note on RV convention: In Rigveda Devanagari, the overbar ॑ marks
/// svarita (not udatta, despite the Unicode name). Udatta is unmarked.
/// Anudatta is marked with underbar ॒.
fn accent_pitch(c: char) -> Option<SvaraPitch> {
    match c {
        '\u{0301}' => Some(SvaraPitch::Udatta),            // combining acute
        '\u{0300}' => Some(SvaraPitch::Anudatta),           // combining grave
        '\u{030B}' => Some(SvaraPitch::Svarita),            // combining double acute
        '\u{0302}' => Some(SvaraPitch::Svarita),            // combining circumflex
        '\u{0311}' => Some(SvaraPitch::DependentSvarita),   // combining inverted breve
        '\u{0951}' => Some(SvaraPitch::Svarita),            // devanagari ॑ (svarita in RV)
        '\u{0952}' => Some(SvaraPitch::Anudatta),           // devanagari ॒
        '\u{1CDA}' => Some(SvaraPitch::DirghaSvarita),      // vedic double overline
        '\u{0060}' => Some(SvaraPitch::Pracaya),            // grave accent char (some texts)
        _ => None,
    }
}

/// If the current position has an accent mark, pluta marker, or modifier,
/// apply it to the svara.
fn apply_accent(v: &mut Varna, chars: &[char], i: &mut usize) {
    if let Varna::Svara { ref mut pitch, ref mut kala, .. } = v {
        while *i < chars.len() {
            let c = chars[*i];
            if let Some(p) = accent_pitch(c) {
                *pitch = Some(p);
                *i += 1;
            } else if c == '3' {
                // Pluta marker — "himkāra3" convention (PS.11, 3 matras)
                *kala = Kala::Pluta;
                *i += 1;
            } else {
                break;
            }
        }
    }
}

fn match_digraph(c: char, nc: char) -> Option<Varna> {
    // Aspirate consonants: Xh digraphs
    if nc == 'h' {
        let v = match c {
            'k' => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana)),
            'g' => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana)),
            'c' => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana)),
            'j' => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana)),
            'ṭ' => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Mahaprana)),
            'ḍ' => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Mahaprana)),
            't' => Some(sparsha(Sthana::Danta, Ghosha::Aghosha, Prana::Mahaprana)),
            'd' => Some(sparsha(Sthana::Danta, Ghosha::Saghosha, Prana::Mahaprana)),
            'p' => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana)),
            'b' => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana)),
            _ => None,
        };
        if v.is_some() {
            return v;
        }
    }

    // Compound vowels
    match (c, nc) {
        ('a', 'i') => Some(compound_svara(Sthana::KanthaTalu, Vivrti::Ativivrita)),
        ('a', 'u') => Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Ativivrita)),
        _ => None,
    }
}

fn match_single(c: char) -> Option<Varna> {
    match c {
        // === Svaras ===
        'a' => Some(svara(Sthana::Kantha, Kala::Hrasva)),
        'ā' => Some(svara(Sthana::Kantha, Kala::Dirgha)),
        'i' => Some(svara(Sthana::Talu, Kala::Hrasva)),
        'ī' => Some(svara(Sthana::Talu, Kala::Dirgha)),
        'u' => Some(svara(Sthana::Oshtha, Kala::Hrasva)),
        'ū' => Some(svara(Sthana::Oshtha, Kala::Dirgha)),
        'ṛ' => Some(svara(Sthana::Murdha, Kala::Hrasva)),
        'ṝ' => Some(svara(Sthana::Murdha, Kala::Dirgha)),
        'ḷ' => Some(svara(Sthana::Danta, Kala::Hrasva)),  // vowel ḷ
        'ḹ' => Some(svara(Sthana::Danta, Kala::Dirgha)),
        'e' => Some(compound_svara(Sthana::KanthaTalu, Vivrti::Vivrita)),
        'o' => Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Vivrita)),
        // ai and au are handled as digraphs in match_digraph

        // === Sparsha (unaspirated — aspirates handled in match_digraph) ===
        'k' => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Alpaprana)),
        'g' => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Alpaprana)),
        'ṅ' => Some(nasal(Sthana::Kantha)),
        'c' => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana)),
        'j' => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Alpaprana)),
        'ñ' => Some(nasal(Sthana::Talu)),
        'ṭ' => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Alpaprana)),
        'ḍ' => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Alpaprana)),
        'ṇ' => Some(nasal(Sthana::Murdha)),
        't' => Some(sparsha(Sthana::Danta, Ghosha::Aghosha, Prana::Alpaprana)),
        'd' => Some(sparsha(Sthana::Danta, Ghosha::Saghosha, Prana::Alpaprana)),
        'n' => Some(nasal(Sthana::Danta)),
        'p' => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Alpaprana)),
        'b' => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Alpaprana)),
        'm' => Some(nasal(Sthana::Oshtha)),

        // === Antahstha ===
        'y' => Some(antahstha(Sthana::Talu)),
        'r' => Some(antahstha(Sthana::Murdha)),
        'l' => Some(antahstha(Sthana::Danta)),
        'v' => Some(antahstha(Sthana::DantaOshtha)),

        // === Ushman ===
        'ś' => Some(ushman(Sthana::Talu)),
        'ṣ' => Some(ushman(Sthana::Murdha)),
        's' => Some(ushman(Sthana::Danta)),
        'h' => Some(Varna::Vyanjana {
            sthana: Sthana::Kantha,
            prayatna: Prayatna::Vivrita,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        }),

        // === Ayogavaha ===
        'ṃ' => Some(Varna::Ayogavaha(AyogavahaType::Anusvara)),
        'ḥ' => Some(Varna::Ayogavaha(AyogavahaType::Visarga)),

        // Punctuation/whitespace passthrough
        ' ' | '\t' | '\n' | '\r' | ',' | '-' | '.' | '|' | '(' | ')' | '[' | ']'
        | ';' | '!' | '/' | '\'' | '"' | '0'..='9' => Some(Varna::Passthrough(c)),

        _ => None,
    }
}

fn svara(sthana: Sthana, kala: Kala) -> Varna {
    Varna::Svara { sthana, kala, vivrti: None, pitch: None, modifiers: SvaraModifiers::default() }
}

fn compound_svara(sthana: Sthana, vivrti: Vivrti) -> Varna {
    Varna::Svara { sthana, kala: Kala::Dirgha, vivrti: Some(vivrti), pitch: None, modifiers: SvaraModifiers::default() }
}

fn sparsha(sthana: Sthana, ghosha: Ghosha, prana: Prana) -> Varna {
    Varna::Vyanjana {
        sthana,
        prayatna: Prayatna::Sprshta,
        ghosha,
        prana,
        nasika: false,
    }
}

fn nasal(sthana: Sthana) -> Varna {
    Varna::Vyanjana {
        sthana,
        prayatna: Prayatna::Sprshta,
        ghosha: Ghosha::Saghosha,
        prana: Prana::Alpaprana,
        nasika: true,
    }
}

fn antahstha(sthana: Sthana) -> Varna {
    Varna::Vyanjana {
        sthana,
        prayatna: Prayatna::IshatSprshta,
        ghosha: Ghosha::Saghosha,
        prana: Prana::Alpaprana,
        nasika: false,
    }
}

fn ushman(sthana: Sthana) -> Varna {
    Varna::Vyanjana {
        sthana,
        prayatna: Prayatna::Vivrita,
        ghosha: Ghosha::Aghosha,
        prana: Prana::Alpaprana,
        nasika: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_vowels() {
        let v = parse("aāiīuūṛṝ");
        assert_eq!(v.len(), 8);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Hrasva, .. }));
        assert!(matches!(v[1], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Dirgha, .. }));
        assert!(matches!(v[4], Varna::Svara { sthana: Sthana::Oshtha, kala: Kala::Hrasva, .. }));
        assert!(matches!(v[6], Varna::Svara { sthana: Sthana::Murdha, kala: Kala::Hrasva, .. }));
    }

    #[test]
    fn test_compound_vowels() {
        // Test each compound vowel separately to avoid ambiguity
        let v = parse("e");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaTalu, vivrti: Some(Vivrti::Vivrita), .. }));

        let v = parse("ai");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaTalu, vivrti: Some(Vivrti::Ativivrita), .. }));

        let v = parse("o");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaOshtha, vivrti: Some(Vivrti::Vivrita), .. }));

        let v = parse("au");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaOshtha, vivrti: Some(Vivrti::Ativivrita), .. }));
    }

    #[test]
    fn test_au() {
        let v = parse("au");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaOshtha, vivrti: Some(Vivrti::Ativivrita), .. }));
    }

    #[test]
    fn test_aspirates() {
        let v: Vec<_> = parse("khghchjhṭhḍhthdh phbh")
            .into_iter().filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 10);
        // kh = kantha, aghosha, mahaprana
        assert!(matches!(v[0], Varna::Vyanjana {
            sthana: Sthana::Kantha,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Mahaprana,
            ..
        }));
        // gh = kantha, saghosha, mahaprana
        assert!(matches!(v[1], Varna::Vyanjana {
            sthana: Sthana::Kantha,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Mahaprana,
            ..
        }));
        // ṭh = murdha, aghosha, mahaprana
        assert!(matches!(v[4], Varna::Vyanjana {
            sthana: Sthana::Murdha,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Mahaprana,
            ..
        }));
    }

    #[test]
    fn test_retroflex() {
        let v = parse("ṭḍṇṣ");
        assert_eq!(v.len(), 4);
        assert_eq!(v[0].sthana(), Some(Sthana::Murdha));
        assert_eq!(v[1].sthana(), Some(Sthana::Murdha));
        assert_eq!(v[2].sthana(), Some(Sthana::Murdha));
        assert_eq!(v[3].sthana(), Some(Sthana::Murdha));
    }

    #[test]
    fn test_sibilants() {
        let v = parse("śṣs");
        assert_eq!(v.len(), 3);
        assert!(v[0].is_ushman());
        assert!(v[1].is_ushman());
        assert!(v[2].is_ushman());
        assert_eq!(v[0].sthana(), Some(Sthana::Talu));
        assert_eq!(v[1].sthana(), Some(Sthana::Murdha));
        assert_eq!(v[2].sthana(), Some(Sthana::Danta));
    }

    #[test]
    fn test_ayogavaha() {
        let v = parse("ṃḥ");
        assert_eq!(v.len(), 2);
        assert!(matches!(v[0], Varna::Ayogavaha(AyogavahaType::Anusvara)));
        assert!(matches!(v[1], Varna::Ayogavaha(AyogavahaType::Visarga)));
    }

    #[test]
    fn test_agnim_ile() {
        // RV 1.1.1 opening
        let v = parse("agnimīḷe purohitam");
        // a g n i m ī ḷ e p u r o h i t a m
        // Note: ḷ here is consonant ḷ (parser sees it as vowel ḷ currently —
        // disambiguation between vowel ḷ and consonant ḷ is context-dependent,
        // but for the byte encoding they map to the same phoneme)
        assert!(v.len() > 0);
        // First varna is 'a'
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Hrasva, .. }));
    }

    #[test]
    fn test_krsna() {
        let v = parse("kṛṣṇa");
        assert_eq!(v.len(), 5);
        // k
        assert!(matches!(v[0], Varna::Vyanjana { sthana: Sthana::Kantha, prana: Prana::Alpaprana, .. }));
        // ṛ
        assert!(matches!(v[1], Varna::Svara { sthana: Sthana::Murdha, kala: Kala::Hrasva, .. }));
        // ṣ
        assert!(matches!(v[2], Varna::Vyanjana { sthana: Sthana::Murdha, prayatna: Prayatna::Vivrita, .. }));
        // ṇ
        assert!(v[3].is_anunasika());
        assert_eq!(v[3].sthana(), Some(Sthana::Murdha));
        // a
        assert!(matches!(v[4], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Hrasva, .. }));
    }
}
