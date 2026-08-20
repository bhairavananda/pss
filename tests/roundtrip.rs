/// Full roundtrip test: SLP1 → Varna → PSS bytes → Varna → SLP1.
///
/// This is the critical correctness test. If any step loses information,
/// the roundtrip fails.

use pss::parser::slp1 as slp1_parser;
use pss::emitter::slp1 as slp1_emitter;
use pss::encode;
use pss::decode;

#[test]
fn test_full_roundtrip() {
    let cases = vec![
        "agni",
        "agnimIqepurohitam",
        "yajYasyadevaM",
        "kfzRa",
        "saMskftam",
        "BagavadgItA",
        "Darmasya",
        "SivaH",
        "vizRuH",
        "brahmA",
    ];

    for input in cases {
        let varnas = slp1_parser::parse(input);
        let bytes = encode::encode(&varnas);
        let decoded = decode::decode(&bytes);
        let output = slp1_emitter::emit(&decoded);

        assert_eq!(
            output, input,
            "roundtrip failed for '{}': got '{}'",
            input, output
        );
    }
}

#[test]
fn test_all_sparsha() {
    // All 25 sparsha consonants followed by 'a'
    let input = "kakKagaGaNacacCajaJaYawawWaqaQaRatatTadaDanaPapaPabaBAma";
    let varnas = slp1_parser::parse(input);
    let bytes = encode::encode(&varnas);
    let decoded = decode::decode(&bytes);
    let output = slp1_emitter::emit(&decoded);
    assert_eq!(output, input);
}

#[test]
fn test_all_svaras() {
    let input = "aAiIuUfFeEoO";
    let varnas = slp1_parser::parse(input);
    let bytes = encode::encode(&varnas);
    let decoded = decode::decode(&bytes);
    let output = slp1_emitter::emit(&decoded);
    assert_eq!(output, input);
}

#[test]
fn test_antahstha_ushman() {
    let input = "yaralavaSzsa";
    let varnas = slp1_parser::parse(input);
    let bytes = encode::encode(&varnas);
    let decoded = decode::decode(&bytes);
    let output = slp1_emitter::emit(&decoded);
    assert_eq!(output, input);
}

#[test]
fn test_ayogavaha() {
    // anusvara and visarga
    let input = "rAmaHkfzRaM";
    let varnas = slp1_parser::parse(input);
    let bytes = encode::encode(&varnas);
    let decoded = decode::decode(&bytes);
    let output = slp1_emitter::emit(&decoded);
    assert_eq!(output, input);
}
