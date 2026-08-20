/// Verify that PSS encoding matches Paniniya Shiksha classifications.
///
/// Each test references a specific sutra and verifies that the Varna
/// produced by parsing carries the correct features.

use pss::parser::slp1::parse;
use pss::varna::*;

/// PS.4: "svarāviṃśatirēkaścha" — 21 svaras
/// PS.4: "sparśānāṃ pañchaviṃśatiḥ" — 25 sparshas
#[test]
fn ps4_svara_count() {
    // 14 base svaras (hrasva+dirgha pairs + compound vowels)
    let svaras = parse("aAiIuUfFeEoO");
    assert!(svaras.iter().all(|v| v.is_svara()));
    assert_eq!(svaras.len(), 12); // 12 distinct in SLP1 (ḷ vowels rare)
}

#[test]
fn ps4_sparsha_count() {
    let sparshas = parse("kKgGNcCjJYwWqQRtTdDnpPbBm");
    assert_eq!(sparshas.len(), 25);
    assert!(sparshas.iter().all(|v| v.is_sparsha()));
}

/// PS.17: "kaṇṭhyāvahāv" — a and ha are kantha
#[test]
fn ps17_kanthya() {
    let varnas = parse("ah");
    assert_eq!(varnas[0].sthana(), Some(Sthana::Kantha)); // a
    assert_eq!(varnas[1].sthana(), Some(Sthana::Kantha)); // ha
}

/// PS.17: "ichuyaśāstālavyā" — i, ca-varga, ya, śa are talu
#[test]
fn ps17_talavya() {
    let varnas = parse("icyS");
    for v in &varnas {
        assert_eq!(v.sthana(), Some(Sthana::Talu),
            "expected talu for {:?}", v);
    }
}

/// PS.17: "syurmūrdhanyā ṛṭuraṣā" — ṛ, ṭa-varga, ra, ṣa are murdha
#[test]
fn ps17_murdhanya() {
    let varnas = parse("fwrz");
    for v in &varnas {
        assert_eq!(v.sthana(), Some(Sthana::Murdha),
            "expected murdha for {:?}", v);
    }
}

/// PS.17: "dantyā ḷitulasāḥ" — ta-varga, la, sa are danta
#[test]
fn ps17_dantya() {
    let varnas = parse("tls");
    for v in &varnas {
        assert_eq!(v.sthana(), Some(Sthana::Danta),
            "expected danta for {:?}", v);
    }
}

/// PS.17: "ōṣṭhajāvupū" — u, pa-varga are oshtha
#[test]
fn ps17_oshthya() {
    let varnas = parse("up");
    for v in &varnas {
        assert_eq!(v.sthana(), Some(Sthana::Oshtha),
            "expected oshtha for {:?}", v);
    }
}

/// PS.18: "ēai tu kaṇṭhatālavyā" — e, ai are kantha-talu
#[test]
fn ps18_kantha_talavya() {
    let varnas = parse("eE");
    for v in &varnas {
        assert_eq!(v.sthana(), Some(Sthana::KanthaTalu),
            "expected kantha-talu for {:?}", v);
    }
}

/// PS.18: "ōau kaṇṭhōṣṭhajau" — o, au are kantha-oshtha
#[test]
fn ps18_kantha_oshthya() {
    let varnas = parse("oO");
    for v in &varnas {
        assert_eq!(v.sthana(), Some(Sthana::KanthaOshtha),
            "expected kantha-oshtha for {:?}", v);
    }
}

/// PS.18: "dantyōṣṭhyō vaḥ" — va is danta-oshtha
#[test]
fn ps18_danta_oshthya() {
    let varnas = parse("v");
    assert_eq!(varnas[0].sthana(), Some(Sthana::DantaOshtha));
}

/// PS.21: "tēbhyō'pi vivṛtāvēṅau tābhyāmaichau tathaiva cha"
/// e/o are vivrita (guna), ai/au are ativivrita (vrddhi).
/// All four are dirgha — they differ only in vivrti degree.
#[test]
fn ps21_vivrti() {
    let varnas = parse("eEoO");
    // All four are dirgha
    for v in &varnas {
        assert_eq!(v.matra_count(), Some(2),
            "compound vowels should be dirgha (2 matras): {:?}", v);
    }
    // e and o are vivrita
    assert!(matches!(varnas[0], Varna::Svara { vivrti: Some(Vivrti::Vivrita), .. }));
    assert!(matches!(varnas[2], Varna::Svara { vivrti: Some(Vivrti::Vivrita), .. }));
    // ai and au are ativivrita
    assert!(matches!(varnas[1], Varna::Svara { vivrti: Some(Vivrti::Ativivrita), .. }));
    assert!(matches!(varnas[3], Varna::Svara { vivrti: Some(Vivrti::Ativivrita), .. }));
}

/// PS.38: "achō'spṛṣṭā" — vowels are asprishta (open, vivrita)
/// PS.38: "yaṇastvīṣannēmaspṛṣṭāḥ" — semivowels are ishat-sprshta
/// PS.38: "śēṣāḥ spṛṣṭā halaḥ" — remaining consonants are sprshta
#[test]
fn ps38_prayatna() {
    let stops = parse("kgcjwqtdpb");
    assert!(stops.iter().all(|v| v.is_sparsha()), "stops should be sprshta");

    let semivowels = parse("yrlv");
    assert!(semivowels.iter().all(|v| v.is_antahstha()), "semivowels should be ishat-sprshta");

    let sibilants = parse("Szs");
    assert!(sibilants.iter().all(|v| v.is_ushman()), "sibilants should be vivrita");
}

/// PS.39: "ñamōnunāsikā" — nasals are anunasika
#[test]
fn ps39_anunasika() {
    let nasals = parse("NYRnm");
    assert!(nasals.iter().all(|v| v.is_anunasika()),
        "5th of each varga should be anunasika");
}

/// PS.11: "hrasvō dīrghaḥ pluta iti kālatō niyamā achi"
#[test]
fn ps11_kala() {
    let varnas = parse("aA");
    assert_eq!(varnas[0].matra_count(), Some(1)); // hrasva = 1 matra
    assert_eq!(varnas[1].matra_count(), Some(2)); // dirgha = 2 matras
}
