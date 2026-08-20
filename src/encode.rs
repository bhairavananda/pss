/// PSS byte encoding.
///
/// Maps each Varna to a unique byte value. The byte assignments follow the
/// canonical ordering of the Paniniya Shiksha: vargas in sthana order,
/// then antahstha, then ushman, then svaras.

use crate::varna::*;

// === Vyanjana byte values (0x00–0x25) ===

// Ka-varga (kantha, sprshta) — PS.17: "kaṇṭhyāvahāv"
pub const KA: u8 = 0x00;
pub const KHA: u8 = 0x01;
pub const GA: u8 = 0x02;
pub const GHA: u8 = 0x03;
pub const NGA: u8 = 0x04;

// Ca-varga (talu, sprshta) — PS.17: "ichuyaśāstālavyā"
pub const CA: u8 = 0x05;
pub const CHA: u8 = 0x06;
pub const JA: u8 = 0x07;
pub const JHA: u8 = 0x08;
pub const NYA: u8 = 0x09;

// Ta-varga (murdha, sprshta) — PS.17: "syurmūrdhanyā ṛṭuraṣā"
pub const TTA: u8 = 0x0A;
pub const TTHA: u8 = 0x0B;
pub const DDA: u8 = 0x0C;
pub const DDHA: u8 = 0x0D;
pub const NNA: u8 = 0x0E;

// ta-varga (danta, sprshta) — PS.17: "dantyā ḷitulasāḥ"
pub const TA: u8 = 0x0F;
pub const THA: u8 = 0x10;
pub const DA: u8 = 0x11;
pub const DHA: u8 = 0x12;
pub const NA: u8 = 0x13;

// Pa-varga (oshtha, sprshta) — PS.17: "ōṣṭhajāvupū"
pub const PA: u8 = 0x14;
pub const PHA: u8 = 0x15;
pub const BA: u8 = 0x16;
pub const BHA: u8 = 0x17;
pub const MA: u8 = 0x18;

// Antahstha (ishat-sprshta) — PS.38: "yaṇastvīṣannēmaspṛṣṭāḥ"
pub const YA: u8 = 0x19;
pub const RA: u8 = 0x1A;
pub const LA: u8 = 0x1B;
pub const VA: u8 = 0x1C;

// Ushman (vivrita) — PS.38: "śalaḥ smṛtāḥ"
pub const SHA: u8 = 0x1D;  // śa (talu)
pub const SSA: u8 = 0x1E;  // ṣa (murdha)
pub const SA: u8 = 0x1F;   // sa (danta)
pub const HA: u8 = 0x20;   // ha (kantha)

// Additional consonants
pub const LLA: u8 = 0x21;  // ḷa (duhsprshta, PS.5)

// === Svara byte values (0x40–0x4D) ===

pub const A_SHORT: u8 = 0x40;   // a (kantha, hrasva)
pub const A_LONG: u8 = 0x41;    // ā (kantha, dirgha)
pub const I_SHORT: u8 = 0x42;   // i (talu, hrasva)
pub const I_LONG: u8 = 0x43;    // ī (talu, dirgha)
pub const U_SHORT: u8 = 0x44;   // u (oshtha, hrasva)
pub const U_LONG: u8 = 0x45;    // ū (oshtha, dirgha)
pub const R_SHORT: u8 = 0x46;   // ṛ (murdha, hrasva)
pub const R_LONG: u8 = 0x47;    // ṝ (murdha, dirgha)
pub const L_SHORT: u8 = 0x48;   // ḷ (danta, hrasva)
pub const L_LONG: u8 = 0x49;    // ḹ (danta, dirgha)
pub const E: u8 = 0x4A;         // e (kantha-talu, dirgha)
pub const AI: u8 = 0x4B;        // ai (kantha-talu, dirgha — PS.21 "vivṛtāvēṅau")
pub const O: u8 = 0x4C;         // o (kantha-oshtha, dirgha)
pub const AU: u8 = 0x4D;        // au (kantha-oshtha, dirgha)

// === Accent overlay (0x60–0x62) ===
// Applied after a svara byte to mark pitch (PS.11)

pub const UDATTA: u8 = 0x60;
pub const ANUDATTA: u8 = 0x61;
pub const SVARITA: u8 = 0x62;

// === Ayogavaha (0x70–0x74) — PS.5 ===

pub const ANUSVARA: u8 = 0x70;
pub const VISARGA: u8 = 0x71;
pub const JIHVAMULIYA: u8 = 0x72;
pub const UPADHMANIYA: u8 = 0x73;
pub const CHANDRABINDU: u8 = 0x74;

// === Structural markers ===

pub const AVAGRAHA: u8 = 0xFD;
pub const PADA_BOUNDARY: u8 = 0xFE;
pub const VAKYA_BOUNDARY: u8 = 0xFF;

