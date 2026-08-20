/// Baraha → Varna parser.
///
/// Baraha is an ASCII transliteration scheme popular in South Indian
/// computing contexts, extensively used for Vedic texts.
///
/// Basic conventions:
/// - Retroflex via uppercase: T Th D Dh N
/// - Nasals via tilde prefix: ~g (ṅ), ~j (ñ)
/// - ch = ca (palatal unaspirated), Ch/chh = cha (aspirated)
/// - sh = śa, Sh = ṣa, s = sa
/// - aa/A = ā, ee/I = ī, oo/U = ū
/// - Ru = ṛ, Roo = ṝ
/// - M = anusvara, H = visarga
///
/// Vedic svara accents (KYV/Taittiriya convention):
/// - q after vowel = anudatta (lowered pitch)
/// - # after vowel = svarita (falling pitch)
/// - $ after vowel = anudatta (phrase-final variant)
/// - unmarked = udatta
///
/// Vedic special conventions (from TS Baraha corpus):
/// - &  = avagraha (ऽ)
/// - && = double avagraha
/// - ,- = sandhi word-break (renders as space)
/// - (gm) = anusvara nasalization (gum-sound)
/// - ~M / ~m = chandrabindu (anunasika)
/// - ~g = ṅa, ~j = ña, ~n = ṇa (alternate)
/// - .  between consonants = conjunct separator (ignored)
/// - ^^ = ksha-type conjunct marker
/// - || = verse end (double danda ॥)
/// - [ ] = section markers (passthrough)

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

        // Try multi-char special sequences (longest match first)
        if let Some((v, consumed)) = match_special(&chars, i) {
            if let Some(varna) = v {
                varnas.push(varna);
            }
            // else: consumed but produces nothing (like ,- → space or . separator)
            i += consumed;
            apply_baraha_accent(&mut varnas, &chars, &mut i);
            continue;
        }

        // Try 3-char sequences
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

/// Match multi-char special sequences unique to Baraha Vedic encoding.
/// Returns (Option<Varna>, chars_consumed).
fn match_special(chars: &[char], i: usize) -> Option<(Option<Varna>, usize)> {
    let remaining = chars.len() - i;

    // 4-char: (gm) — anusvara nasalization
    if remaining >= 4 && chars[i] == '(' && chars[i+1] == 'g' && chars[i+2] == 'm' && chars[i+3] == ')' {
        return Some((Some(Varna::Ayogavaha(AyogavahaType::Anusvara)), 4));
    }

    // 2-char sequences
    if remaining >= 2 {
        match (chars[i], chars[i+1]) {
            // ,- = sandhi word-break → space
            (',', '-') => return Some((Some(Varna::Passthrough(' ')), 2)),
            // || = double danda
            ('|', '|') => return Some((Some(Varna::Passthrough('॥')), 2)),
            // && = double avagraha
            ('&', '&') => return Some((Some(Varna::Passthrough('ऽ')), 2)),
            // ^^ = special conjunct marker (ignored, let normal parsing handle)
            ('^', '^') => return Some((None, 2)),
            // ~M or ~m = chandrabindu
            ('~', 'M') | ('~', 'm') => return Some((Some(Varna::Ayogavaha(AyogavahaType::Chandrabindu)), 2)),
            // ~g = ṅa
            ('~', 'g') => return Some((Some(nasal(Sthana::Kantha)), 2)),
            // ~j = ña
            ('~', 'j') => return Some((Some(nasal(Sthana::Talu)), 2)),
            // ~n = ṇa (alternate)
            ('~', 'n') => return Some((Some(nasal(Sthana::Murdha)), 2)),
            _ => {}
        }
    }

    // 1-char specials
    match chars[i] {
        // & = avagraha
        '&' => Some((Some(Varna::Passthrough('ऽ')), 1)),
        // . between consonants = conjunct separator (skip it)
        '.' => Some((None, 1)),
        // ~ alone (not followed by g/j/n/M/m) = skip
        '~' => Some((None, 1)),
        _ => None,
    }
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
            '$' => SvaraPitch::Anudatta,
            _ => unreachable!(),
        };
        if let Some(Varna::Svara { pitch: ref mut p, .. }) = varnas.last_mut() {
            *p = Some(pitch);
        }
        *i += 1;
    }
}

fn match_three(a: char, b: char, c: char) -> Option<Varna> {
    match (a, b, c) {
        // Vowels
        ('R', 'o', 'o') => Some(svara(Sthana::Murdha, Kala::Dirgha)),         // Roo = ṝ
        // Consonant clusters
        ('c', 'h', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana)),  // chh = cha
        _ => None,
    }
}

