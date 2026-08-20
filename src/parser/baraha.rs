/// Baraha → Varna parser.
///
/// Baraha is an ASCII transliteration scheme popular in South Indian
/// computing contexts. Key conventions:
///
/// - Retroflex via uppercase: T Th D Dh N
/// - Nasals via tilde prefix: ~g (ṅ), ~j (ñ)
/// - ch = ca (palatal unaspirated), Ch/chh = cha (aspirated)
/// - sh = śa, Sh = ṣa, s = sa
/// - aa/A = ā, ee/I = ī, oo/U = ū
/// - Ru = ṛ, Roo = ṝ
/// - M = anusvara, H = visarga (after vowel)
///
/// Vedic svara accents (KYV/Taittiriya convention):
/// - q after vowel = anudatta (lowered pitch)
/// - # after vowel = svarita (falling pitch)
/// - $ after vowel = anudatta (phrase-final variant)
/// - unmarked = udatta (KYV convention: udatta is default)
///
/// Longest-match parsing required due to multi-character sequences.

use crate::varna::*;

/// Parse a Baraha string into a sequence of Varnas.
pub fn parse(input: &str) -> Vec<Varna> {
    let mut varnas = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Skip accent markers (handled after vowel parsing)
        if is_baraha_accent(chars[i]) {
            i += 1;
            continue;
        }

        // Try 3-char sequences first
        if i + 2 < len {
            if let Some(v) = match_three(chars[i], chars[i + 1], chars[i + 2]) {
                varnas.push(v);
                i += 3;
                apply_baraha_accent(&mut varnas, &chars, &mut i);
                continue;
            }
        }

        // Try 2-char sequences
        if i + 1 < len {
            if let Some(v) = match_two(chars[i], chars[i + 1]) {
                varnas.push(v);
                i += 2;
                apply_baraha_accent(&mut varnas, &chars, &mut i);
                continue;
            }
        }

        // Single char
        if let Some(v) = match_one(chars[i]) {
            varnas.push(v);
            i += 1;
            apply_baraha_accent(&mut varnas, &chars, &mut i);
        } else {
            i += 1;
        }
    }

    varnas
}

/// Check if a character is a Baraha Vedic accent marker.
fn is_baraha_accent(c: char) -> bool {
    matches!(c, 'q' | '#' | '$')
}

/// After parsing a varna, check if the next char is an accent marker.
/// If the last varna is a svara, apply the accent to it.
fn apply_baraha_accent(varnas: &mut Vec<Varna>, chars: &[char], i: &mut usize) {
    while *i < chars.len() && is_baraha_accent(chars[*i]) {
        let pitch = match chars[*i] {
            'q' => SvaraPitch::Anudatta,
            '#' => SvaraPitch::Svarita,
            '$' => SvaraPitch::Anudatta,  // phrase-final anudatta
            _ => unreachable!(),
        };
        // Apply to the last svara in the varnas list
        // In Baraha, accent follows the vowel of the akshara, so
        // we need to find the last svara and set its pitch
        if let Some(Varna::Svara { pitch: ref mut p, .. }) = varnas.last_mut() {
            *p = Some(pitch);
        }
        *i += 1;
    }
}

fn match_three(a: char, b: char, c: char) -> Option<Varna> {
    match (a, b, c) {
        // 3-char aspirates with uppercase retroflex
        ('T', 'h', 'a') => None, // let match_two handle Th, then 'a' as vowel
        // Vowels
        ('R', 'o', 'o') => Some(svara(Sthana::Murdha, Kala::Dirgha)),         // Roo = ṝ
        ('~', 'l', 'u') => Some(svara(Sthana::Danta, Kala::Hrasva)),          // ~lu = ḷ (vowel)
        // Consonant clusters
        ('c', 'h', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana)),  // chh = cha
        ('s', 'h', 'a') => None, // let match_two handle sh, then 'a' as vowel
        ('S', 'h', 'a') => None, // let match_two handle Sh, then 'a' as vowel
        _ => None,
    }
}

