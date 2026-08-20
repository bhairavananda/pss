/// Varna → SLP1 emitter.

use crate::varna::*;

/// Emit a sequence of Varnas as an SLP1 string.
pub fn emit(varnas: &[Varna]) -> String {
    let mut out = String::with_capacity(varnas.len());

    for v in varnas {
        let ch = match v {
            // Svaras
            Varna::Svara { sthana, kala, .. } => match (sthana, kala) {
                (Sthana::Kantha, Kala::Hrasva) => 'a',
                (Sthana::Kantha, Kala::Dirgha | Kala::Pluta) => 'A',
                (Sthana::Talu, Kala::Hrasva) => 'i',
                (Sthana::Talu, Kala::Dirgha | Kala::Pluta) => 'I',
                (Sthana::Oshtha, Kala::Hrasva) => 'u',
                (Sthana::Oshtha, Kala::Dirgha | Kala::Pluta) => 'U',
                (Sthana::Murdha, Kala::Hrasva) => 'f',
                (Sthana::Murdha, Kala::Dirgha | Kala::Pluta) => 'F',
                (Sthana::Danta, Kala::Hrasva) => 'x',
                (Sthana::Danta, Kala::Dirgha | Kala::Pluta) => 'X',
                (Sthana::KanthaTalu, Kala::Dirgha) => 'e',
                (Sthana::KanthaTalu, Kala::Hrasva | Kala::Pluta) => 'E',
                (Sthana::KanthaOshtha, Kala::Dirgha) => 'o',
                (Sthana::KanthaOshtha, Kala::Hrasva | Kala::Pluta) => 'O',
                _ => continue,
            },

            // Vyanjanas
            Varna::Vyanjana { sthana, prayatna, ghosha, prana, nasika } => {
                match prayatna {
                    Prayatna::Sprshta => {
                        if *nasika {
                            match sthana {
                                Sthana::Kantha => 'N',
                                Sthana::Talu => 'Y',
                                Sthana::Murdha => 'R',
                                Sthana::Danta => 'n',
                                Sthana::Oshtha => 'm',
                                _ => continue,
                            }
                        } else {
                            match (sthana, ghosha, prana) {
                                (Sthana::Kantha, Ghosha::Aghosha, Prana::Alpaprana) => 'k',
                                (Sthana::Kantha, Ghosha::Aghosha, Prana::Mahaprana) => 'K',
                                (Sthana::Kantha, Ghosha::Saghosha, Prana::Alpaprana) => 'g',
                                (Sthana::Kantha, Ghosha::Saghosha, Prana::Mahaprana) => 'G',
                                (Sthana::Talu, Ghosha::Aghosha, Prana::Alpaprana) => 'c',
                                (Sthana::Talu, Ghosha::Aghosha, Prana::Mahaprana) => 'C',
                                (Sthana::Talu, Ghosha::Saghosha, Prana::Alpaprana) => 'j',
                                (Sthana::Talu, Ghosha::Saghosha, Prana::Mahaprana) => 'J',
                                (Sthana::Murdha, Ghosha::Aghosha, Prana::Alpaprana) => 'w',
                                (Sthana::Murdha, Ghosha::Aghosha, Prana::Mahaprana) => 'W',
                                (Sthana::Murdha, Ghosha::Saghosha, Prana::Alpaprana) => 'q',
                                (Sthana::Murdha, Ghosha::Saghosha, Prana::Mahaprana) => 'Q',
                                (Sthana::Danta, Ghosha::Aghosha, Prana::Alpaprana) => 't',
                                (Sthana::Danta, Ghosha::Aghosha, Prana::Mahaprana) => 'T',
                                (Sthana::Danta, Ghosha::Saghosha, Prana::Alpaprana) => 'd',
                                (Sthana::Danta, Ghosha::Saghosha, Prana::Mahaprana) => 'D',
                                (Sthana::Oshtha, Ghosha::Aghosha, Prana::Alpaprana) => 'p',
                                (Sthana::Oshtha, Ghosha::Aghosha, Prana::Mahaprana) => 'P',
                                (Sthana::Oshtha, Ghosha::Saghosha, Prana::Alpaprana) => 'b',
                                (Sthana::Oshtha, Ghosha::Saghosha, Prana::Mahaprana) => 'B',
                                _ => continue,
                            }
                        }
                    }
                    Prayatna::IshatSprshta => match sthana {
                        Sthana::Talu => 'y',
                        Sthana::Murdha => 'r',
                        Sthana::Danta => 'l',
                        Sthana::DantaOshtha => 'v',
                        _ => continue,
                    },
                    Prayatna::Vivrita => match sthana {
                        Sthana::Talu => 'S',
                        Sthana::Murdha => 'z',
                        Sthana::Danta => 's',
                        Sthana::Kantha => 'h',
                        _ => continue,
                    },
                }
            }

            // Ayogavaha
            Varna::Ayogavaha(typ) => match typ {
                AyogavahaType::Anusvara => 'M',
                AyogavahaType::Visarga => 'H',
                AyogavahaType::Chandrabindu => '~',
                AyogavahaType::Jihvamuliya | AyogavahaType::Upadhmaniya => continue,
            },
        };

        out.push(ch);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::slp1::parse;

    #[test]
    fn test_roundtrip() {
        let inputs = vec![
            "agnimIqe purohitam",
            "Darmaraje",
            "kfzRa",
            "saMskftam",
            "BagavadgItA",
        ];

        for input in inputs {
            let varnas = parse(input);
            let output = emit(&varnas);
            // Whitespace is stripped by parser, so compare without spaces
            let expected: String = input.chars().filter(|c| !c.is_whitespace()).collect();
            assert_eq!(output, expected, "roundtrip failed for: {}", input);
        }
    }
}