fn match_two(a: char, b: char) -> Option<Varna> {
    match (a, b) {
        // === Vowels (long forms) ===
        ('a', 'a') => Some(svara(Sthana::Kantha, Kala::Dirgha)),               // aa = ā
        ('e', 'e') | ('E', 'E') => Some(svara(Sthana::Talu, Kala::Dirgha)),    // ee = ī
        ('o', 'o') | ('O', 'O') => Some(svara(Sthana::Oshtha, Kala::Dirgha)),  // oo = ū
        ('R', 'u') => Some(svara(Sthana::Murdha, Kala::Hrasva)),               // Ru = ṛ
        ('a', 'i') => Some(compound_svara(Sthana::KanthaTalu, Vivrti::Ativivrita)),  // ai
        ('a', 'u') => Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Ativivrita)), // au

        // === Aspirate consonants ===
        ('k', 'h') => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana)),
        ('g', 'h') => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana)),
        ('C', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana)),
        ('j', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana)),
        ('T', 'h') => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Mahaprana)),
        ('D', 'h') => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Mahaprana)),
        ('t', 'h') => Some(sparsha(Sthana::Danta, Ghosha::Aghosha, Prana::Mahaprana)),
        ('d', 'h') => Some(sparsha(Sthana::Danta, Ghosha::Saghosha, Prana::Mahaprana)),
        ('p', 'h') => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana)),
        ('b', 'h') => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana)),

        // === Palatal unaspirated ===
        ('c', 'h') => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana)),

        // === Sibilants ===
        ('s', 'h') => Some(ushman(Sthana::Talu)),     // sh = śa
        ('S', 'h') => Some(ushman(Sthana::Murdha)),    // Sh = ṣa

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

        // === Stops ===
        'k' => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Alpaprana)),
        'K' => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana)),  // K = kha
        'g' => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Alpaprana)),
        'G' => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana)),  // G = gha
        'c' => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana)),     // c alone = ca
        'C' => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana)),     // C alone = cha
        'j' => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Alpaprana)),
        'J' => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana)),    // J = jha
        'T' => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Alpaprana)),
        'D' => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Alpaprana)),
        'N' => Some(nasal(Sthana::Murdha)),
        't' => Some(sparsha(Sthana::Danta, Ghosha::Aghosha, Prana::Alpaprana)),
        'd' => Some(sparsha(Sthana::Danta, Ghosha::Saghosha, Prana::Alpaprana)),
        'n' => Some(nasal(Sthana::Danta)),
        'p' => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Alpaprana)),
        'P' => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana)),   // P = pha
        'b' => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Alpaprana)),
        'B' => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana)),  // B = bha
        'm' => Some(nasal(Sthana::Oshtha)),

        // === Antahstha ===
        'y' => Some(antahstha(Sthana::Talu)),
        'r' => Some(antahstha(Sthana::Murdha)),
        'l' => Some(antahstha(Sthana::Danta)),
        'v' => Some(antahstha(Sthana::DantaOshtha)),
        'w' => Some(antahstha(Sthana::DantaOshtha)),

        // === Ushman ===
        'S' => Some(ushman(Sthana::Talu)),     // S alone = śa (when not followed by 'h')
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

        // === Special consonants ===
        'L' => Some(Varna::Vyanjana {
            sthana: Sthana::Danta,
            prayatna: Prayatna::IshatSprshta,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        }),

        // === Passthrough ===
        ' ' | '\t' | '\n' | '\r' => Some(Varna::Passthrough(' ')),
        ',' => Some(Varna::Passthrough(' ')),  // comma = word separator in Vedic Baraha
        '(' | ')' | '[' | ']' => None,         // skip annotation brackets
        '|' => Some(Varna::Passthrough('।')),  // single pipe = danda
        '0'..='9' => Some(Varna::Passthrough(c)),
        '-' => None,                            // hyphen = word continuation, skip

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
        let v: Vec<_> = parse("aAiIuU").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 6);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Hrasva, .. }));
        assert!(matches!(v[1], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Dirgha, .. }));
    }

    #[test]
    fn test_long_vowel_digraphs() {
        let v: Vec<_> = parse("aa").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Kantha, kala: Kala::Dirgha, .. }));
    }

    #[test]
    fn test_ri_vowel() {
        let v: Vec<_> = parse("Ru").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::Murdha, kala: Kala::Hrasva, .. }));
    }

    #[test]
    fn test_compound_vowels() {
        let v: Vec<_> = parse("ai").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Svara { sthana: Sthana::KanthaTalu, vivrti: Some(Vivrti::Ativivrita), .. }));
    }

    #[test]
    fn test_palatal_ch() {
        let v: Vec<_> = parse("ch").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Vyanjana { sthana: Sthana::Talu, prana: Prana::Alpaprana, .. }));

        let v: Vec<_> = parse("Ch").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Varna::Vyanjana { sthana: Sthana::Talu, prana: Prana::Mahaprana, .. }));
    }

    #[test]
    fn test_sibilants() {
        let v = parse("sh");
        let v: Vec<_> = v.into_iter().filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].sthana(), Some(Sthana::Talu));
    }

    #[test]
    fn test_tilde_nasals() {
        let v: Vec<_> = parse("~ga~ja").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 4);
        assert!(v[0].is_anunasika());
        assert_eq!(v[0].sthana(), Some(Sthana::Kantha));
        assert!(v[2].is_anunasika());
        assert_eq!(v[2].sthana(), Some(Sthana::Talu));
    }

    #[test]
    fn test_krsna() {
        let v: Vec<_> = parse("kRuShNa").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_cross_format_with_iast() {
        use crate::parser::iast as iast_parser;
        use crate::encode;

        let baraha_varnas: Vec<_> = parse("kRuShNa").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        let iast_varnas: Vec<_> = iast_parser::parse("kṛṣṇa").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();

        let baraha_bytes = encode::encode(&baraha_varnas);
        let iast_bytes = encode::encode(&iast_varnas);

        assert_eq!(baraha_bytes, iast_bytes);
    }

    // === Vedic special convention tests ===

    #[test]
    fn test_sandhi_word_break() {
        // ,- = sandhi word-break → renders as space
        let v = parse("tvOq,-rjE");
        let spaces: Vec<_> = v.iter().filter(|v| matches!(v, Varna::Passthrough(' '))).collect();
        assert_eq!(spaces.len(), 1, ",- should produce a space");
    }

    #[test]
    fn test_avagraha() {
        let v = parse("&gnayE#");
        // & → avagraha (ऽ)
        assert!(matches!(v[0], Varna::Passthrough('ऽ')));
    }

    #[test]
    fn test_gm_anusvara() {
        // (gm) = anusvara nasalization
        let v: Vec<_> = parse("a(gm)").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert_eq!(v.len(), 2); // a + anusvara
        assert!(matches!(v[1], Varna::Ayogavaha(AyogavahaType::Anusvara)));
    }

    #[test]
    fn test_chandrabindu() {
        let v: Vec<_> = parse("~Mvi").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        assert!(matches!(v[0], Varna::Ayogavaha(AyogavahaType::Chandrabindu)));
    }

    #[test]
    fn test_double_danda() {
        let v = parse("pAhi || 1");
        let dandas: Vec<_> = v.iter().filter(|v| matches!(v, Varna::Passthrough('॥'))).collect();
        assert_eq!(dandas.len(), 1);
    }

    #[test]
    fn test_vedic_accents() {
        let v: Vec<_> = parse("iqShE tvA#").into_iter()
            .filter(|v| !matches!(v, Varna::Passthrough(_))).collect();
        // i(anudatta) Sh E t v A(svarita)
        assert!(matches!(v[0], Varna::Svara { pitch: Some(SvaraPitch::Anudatta), .. }));
        // tvA# → last svara 'A' should have svarita
        let last_svara = v.iter().rev().find(|v| v.is_svara()).unwrap();
        assert!(matches!(last_svara, Varna::Svara { pitch: Some(SvaraPitch::Svarita), .. }));
    }

    #[test]
    fn test_ts_1_1_1_full() {
        use crate::emitter::devanagari;
        let input = "iqShE tvOq,-rjE tvA#, vAqyava#H sthOpAqyava#H stha";
        let varnas = parse(input);
        let dev = devanagari::emit(&varnas);
        // Should have spaces, svaras, no stray commas/hyphens
        assert!(dev.contains(' '), "should have spaces");
        assert!(dev.contains('॒'), "should have anudatta marks");
        assert!(dev.contains('॑'), "should have svarita marks");
        assert!(!dev.contains(','), "commas should not appear in Devanagari");
        assert!(!dev.contains('-'), "hyphens should not appear in Devanagari");
    }
}