fn match_two(a: char, b: char) -> Option<Varna> {
    match (a, b) {
        // === Vowels (long forms) ===
        ('a', 'a') => Some(svara(Sthana::Kantha, Kala::Dirgha)),               // aa = ā
        ('e', 'e') => Some(svara(Sthana::Talu, Kala::Dirgha)),                 // ee = ī
        ('o', 'o') => Some(svara(Sthana::Oshtha, Kala::Dirgha)),               // oo = ū
        ('R', 'u') => Some(svara(Sthana::Murdha, Kala::Hrasva)),               // Ru = ṛ
        ('a', 'i') => Some(compound_svara(Sthana::KanthaTalu, Vivrti::Ativivrita)),  // ai
        ('a', 'u') => Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Ativivrita)), // au

        // === Aspirate consonants ===
        ('k', 'h') => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana)),   // kh
        ('g', 'h') => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana)),   // gh
        ('C', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana)),      // Ch = cha
        ('j', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana)),     // jh
        ('T', 'h') => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Mahaprana)),    // Th = ṭha
        ('D', 'h') => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Mahaprana)),   // Dh = ḍha
        ('t', 'h') => Some(sparsha(Sthana::Danta, Ghosha::Aghosha, Prana::Mahaprana)),     // th
        ('d', 'h') => Some(sparsha(Sthana::Danta, Ghosha::Saghosha, Prana::Mahaprana)),    // dh
        ('p', 'h') => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana)),    // ph
        ('b', 'h') => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana)),   // bh

        // === Palatal unaspirated ===
        ('c', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana)),      // ch = ca

        // === Sibilants ===
        ('s', 'h') => Some(ushman(Sthana::Talu)),     // sh = śa
        ('S', 'h') => Some(ushman(Sthana::Murdha)),    // Sh = ṣa

        // === Nasals with tilde ===
        ('~', 'g') => Some(nasal(Sthana::Kantha)),     // ~g = ṅa
        ('~', 'j') => Some(nasal(Sthana::Talu)),       // ~j = ña
        ('~', 'n') => Some(nasal(Sthana::Murdha)),     // ~n = ṇa (alternate)

        _ => None,
    }
}

