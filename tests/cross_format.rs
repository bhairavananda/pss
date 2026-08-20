/// Cross-format tests: parse from one format, emit to another.
/// The PSS byte encoding must be identical regardless of input format.

use pss::parser::iast as iast_parser;
use pss::parser::slp1 as slp1_parser;
use pss::emitter::iast as iast_emitter;
use pss::emitter::slp1 as slp1_emitter;
use pss::encode;

/// IAST and SLP1 must produce identical PSS bytes for the same word.
#[test]
fn test_iast_slp1_same_bytes() {
    let pairs = vec![
        ("agni", "agni"),
        ("kṛṣṇa", "kfzRa"),
        ("dharma", "Darma"),
        ("śiva", "Siva"),
        ("viṣṇu", "vizRu"),
        ("rāma", "rAma"),
        ("bhagavadgītā", "BagavadgItA"),
        ("saṃskṛtam", "saMskftam"),
        ("khaḍga", "Kaqga"),
        ("jñāna", "jYAna"),
    ];

    for (iast, slp1) in pairs {
        let iast_varnas = iast_parser::parse(iast);
        let slp1_varnas = slp1_parser::parse(slp1);

        let iast_bytes = encode::encode(&iast_varnas);
        let slp1_bytes = encode::encode(&slp1_varnas);

        assert_eq!(
            iast_bytes, slp1_bytes,
            "byte mismatch: IAST '{}' vs SLP1 '{}'\n  IAST varnas: {:?}\n  SLP1 varnas: {:?}",
            iast, slp1, iast_varnas, slp1_varnas
        );
    }
}

/// Parse IAST → emit SLP1 (cross-format conversion via PSS)
#[test]
fn test_iast_to_slp1() {
    let cases = vec![
        ("agni", "agni"),
        ("kṛṣṇa", "kfzRa"),
        ("dharma", "Darma"),
        ("śiva", "Siva"),
        ("bhagavadgītā", "BagavadgItA"),
    ];

    for (iast_input, expected_slp1) in cases {
        let varnas = iast_parser::parse(iast_input);
        let slp1_output = slp1_emitter::emit(&varnas);
        assert_eq!(
            slp1_output, expected_slp1,
            "IAST→SLP1 failed for '{}'", iast_input
        );
    }
}

/// Parse SLP1 → emit IAST (cross-format conversion via PSS)
#[test]
fn test_slp1_to_iast() {
    let cases = vec![
        ("agni", "agni"),
        ("kfzRa", "kṛṣṇa"),
        ("Darma", "dharma"),
        ("Siva", "śiva"),
        ("BagavadgItA", "bhagavadgītā"),
    ];

    for (slp1_input, expected_iast) in cases {
        let varnas = slp1_parser::parse(slp1_input);
        let iast_output = iast_emitter::emit(&varnas);
        assert_eq!(
            iast_output, expected_iast,
            "SLP1→IAST failed for '{}'", slp1_input
        );
    }
}

/// Compound vowels must survive cross-format roundtrip
#[test]
fn test_compound_vowels_cross_format() {
    // IAST ai/au → SLP1 E/O
    let v = iast_parser::parse("aindra aurṇavābha");
    let slp1 = slp1_emitter::emit(&v);
    assert_eq!(slp1, "EndraOrRavABa");

    // SLP1 E/O → IAST ai/au
    let v2 = slp1_parser::parse("EndrA");
    let iast = iast_emitter::emit(&v2);
    assert_eq!(iast, "aindrā");
}