/// Encode a Varna to its PSS byte value.
pub fn encode_varna(v: &Varna) -> u8 {
    match v {
        Varna::Svara { sthana, kala, .. } => encode_svara(*sthana, *kala),
        Varna::Vyanjana { sthana, prayatna, ghosha, prana, nasika } => {
            encode_vyanjana(*sthana, *prayatna, *ghosha, *prana, *nasika)
        }
        Varna::Ayogavaha(typ) => match typ {
            AyogavahaType::Anusvara => ANUSVARA,
            AyogavahaType::Visarga => VISARGA,
            AyogavahaType::Jihvamuliya => JIHVAMULIYA,
            AyogavahaType::Upadhmaniya => UPADHMANIYA,
            AyogavahaType::Chandrabindu => CHANDRABINDU,
        },
    }
}

fn encode_svara(sthana: Sthana, kala: Kala) -> u8 {
    match (sthana, kala) {
        (Sthana::Kantha, Kala::Hrasva) => A_SHORT,
        (Sthana::Kantha, Kala::Dirgha) => A_LONG,
        (Sthana::Talu, Kala::Hrasva) => I_SHORT,
        (Sthana::Talu, Kala::Dirgha) => I_LONG,
        (Sthana::Oshtha, Kala::Hrasva) => U_SHORT,
        (Sthana::Oshtha, Kala::Dirgha) => U_LONG,
        (Sthana::Murdha, Kala::Hrasva) => R_SHORT,
        (Sthana::Murdha, Kala::Dirgha) => R_LONG,
        (Sthana::Danta, Kala::Hrasva) => L_SHORT,
        (Sthana::Danta, Kala::Dirgha) => L_LONG,
        (Sthana::KanthaTalu, Kala::Dirgha) => E,
        (Sthana::KanthaTalu, Kala::Hrasva) => AI, // ai is "more vivrita" (PS.21)
        (Sthana::KanthaOshtha, Kala::Dirgha) => O,
        (Sthana::KanthaOshtha, Kala::Hrasva) => AU,
        // Pluta forms: use the dirgha byte + a pluta marker would be needed,
        // but for now encode same as dirgha (pluta is rare in text)
        (s, Kala::Pluta) => encode_svara(s, Kala::Dirgha),
        _ => unreachable!("invalid sthana/kala combination for svara"),
    }
}

fn encode_vyanjana(
    sthana: Sthana,
    prayatna: Prayatna,
    ghosha: Ghosha,
    prana: Prana,
    nasika: bool,
) -> u8 {
    match prayatna {
        Prayatna::Sprshta => {
            // Sparsha consonants: 5 vargas × 5 positions
            let varga_base = match sthana {
                Sthana::Kantha => 0x00,
                Sthana::Talu => 0x05,
                Sthana::Murdha => 0x0A,
                Sthana::Danta => 0x0F,
                Sthana::Oshtha => 0x14,
                _ => unreachable!("invalid sthana for sparsha"),
            };
            if nasika {
                // 5th of varga (anunasika)
                varga_base + 4
            } else {
                let offset = match (ghosha, prana) {
                    (Ghosha::Aghosha, Prana::Alpaprana) => 0, // 1st
                    (Ghosha::Aghosha, Prana::Mahaprana) => 1, // 2nd
                    (Ghosha::Saghosha, Prana::Alpaprana) => 2, // 3rd
                    (Ghosha::Saghosha, Prana::Mahaprana) => 3, // 4th
                };
                varga_base + offset
            }
        }
        Prayatna::IshatSprshta => {
            // Antahstha: ya ra la va
            match sthana {
                Sthana::Talu => YA,
                Sthana::Murdha => RA,
                Sthana::Danta => LA,
                Sthana::DantaOshtha => VA,
                _ => unreachable!("invalid sthana for antahstha"),
            }
        }
        Prayatna::Vivrita => {
            // Ushman: śa ṣa sa ha
            match sthana {
                Sthana::Talu => SHA,
                Sthana::Murdha => SSA,
                Sthana::Danta => SA,
                Sthana::Kantha => HA,
                _ => unreachable!("invalid sthana for ushman"),
            }
        }
    }
}

/// Encode a sequence of Varnas to PSS bytes.
pub fn encode(varnas: &[Varna]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(varnas.len());
    for v in varnas {
        bytes.push(encode_varna(v));
        // Append accent overlay if present
        if let Varna::Svara { pitch: Some(p), .. } = v {
            bytes.push(match p {
                SvaraPitch::Udatta => UDATTA,
                SvaraPitch::Anudatta => ANUDATTA,
                SvaraPitch::Svarita => SVARITA,
            });
        }
    }
    bytes
}
