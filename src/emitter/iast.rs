/// Varna → IAST emitter.

use crate::varna::*;

/// Emit a sequence of Varnas as an IAST string.
pub fn emit(varnas: &[Varna]) -> String {
    let mut out = String::with_capacity(varnas.len() * 2);

    for v in varnas {
        match v {
            // Svaras
            Varna::Svara { sthana, kala, vivrti, .. } => {
                match (sthana, vivrti) {
                    // Compound vowels: distinguished by vivrti
                    (Sthana::KanthaTalu, Some(Vivrti::Ativivrita)) => out.push_str("ai"),
                    (Sthana::KanthaTalu, _) => out.push('e'),
                    (Sthana::KanthaOshtha, Some(Vivrti::Ativivrita)) => out.push_str("au"),
                    (Sthana::KanthaOshtha, _) => out.push('o'),
                    // Simple vowels: distinguished by kala
                    _ => {
                        let s = match (sthana, kala) {
                            (Sthana::Kantha, Kala::Hrasva) => "a",
                            (Sthana::Kantha, _) => "ā",
                            (Sthana::Talu, Kala::Hrasva) => "i",
                            (Sthana::Talu, _) => "ī",
                            (Sthana::Oshtha, Kala::Hrasva) => "u",
                            (Sthana::Oshtha, _) => "ū",
                            (Sthana::Murdha, Kala::Hrasva) => "ṛ",
                            (Sthana::Murdha, _) => "ṝ",
                            (Sthana::Danta, Kala::Hrasva) => "ḷ",
                            (Sthana::Danta, _) => "ḹ",
                            _ => continue,
                        };
                        out.push_str(s);
                    }
                }
            }

            // Vyanjanas
            Varna::Vyanjana { sthana, prayatna, ghosha, prana, nasika } => {
                let s = match prayatna {
                    Prayatna::Sprshta => {
                        if *nasika {
                            match sthana {
                                Sthana::Kantha => "ṅ",
                                Sthana::Talu => "ñ",
                                Sthana::Murdha => "ṇ",
                                Sthana::Danta => "n",
                                Sthana::Oshtha => "m",
                                _ => continue,
                            }
                        } else {
                            match (sthana, ghosha, prana) {
                                (Sthana::Kantha, Ghosha::Aghosha, Prana::Alpaprana) => "k",
                                (Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana) => "kh",
                                (Sthana::Kantha, Ghosha::Saghosha, Prana::Alpaprana) => "g",
                                (Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana) => "gh",
                                (Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana) => "c",
                                (Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana) => "ch",
                                (Sthana::Talu, Ghosha::Saghosha, Prana::Alpaprana) => "j",
                                (Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana) => "jh",
                                (Sthana::Murdha, Ghosha::Aghosha, Prana::Alpaprana) => "ṭ",
                                (Sthana::Murdha, Ghosha::Aghosha, Prana::Mahaprana) => "ṭh",
                                (Sthana::Murdha, Ghosha::Saghosha, Prana::Alpaprana) => "ḍ",
                                (Sthana::Murdha, Ghosha::Saghosha, Prana::Mahaprana) => "ḍh",
                                (Sthana::Danta, Ghosha::Aghosha, Prana::Alpaprana) => "t",
                                (Sthana::Danta, Ghosha::Aghosha, Prana::Mahaprana) => "th",
                                (Sthana::Danta, Ghosha::Saghosha, Prana::Alpaprana) => "d",
                                (Sthana::Danta, Ghosha::Saghosha, Prana::Mahaprana) => "dh",
                                (Sthana::Oshtha, Ghosha::Aghosha, Prana::Alpaprana) => "p",
                                (Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana) => "ph",
                                (Sthana::Oshtha, Ghosha::Saghosha, Prana::Alpaprana) => "b",
                                (Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana) => "bh",
                                _ => continue,
                            }
                        }
                    }
                    Prayatna::IshatSprshta => match sthana {
                        Sthana::Talu => "y",
                        Sthana::Murdha => "r",
                        Sthana::Danta => "l",
                        Sthana::DantaOshtha => "v",
                        _ => continue,
                    },
                    Prayatna::Vivrita => match sthana {
                        Sthana::Talu => "ś",
                        Sthana::Murdha => "ṣ",
                        Sthana::Danta => "s",
                        Sthana::Kantha => "h",
                        _ => continue,
                    },
                };
                out.push_str(s);
            }

            // Ayogavaha
            Varna::Ayogavaha(typ) => {
                let s = match typ {
                    AyogavahaType::Anusvara => "ṃ",
                    AyogavahaType::Visarga => "ḥ",
                    AyogavahaType::Chandrabindu => "m̐",
                    AyogavahaType::Jihvamuliya | AyogavahaType::Upadhmaniya => continue,
                };
                out.push_str(s);
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::iast::parse;

    #[test]
    fn test_roundtrip_simple() {
        let cases = vec![
            "agni",
            "kṛṣṇa",
            "dharma",
            "śiva",
            "viṣṇu",
            "saṃskṛtam",
            "rāmaḥ",
            "brāhmaṇa",
        ];
        for input in cases {
            let varnas = parse(input);
            let output = emit(&varnas);
            assert_eq!(output, input, "roundtrip failed for: {}", input);
        }
    }

    #[test]
    fn test_roundtrip_aspirates() {
        let cases = vec![
            "kha",
            "gha",
            "cha",
            "jha",
            "ṭha",
            "ḍha",
            "tha",
            "dha",
            "pha",
            "bha",
        ];
        for input in cases {
            let varnas = parse(input);
            let output = emit(&varnas);
            assert_eq!(output, input, "roundtrip failed for: {}", input);
        }
    }

    #[test]
    fn test_roundtrip_compound_vowels() {
        let cases = vec!["ai", "au", "e", "o"];
        for input in cases {
            let varnas = parse(input);
            let output = emit(&varnas);
            assert_eq!(output, input, "roundtrip failed for: {}", input);
        }
    }

    #[test]
    fn test_roundtrip_full_verse() {
        let input = "agnimīḷe purohitam";
        let varnas = parse(input);
        let output = emit(&varnas);
        // Whitespace stripped by parser
        assert_eq!(output, "agnimīḷepurohitam");
    }
}
