/// SLP1 → Varna parser.
///
/// SLP1 (Sanskrit Library Phonetic 1) is the ideal starting parser because
/// it is a strict 1:1 ASCII mapping with no ambiguity. Every SLP1 character
/// maps to exactly one varna.
///
/// SLP1 spec: https://www.sanskrit-lexicon.uni-koeln.de/talkMay2008/SLP1.pdf

use crate::varna::*;

/// Parse an SLP1 string into a sequence of Varnas.
pub fn parse(input: &str) -> Vec<Varna> {
    let mut varnas = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let ch = bytes[i] as char;

        let v = match ch {
            // === Svaras ===
            'a' => Some(svara(Sthana::Kantha, Kala::Hrasva)),
            'A' => Some(svara(Sthana::Kantha, Kala::Dirgha)),
            'i' => Some(svara(Sthana::Talu, Kala::Hrasva)),
            'I' => Some(svara(Sthana::Talu, Kala::Dirgha)),
            'u' => Some(svara(Sthana::Oshtha, Kala::Hrasva)),
            'U' => Some(svara(Sthana::Oshtha, Kala::Dirgha)),
            'f' => Some(svara(Sthana::Murdha, Kala::Hrasva)),   // ṛ
            'F' => Some(svara(Sthana::Murdha, Kala::Dirgha)),   // ṝ
            'x' => Some(svara(Sthana::Danta, Kala::Hrasva)),    // ḷ (vowel)
            'X' => Some(svara(Sthana::Danta, Kala::Dirgha)),    // ḹ
            // e, ai, o, au: all dirgha. Distinguished by vivrti degree (PS.21).
            'e' => Some(compound_svara(Sthana::KanthaTalu, Vivrti::Vivrita)),
            'E' => Some(compound_svara(Sthana::KanthaTalu, Vivrti::Ativivrita)),   // ai
            'o' => Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Vivrita)),
            'O' => Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Ativivrita)), // au

            // === Sparsha (ka-varga) ===
            'k' => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Alpaprana)),
            'K' => Some(sparsha(Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana)),
            'g' => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Alpaprana)),
            'G' => Some(sparsha(Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana)),
            'N' => Some(nasal(Sthana::Kantha)),

            // === Sparsha (ca-varga) ===
            'c' => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana)),
            'C' => Some(sparsha(Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana)),
            'j' => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Alpaprana)),
            'J' => Some(sparsha(Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana)),
            'Y' => Some(nasal(Sthana::Talu)),

            // === Sparsha (ṭa-varga) ===
            'w' => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Alpaprana)),
            'W' => Some(sparsha(Sthana::Murdha, Ghosha::Aghosha, Prana::Mahaprana)),
            'q' => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Alpaprana)),
            'Q' => Some(sparsha(Sthana::Murdha, Ghosha::Saghosha, Prana::Mahaprana)),
            'R' => Some(nasal(Sthana::Murdha)),

            // === Sparsha (ta-varga) ===
            't' => Some(sparsha(Sthana::Danta, Ghosha::Aghosha, Prana::Alpaprana)),
            'T' => Some(sparsha(Sthana::Danta, Ghosha::Aghosha, Prana::Mahaprana)),
            'd' => Some(sparsha(Sthana::Danta, Ghosha::Saghosha, Prana::Alpaprana)),
            'D' => Some(sparsha(Sthana::Danta, Ghosha::Saghosha, Prana::Mahaprana)),
            'n' => Some(nasal(Sthana::Danta)),

            // === Sparsha (pa-varga) ===
            'p' => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Alpaprana)),
            'P' => Some(sparsha(Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana)),
            'b' => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Alpaprana)),
            'B' => Some(sparsha(Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana)),
            'm' => Some(nasal(Sthana::Oshtha)),

            // === Antahstha (PS.38 ishat-sprshta) ===
            'y' => Some(antahstha(Sthana::Talu)),
            'r' => Some(antahstha(Sthana::Murdha)),
            'l' => Some(antahstha(Sthana::Danta)),
            'v' => Some(antahstha(Sthana::DantaOshtha)),

            // === Ushman (PS.38 vivrita) ===
            'S' => Some(ushman(Sthana::Talu)),     // śa
            'z' => Some(ushman(Sthana::Murdha)),    // ṣa
            's' => Some(ushman(Sthana::Danta)),     // sa
            'h' => Some(Varna::Vyanjana {
                sthana: Sthana::Kantha,
                prayatna: Prayatna::Vivrita,
                ghosha: Ghosha::Saghosha,           // ha is saghosha (PS.39: nādinō ha)
                prana: Prana::Alpaprana,
                nasika: false,
            }),

            // === Ayogavaha (PS.5) ===
            'M' => Some(Varna::Ayogavaha(AyogavahaType::Anusvara)),
            'H' => Some(Varna::Ayogavaha(AyogavahaType::Visarga)),
            '~' => Some(Varna::Ayogavaha(AyogavahaType::Chandrabindu)),

            // Punctuation/whitespace passthrough
            ' ' | '\t' | '\n' | '\r' | ',' | '-' | '.' | '|' | '(' | ')' | '[' | ']'
            | ';' | '!' | '/' | '\'' | '"' | '0'..='9' => Some(Varna::Passthrough(ch)),

            _ => None,
        };

        if let Some(v) = v {
            varnas.push(v);
        }

        i += 1;
    }

    varnas
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
    fn test_ka_varga() {
        let varnas = parse("kKgGN");
        assert_eq!(varnas.len(), 5);

        // ka: kantha, sprshta, aghosha, alpaprana
        assert!(matches!(varnas[0], Varna::Vyanjana {
            sthana: Sthana::Kantha,
            prayatna: Prayatna::Sprshta,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        }));

        // nga: kantha, sprshta, saghosha, alpaprana, nasika
        assert!(matches!(varnas[4], Varna::Vyanjana {
            sthana: Sthana::Kantha,
            nasika: true,
            ..
        }));
    }

    #[test]
    fn test_svaras() {
        let varnas = parse("aAiIuU");
        assert_eq!(varnas.len(), 6);

        assert!(matches!(varnas[0], Varna::Svara {
            sthana: Sthana::Kantha,
            kala: Kala::Hrasva,
            ..
        }));

        assert!(matches!(varnas[3], Varna::Svara {
            sthana: Sthana::Talu,
            kala: Kala::Dirgha,
            ..
        }));
    }

    #[test]
    fn test_agni() {
        // "agni" in SLP1
        let varnas = parse("agni");
        assert_eq!(varnas.len(), 4);
        // a
        assert!(varnas[0].is_svara());
        // g
        assert!(varnas[1].is_sparsha());
        // n
        assert!(varnas[2].is_anunasika());
        // i
        assert!(varnas[3].is_svara());
    }
}
