# PSS: Paniniya Shiksha Serialization

A serialization format for Sanskrit that encodes text as phonological feature
bundles derived from the Paniniya Shiksha, rather than as characters or
codepoints. Any surface representation (Devanagari, IAST, ITRANS, SLP1)
compiles down to the same canonical bytes.

```
"śiva"  ──┐
"शिव"  ──┼──▶ [SHA(talu,vivrita) I(hrasva) VA(dantoshtha) A(hrasva)]
"Siva"  ──┘     (identical PSS bytes regardless of input)
```

## Phonological Basis

The five-fold classification of varnas from Paniniya Shiksha 10:

> svarataḥ kālataḥ sthānāt prayatnānupradānataḥ

1. **Svara** — pitch accent (udatta, anudatta, svarita)
2. **Kala** — duration (hrasva, dirgha, pluta)
3. **Sthana** — place of articulation (8 positions)
4. **Prayatna** — articulatory effort (sprshta, ishat-sprshta, vivrita)
5. **Anupradana** — voicing/aspiration (ghoshavat, aghosha, shvasa, nada)

These five axes fully determine every Sanskrit phoneme. PSS encodes them
directly into a compact binary format.

## Usage

```rust
use pss::{Varna, parse_iast, emit_devanagari};

let varnas: Vec<Varna> = parse_iast("agnimīḷe purohitam");
let bytes: Vec<u8> = pss::encode(&varnas);
let text: String = emit_devanagari(&pss::decode(&bytes));
```

## Structure

```
pss/
  proto/varna.proto          — protobuf schema (formal spec)
  src/
    lib.rs                   — public API
    varna.rs                 — Varna enum + feature accessors
    encode.rs                — Varna → PSS byte encoding
    decode.rs                — PSS bytes → Varna
    parser/                  — surface text → Varna
      slp1.rs                — SLP1 (start here: 1:1, unambiguous)
      iast.rs                — IAST
      devanagari.rs          — Devanagari Unicode
    emitter/                 — Varna → surface text
      slp1.rs
      iast.rs
      devanagari.rs
    sandhi.rs                — sandhi as feature-level operations
    akshara.rs               — syllabification
    chandas.rs               — meter detection
  tests/
    roundtrip.rs             — parse → encode → decode → emit must be identity
    shiksha_compliance.rs    — verify all sutras hold on the encoding
    sandhi.rs                — known sandhi pairs
```

## License

MIT
