/// PSS byte decoding — byte values back to Varna structs.

use crate::encode::*;
use crate::varna::*;

/// Decode a single PSS byte to a Varna.
/// Returns None for accent overlays, structural markers, or invalid bytes.
pub fn decode_varna(byte: u8) -> Option<Varna> {
    // Ayogavaha block
    match byte {
        ANUSVARA => return Some(Varna::Ayogavaha(AyogavahaType::Anusvara)),
        VISARGA => return Some(Varna::Ayogavaha(AyogavahaType::Visarga)),
        JIHVAMULIYA => return Some(Varna::Ayogavaha(AyogavahaType::Jihvamuliya)),
        UPADHMANIYA => return Some(Varna::Ayogavaha(AyogavahaType::Upadhmaniya)),
        CHANDRABINDU => return Some(Varna::Ayogavaha(AyogavahaType::Chandrabindu)),
        _ => {}
    }

    // Svara block (0x40–0x4D)
    match byte {
        A_SHORT => return Some(svara(Sthana::Kantha, Kala::Hrasva)),
        A_LONG => return Some(svara(Sthana::Kantha, Kala::Dirgha)),
        I_SHORT => return Some(svara(Sthana::Talu, Kala::Hrasva)),
        I_LONG => return Some(svara(Sthana::Talu, Kala::Dirgha)),
        U_SHORT => return Some(svara(Sthana::Oshtha, Kala::Hrasva)),
        U_LONG => return Some(svara(Sthana::Oshtha, Kala::Dirgha)),
        R_SHORT => return Some(svara(Sthana::Murdha, Kala::Hrasva)),
        R_LONG => return Some(svara(Sthana::Murdha, Kala::Dirgha)),
        L_SHORT => return Some(svara(Sthana::Danta, Kala::Hrasva)),
        L_LONG => return Some(svara(Sthana::Danta, Kala::Dirgha)),
        // Compound vowels: all dirgha, distinguished by vivrti (PS.21)
        E => return Some(compound_svara(Sthana::KanthaTalu, Vivrti::Vivrita)),
        AI => return Some(compound_svara(Sthana::KanthaTalu, Vivrti::Ativivrita)),
        O => return Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Vivrita)),
        AU => return Some(compound_svara(Sthana::KanthaOshtha, Vivrti::Ativivrita)),
        _ => {}
    }

    // Vyanjana block (0x00–0x21)
    if byte <= LLA {
        return Some(decode_vyanjana(byte));
    }

    // Accent overlays and structural markers return None
    None
}

/// Returns true if this byte is an accent overlay.
pub fn is_accent(byte: u8) -> bool {
    matches!(byte, UDATTA | ANUDATTA | SVARITA | DEPENDENT_SVARITA | DIRGHA_SVARITA | PRACAYA)
}

/// Returns true if this byte is a structural boundary.
pub fn is_boundary(byte: u8) -> bool {
    matches!(byte, PADA_BOUNDARY | VAKYA_BOUNDARY | AVAGRAHA)
}

/// Decode a full PSS byte sequence to Varnas, applying accent overlays.
pub fn decode(bytes: &[u8]) -> Vec<Varna> {
    let mut varnas = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes[i];

        if is_boundary(byte) || is_accent(byte) {
            i += 1;
            continue;
        }

        if let Some(mut v) = decode_varna(byte) {
            // Check if next byte is an accent overlay
            if let Varna::Svara { ref mut pitch, .. } = v {
                if i + 1 < bytes.len() {
                    match bytes[i + 1] {
                        UDATTA => { *pitch = Some(SvaraPitch::Udatta); i += 1; }
                        ANUDATTA => { *pitch = Some(SvaraPitch::Anudatta); i += 1; }
                        SVARITA => { *pitch = Some(SvaraPitch::Svarita); i += 1; }
                        DEPENDENT_SVARITA => { *pitch = Some(SvaraPitch::DependentSvarita); i += 1; }
                        DIRGHA_SVARITA => { *pitch = Some(SvaraPitch::DirghaSvarita); i += 1; }
                        PRACAYA => { *pitch = Some(SvaraPitch::Pracaya); i += 1; }
                        _ => {}
                    }
                }
            }
            varnas.push(v);
        }

        i += 1;
    }

    varnas
}

fn svara(sthana: Sthana, kala: Kala) -> Varna {
    Varna::Svara { sthana, kala, vivrti: None, pitch: None }
}

fn compound_svara(sthana: Sthana, vivrti: Vivrti) -> Varna {
    Varna::Svara { sthana, kala: Kala::Dirgha, vivrti: Some(vivrti), pitch: None }
}

fn decode_vyanjana(byte: u8) -> Varna {
    // Sparsha consonants: 0x00–0x18 (5 vargas × 5)
    if byte <= MA {
        let varga = byte / 5;
        let position = byte % 5;

        let sthana = match varga {
            0 => Sthana::Kantha,
            1 => Sthana::Talu,
            2 => Sthana::Murdha,
            3 => Sthana::Danta,
            4 => Sthana::Oshtha,
            _ => unreachable!(),
        };

        let (ghosha, prana, nasika) = match position {
            0 => (Ghosha::Aghosha, Prana::Alpaprana, false),
            1 => (Ghosha::Aghosha, Prana::Mahaprana, false),
            2 => (Ghosha::Saghosha, Prana::Alpaprana, false),
            3 => (Ghosha::Saghosha, Prana::Mahaprana, false),
            4 => (Ghosha::Saghosha, Prana::Alpaprana, true),
            _ => unreachable!(),
        };

        return Varna::Vyanjana {
            sthana,
            prayatna: Prayatna::Sprshta,
            ghosha,
            prana,
            nasika,
        };
    }

    // Antahstha: 0x19–0x1C
    match byte {
        YA => Varna::Vyanjana {
            sthana: Sthana::Talu,
            prayatna: Prayatna::IshatSprshta,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        RA => Varna::Vyanjana {
            sthana: Sthana::Murdha,
            prayatna: Prayatna::IshatSprshta,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        LA => Varna::Vyanjana {
            sthana: Sthana::Danta,
            prayatna: Prayatna::IshatSprshta,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        VA => Varna::Vyanjana {
            sthana: Sthana::DantaOshtha,
            prayatna: Prayatna::IshatSprshta,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        // Ushman: 0x1D–0x20
        SHA => Varna::Vyanjana {
            sthana: Sthana::Talu,
            prayatna: Prayatna::Vivrita,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        SSA => Varna::Vyanjana {
            sthana: Sthana::Murdha,
            prayatna: Prayatna::Vivrita,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        SA => Varna::Vyanjana {
            sthana: Sthana::Danta,
            prayatna: Prayatna::Vivrita,
            ghosha: Ghosha::Aghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        HA => Varna::Vyanjana {
            sthana: Sthana::Kantha,
            prayatna: Prayatna::Vivrita,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        // Duhsprshta ḷa
        LLA => Varna::Vyanjana {
            sthana: Sthana::Danta,
            prayatna: Prayatna::IshatSprshta,
            ghosha: Ghosha::Saghosha,
            prana: Prana::Alpaprana,
            nasika: false,
        },
        _ => unreachable!("invalid vyanjana byte: 0x{:02X}", byte),
    }
}