fn match_one(c: char) -> Option<Varna> {
    match c {
        // === Vowels ===
        'a' => Some(svara(Sthana::Kantha, Kala::Hrasva)),
        'A' => Some(svara(Sthana::Kantha, Kala::Dirgha)),
        'i' => Some(svara(Sthana::Talu, Kala::Hrasva)),
        'I' => Some(svara(Sthana::Talu, Kala::Dirgha)),
        'u' => Some(svara(Sthana::Oshtha, Kala::Hrasva)),
        'U' => Some(svara(Sthana::Oshtha, Kala::Dirgha)),
        'e' | 'E' => Some(compound_svara(Sthana::KanthaTalu, Vivrti::Vivrita)),
        'o' | 'O' => Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Vivrita)),

        // === Unaspirated stops (aspirates handled in match_two) ===
        'k' => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Alpaprana)),
        'g' => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Alpaprana)),
        'j' => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Alpaprana)),
        'T' => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Alpaprana)),
        'D' => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Alpaprana)),
        'N' => Some(nasal(Sthana::Murdha)),
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
        'w' => Some(antahstha(Sthana::DantaOshtha)),   // w = va in some Baraha usage

        // === Ushman ===
        's' => Some(ushman(Sthana::Danta)),
        'h' => Some(Varna::Vyanjana {
            sthana: Sthana::Kantha,
            prayatna: Prayatna::Vivrita,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        }),

        // === Ayogavaha ===
        'M' => Some(Varna::Ayogavaha(AyogavahaType::Anusvara)),
        'H' => Some(Varna::Ayogavaha(AyogavahaType::Visarga)),

        // === Special ===
        'L' => Some(Varna::Vyanjana {  // La = ḷa (consonant)
            sthana: Sthana::Danta,
            prayatna: Prayatna::IshatSprshta,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        }),

        // Punctuation passthrough
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
        let v = parse("aAiIuU");
        assert_eq!(v.len(), 6);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Hrasva, .. }));
        assert!(matches!(v[1], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Dirgha, .. }));
        assert!(matches!(v[2], Varna::Svara { sthana: Sthana::Talu, kala: Kala::Hrasva, .. }));
        assert!(matches!(v[3], Varna::Svara { sthana: Sthana::Talu, kala: Kala::Dirgha, .. }));
    }

    #[test]
    fn test_long_vowel_digraphs() {
        let v = parse("aa");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Dirgha, .. }));

        let v = parse("ee");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Talu, kala: Kala::Dirgha, .. }));

        let v = parse("oo");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Oshtha, kala: Kala::Dirgha, .. }));
    }

    #[test]
    fn test_ri_vowel() {
        let v = parse("Ru");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Murdha, kala: Kala::Hrasva, .. }));

        let v = parse("Roo");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Murdha, kala: Kala::Dirgha, .. }));
    }

    #[test]
    fn test_compound_vowels() {
        let v = parse("ai");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaTalu, vivrti: Some(Vivrti::Ativivrita), .. }));

        let v = parse("au");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaOshtha, vivrti: Some(Vivrti::Ativivrita), .. }));
    }

    #[test]
    fn test_retroflex() {
        let v = parse("TaDa");
        assert_eq!(v.len(), 4);
        assert_eq!(v[0].sthana(), Some(Sthana::Murdha)); // Ta = ṭa
        assert_eq!(v[2].sthana(), Some(Sthana::Murdha)); // Da = ḍa
    }

    #[test]
    fn test_palatal_ch() {
        // ch = ca (unaspirated), Ch = cha (aspirated)
        let v = parse("ch");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Vyanjana {
            sthana: Sthana::Talu,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Alpaprana,
            ..
        }));

        let v = parse("Ch");
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Vyanjana {
            sthana: Sthana::Talu,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Mahaprana,
            ..
        }));
    }

    #[test]
    fn test_sibilants() {
        let v = parse("sh");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].sthana(), Some(Sthana::Talu));     // sh = śa
        assert!(v[0].is_ushman());

        let v = parse("Sh");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].sthana(), Some(Sthana::Murdha));    // Sh = ṣa

        let v = parse("s");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].sthana(), Some(Sthana::Danta));     // s = sa
    }

    #[test]
    fn test_tilde_nasals() {
        let v = parse("~ga~ja");
        assert_eq!(v.len(), 4);
        assert!(v[0].is_anunasika());
        assert_eq!(v[0].sthana(), Some(Sthana::Kantha));    // ~g = ṅa
        assert!(v[2].is_anunasika());
        assert_eq!(v[2].sthana(), Some(Sthana::Talu));      // ~j = ña
    }

    #[test]
    fn test_krsna() {
        // kRuShNa in Baraha
        let v = parse("kRuShNa");
        assert_eq!(v.len(), 5);
        assert_eq!(v[0].sthana(), Some(Sthana::Kantha));    // k
        assert!(matches!(v[1], Varna::Svara { sthana: Sthana::Murdha, .. })); // Ru = ṛ
        assert_eq!(v[2].sthana(), Some(Sthana::Murdha));    // Sh = ṣa
        assert!(v[3].is_anunasika());                        // N = ṇa
        assert!(matches!(v[4], Varna::Svara { sthana: Sthana::Kantha, .. })); // a
    }

    #[test]
    fn test_cross_format_with_iast() {
        use crate::parser::iast as iast_parser;
        use crate::encode;

        // Baraha "kRuShNa" and IAST "kṛṣṇa" should produce same bytes
        let baraha_varnas = parse("kRuShNa");
        let iast_varnas = iast_parser::parse("kṛṣṇa");

        let baraha_bytes = encode::encode(&baraha_varnas);
        let iast_bytes = encode::encode(&iast_varnas);

        assert_eq!(baraha_bytes, iast_bytes,
            "Baraha and IAST should produce identical PSS bytes for kṛṣṇa");
    }
}
