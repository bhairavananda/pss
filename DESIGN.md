# PSS Design Document

## Source Text

All design decisions derive from the Paniniya Shiksha (PS). Sutra numbers
referenced as PS.N throughout.

## 1. Varna Inventory

### PS.3: Total Count
> triṣaṣṭiśchatuḥṣaṣṭirvā varṇāḥ śambhumatē matāḥ

63 or 64 varnas according to Shambhu's doctrine. The difference accounts for
whether the duhsprshta ḷ (PS.5) is counted separately.

### PS.4: Breakdown
> svarāviṃśatirēkaścha sparśānāṃ pañchaviṃśatiḥ
> yādayaścha smṛtā hyaṣṭau chatvāraścha yamāḥ smṛtāḥ

| Category | Count | Members |
|---|---|---|
| Svara (vowels) | 21 | a ā i ī u ū ṛ ṝ ḷ ḹ e ai o au (× hrasva/dirgha/pluta where applicable) |
| Sparsha (stops) | 25 | ka-varga, ca-varga, ṭa-varga, ta-varga, pa-varga (5 × 5) |
| Ya-adi (semivowels + sibilants) | 8 | ya va ra la śa ṣa sa ha |
| Yama (nasalized stops) | 4 | nasal variants of 1st/2nd of each varga |

### PS.5: Auxiliaries
> anusvārō visargaścha ka pau chāpi parāśritau
> duspṛṣṭaśchēti vijñēyō ḷīkāraḥ pluta ēva cha

- Anusvara (ṃ)
- Visarga (ḥ)
- Jihvamuliya (ka-series visarga variant)
- Upadhmaniya (pa-series visarga variant)
- Duhsprshta ḷ
- Pluta (3-matra vowel extension)

These are ayogavaha — "dependent on a host" (PS.22: āśrayasthānabhāginaḥ).

## 2. Five-Fold Classification

### PS.10
> svarataḥ kālataḥ sthānāt prayatnānupradānataḥ
> iti varṇavidaḥ prāhurnipuṇaṃ tannibōdhata

Every varna is classified along five axes. This is the foundation of PSS.

### 2a. Svara (Pitch Accent) — PS.11
> udāttaśchānudāttaścha svaritaścha svarāstrayaḥ

Three accents:
- **Udatta** — raised pitch (mūrdhni, PS.48)
- **Anudatta** — lowered pitch (hṛdi, PS.48)
- **Svarita** — combined/falling (karṇamūlīya, PS.48)

### 2b. Kala (Duration) — PS.11
> hrasvō dīrghaḥ pluta iti kālatō niyamā achi

Three durations for vowels:
- **Hrasva** — 1 matra
- **Dirgha** — 2 matras
- **Pluta** — 3 matras

### 2c. Sthana (Place of Articulation) — PS.13, PS.17-18

> aṣṭausthānāni varṇānāmuraḥ kaṇṭhaḥ śirastathā
> jihvāmūlaṃ cha dantāścha nāsikōṣṭhaucha tālu cha

Eight places of articulation:

| Sthana | PS.17-18 Ref | Varnas |
|---|---|---|
| Kantha (guttural) | kaṇṭhyāvahāv | a ā, ka kha ga gha ṅa, ha, visarga |
| Talu (palatal) | ichuyaśāstālavyā | i ī, ca cha ja jha ña, ya, śa |
| Murdha (retroflex) | syurmūrdhanyā ṛṭuraṣā | ṛ ṝ, ṭa ṭha ḍa ḍha ṇa, ra, ṣa |
| Danta (dental) | dantyā ḷitulasāḥ | ḷ ḹ, ta tha da dha na, la, sa |
| Oshtha (labial) | ōṣṭhajāvupū | u ū, pa pha ba bha ma |
| Jihvamula (tongue root) | jihvāmūlē tu kuḥ | ka kha ga gha ṅa (secondary) |
| Nasika (nasal) | nāsikā sthānam (PS.22) | anusvara, yama nasals |
| Shiras (head) | — | jihvamuliya, upadhmaniya |

Compound sthanas (PS.18):
| Sthana | Varnas |
|---|---|
| Kantha-talu | e, ai |
| Kantha-oshtha | o, au |
| Danta-oshtha | va |

### 2d. Prayatna (Articulatory Effort) — PS.38

> achō'spṛṣṭā yaṇastvīṣannēmaspṛṣṭāḥ śalaḥ smṛtāḥ
> śēṣāḥ spṛṣṭā halaḥ prōktā

Three degrees of contact:
- **Sprshta** (full contact) — sparsha consonants (stops): ka to ma
- **Ishat-sprshta** (slight contact) — antahstha (semivowels): ya va ra la
- **Vivrita** (open) — svara (vowels) and ushman (sibilants): śa ṣa sa ha

Note: PS.21 further distinguishes vivrita degrees:
> svarāṇāmūṣmaṇāṃ chaiva vivṛtaṃ karaṇaṃ smṛtam
> tēbhyō'pi vivṛtāvēṅau tābhyāmaichau tathaiva cha

Vowels and sibilants are vivrita. Among vowels, e/o are more vivrita, and
ai/au even more so. This gradient is preserved in the encoding.

### 2e. Anupradana (Voicing/Aspiration) — PS.39

> ñamōnunāsikā na hrau nādinō hajhaṣaḥ smṛtāḥ
> īṣannādā yaṇō jaśaḥ śvāsinastu khaphādayaḥ

| Anupradana | Varnas |
|---|---|
| Anunasika (nasal) | ña ma ṅa ṇa na |
| Nadin (voiced) | ha, 3rd/4th of each varga (ga gha, ja jha, etc.) |
| Ishannadin (slightly voiced) | ya va ra la, 3rd of each varga (ja ḍa da ga ba) |
| Shvasin (aspirated/breathed) | kha pha cha ṭha tha (2nd of each varga) |
| Ishacchvasin (slightly aspirated) | not explicitly listed; scholarly debate |

For PSS, we reduce anupradana to two binary axes that capture the distinctions:
- **Ghosha** (voicing): aghosha (1st, 2nd of varga, sibilants) vs saghosha (3rd, 4th, 5th, semivowels, vowels)
- **Prana** (aspiration): alpaprana (1st, 3rd, 5th of varga) vs mahaprana (2nd, 4th of varga)

This captures the same information as PS.39's four-way system more efficiently:
- Shvasin = aghosha + mahaprana (kha, pha, etc.)
- Nadin = saghosha + mahaprana (gha, jha, etc.)
- Ishannadin = saghosha + alpaprana (ga, ja, etc.)
- Anunasika = saghosha + alpaprana + nasika sthana (ña, ma, etc.)

## 3. Byte Encoding

### Vyanjana (Consonant) — 1 byte

```
Bit layout: [0][sthana:3][prayatna:2][ghosha:1][prana:1]
             │     │          │          │         │
             │     │          │          │         └─ 0=alpaprana, 1=mahaprana
             │     │          │          └─ 0=aghosha, 1=saghosha
             │     │          └─ 00=sprshta, 01=ishat-sprshta, 10=vivrita
             │     └─ sthana enum (0-8)
             └─ 0 = vyanjana type
```

### Svara (Vowel) — 1 byte

```
Bit layout: [1][sthana:3][kala:2][svara:2]
             │     │        │       │
             │     │        │       └─ 00=anudatta, 01=udatta, 10=svarita, 11=unspecified
             │     │        └─ 00=hrasva, 01=dirgha, 10=pluta
             │     └─ sthana enum (0-8)
             └─ 1 = svara type
```

### Special Bytes

```
0xF0 = anusvara
0xF1 = visarga
0xF2 = jihvamuliya
0xF3 = upadhmaniya
0xF4 = chandrabindu
0xF5 = avagraha
0xFE = pada boundary (word break)
0xFF = vakya boundary (sentence/verse break)
```

### Sthana Enum Values

```
0 = KANTHA
1 = TALU
2 = MURDHA
3 = DANTA
4 = OSHTHA
5 = KANTHA_TALU       (for e, ai)
6 = KANTHA_OSHTHA     (for o, au)
7 = DANTA_OSHTHA      (for va)
```

### Examples

```
ka  = 0b0_000_00_0_0 = 0x00  (kantha, sprshta, aghosha, alpaprana)
kha = 0b0_000_00_0_1 = 0x01  (kantha, sprshta, aghosha, mahaprana)
ga  = 0b0_000_00_1_0 = 0x02  (kantha, sprshta, saghosha, alpaprana)
gha = 0b0_000_00_1_1 = 0x03  (kantha, sprshta, saghosha, mahaprana)
ṅa  = 0b0_000_00_1_0 = 0x02  (same as ga — distinguished by nasika, see note)

ca  = 0b0_001_00_0_0 = 0x08  (talu, sprshta, aghosha, alpaprana)
ṭa  = 0b0_010_00_0_0 = 0x10  (murdha, sprshta, aghosha, alpaprana)
ta  = 0b0_011_00_0_0 = 0x18  (danta, sprshta, aghosha, alpaprana)
pa  = 0b0_100_00_0_0 = 0x20  (oshtha, sprshta, aghosha, alpaprana)

ya  = 0b0_001_01_1_0 = 0x0E  (talu, ishat-sprshta, saghosha, alpaprana)
ra  = 0b0_010_01_1_0 = 0x16  (murdha, ishat-sprshta, saghosha, alpaprana)
la  = 0b0_011_01_1_0 = 0x1E  (danta, ishat-sprshta, saghosha, alpaprana)
va  = 0b0_111_01_1_0 = 0x3E  (danta-oshtha, ishat-sprshta, saghosha, alpaprana)

śa  = 0b0_001_10_0_0 = 0x0C  (talu, vivrita, aghosha, alpaprana)
ṣa  = 0b0_010_10_0_0 = 0x14  (murdha, vivrita, aghosha, alpaprana)
sa  = 0b0_011_10_0_0 = 0x1C  (danta, vivrita, aghosha, alpaprana)
ha  = 0b0_000_10_1_0 = 0x06  (kantha, vivrita, saghosha, alpaprana)

a   = 0b1_000_00_11 = 0x83  (kantha, hrasva, svara unspecified)
ā   = 0b1_000_01_11 = 0x87  (kantha, dirgha, svara unspecified)
i   = 0b1_001_00_11 = 0x8B  (talu, hrasva, svara unspecified)
u   = 0b1_100_00_11 = 0xA3  (oshtha, hrasva, svara unspecified)
e   = 0b1_101_01_11 = 0xAF  (kantha-talu, dirgha, svara unspecified)
o   = 0b1_110_01_11 = 0xB7  (kantha-oshtha, dirgha, svara unspecified)
```

**Note on nasals (5th of varga):** ṅa, ña, ṇa, na, ma share ghosha/prana
with the 3rd of their varga (ga, ja, ḍa, da, ba). They are distinguished by
the nasika sthana (PS.22). In the byte encoding, the 5th position is marked
by setting prana=1 as a convention (nasals are alpaprana phonologically, but
this bit serves as the nasal flag for sprshta consonants):

```
ṅa  = 0b0_000_00_1_1 = 0x03  (kantha, sprshta, saghosha, nasal-flag)
ña  = 0b0_001_00_1_1 = 0x0B  (talu, sprshta, saghosha, nasal-flag)
ṇa  = 0b0_010_00_1_1 = 0x13  (murdha, sprshta, saghosha, nasal-flag)
na  = 0b0_011_00_1_1 = 0x1B  (danta, sprshta, saghosha, nasal-flag)
ma  = 0b0_100_00_1_1 = 0x23  (oshtha, sprshta, saghosha, nasal-flag)
```

Wait — this collides with gha (0x03). The nasal needs its own encoding.
The 5th of each varga (nasals) use a dedicated prayatna value:

```
Revised prayatna encoding:
  00 = sprshta (stops, positions 1-4)
  01 = ishat-sprshta (semivowels)
  10 = vivrita (sibilants + ha)
  11 = anunasika (nasal stops, position 5)
```

This gives unique bytes for all consonants:

```
ṅa  = 0b0_000_11_1_0 = 0x06 — but this collides with ha!
```

Problem: 3 bits of sthana × 2 bits of prayatna × 1 bit ghosha × 1 bit prana
= 128 values, but collisions arise because not all feature combinations are
unique across the phoneme inventory.

### Revised Encoding: Lookup Table

Rather than bit-packing (which causes collisions), assign each of the ~50
consonant phonemes and ~14 vowel phonemes a unique byte based on a canonical
ordering that preserves the Shiksha classification:

**Vyanjana block: 0x00–0x3F (consonants)**

```
Varga stops (PS.17, 25 consonants):
  ka-varga:  0x00 ka  0x01 kha  0x02 ga  0x03 gha  0x04 ṅa
  ca-varga:  0x05 ca  0x06 cha  0x07 ja  0x08 jha  0x09 ña
  ṭa-varga:  0x0A ṭa  0x0B ṭha  0x0C ḍa  0x0D ḍha  0x0E ṇa
  ta-varga:  0x0F ta  0x10 tha  0x11 da  0x12 dha  0x13 na
  pa-varga:  0x14 pa  0x15 pha  0x16 ba  0x17 bha  0x18 ma

Antahstha (semivowels, PS.38 ishat-sprshta):
  0x19 ya  0x1A ra  0x1B la  0x1C va

Ushman (sibilants + ha, PS.38 vivrita):
  0x1D śa  0x1E ṣa  0x1F sa  0x20 ha

Yama (PS.4, nasal variants):
  0x21–0x24 (4 yama consonants)

Additional:
  0x25 ḷa (duhsprshta, PS.5)
```

**Svara block: 0x40–0x5F (vowels)**

```
Simple vowels:
  0x40 a   0x41 ā
  0x42 i   0x43 ī
  0x44 u   0x45 ū
  0x46 ṛ   0x47 ṝ
  0x48 ḷ   0x49 ḹ

Compound vowels (kantha-talu, kantha-oshtha):
  0x4A e   0x4B ai
  0x4C o   0x4D au
```

**Svara accent overlay: 0x60–0x63**

Applied after a svara byte to mark accent:
```
  0x60 udatta
  0x61 anudatta
  0x62 svarita
```

**Ayogavaha block: 0x70–0x77 (PS.5, PS.22)**

```
  0x70 anusvara (ṃ)
  0x71 visarga (ḥ)
  0x72 jihvamuliya
  0x73 upadhmaniya
  0x74 chandrabindu
```

**Structural markers: 0xF0–0xFF**

```
  0xFD avagraha
  0xFE pada boundary
  0xFF vakya boundary
```

### Feature Recovery from Byte Value

Although bytes are sequential IDs rather than bit-packed, the Shiksha features
are recoverable via constant-time lookup tables:

```rust
const STHANA: [Sthana; 38] = [
    // ka-varga (0x00–0x04): all kantha
    Kantha, Kantha, Kantha, Kantha, Kantha,
    // ca-varga (0x05–0x09): all talu
    Talu, Talu, Talu, Talu, Talu,
    // ... etc
];
```

This preserves the design intent: every phoneme's articulatory features are
accessible in O(1), but without collision problems from bit-packing.

## 4. Varna Struct (Rust)

```rust
/// A single Sanskrit phoneme, classified per Paniniya Shiksha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Varna {
    /// Svara (vowel) — PS.4, PS.11
    Svara {
        sthana: Sthana,
        kala: Kala,
        svara: Option<SvaraPitch>,  // None for non-Vedic text
    },

    /// Vyanjana (consonant) — PS.4, PS.38-39
    Vyanjana {
        sthana: Sthana,
        prayatna: Prayatna,
        ghosha: Ghosha,
        prana: Prana,
    },

    /// Ayogavaha (PS.5, PS.22) — dependent phonemes
    Ayogavaha(AyogavahaType),

    /// Structural boundary
    Boundary(BoundaryType),
}
```

## 5. Sandhi Operations

Sandhi rules reference Shiksha features directly.

### Savarna Dirgha (a + a → ā)
Two vowels with the same sthana merge, kala becomes dirgha.
```
Svara(s, hrasva) + Svara(s, hrasva) → Svara(s, dirgha)  // when sthana matches
```

### Guna (a + i → e)
Kantha svara before talu svara → kantha-talu (guna).
```
Svara(kantha, _) + Svara(talu, _) → Svara(kantha_talu, dirgha)
```

### Visarga Sandhi
Visarga before a sprshta vyanjana: determined by sthana of the following consonant.
```
Ayogavaha(visarga) + Vyanjana(kantha, sprshta, ...) → Ayogavaha(jihvamuliya)
Ayogavaha(visarga) + Vyanjana(oshtha, sprshta, ...) → Ayogavaha(upadhmaniya)
```

### Jashtva (Voice Assimilation)
1st/2nd of varga before 3rd/4th/5th: becomes 3rd (voiced, unaspirated).
```
Vyanjana(s, sprshta, aghosha, _) before Vyanjana(_, _, saghosha, _)
  → Vyanjana(s, sprshta, saghosha, alpaprana)
```

## 6. Protobuf Schema

The protobuf is the interchange/storage format. The byte encoding is derived
from it for ML and compact serialization.

See `proto/varna.proto` for the full schema.

## 7. Implementation Order

1. `varna.rs` — Varna enum, Sthana/Prayatna/Ghosha/Prana/Kala enums
2. `encode.rs` — Varna → byte, byte → Varna lookup tables
3. `parser/slp1.rs` — SLP1 text → Vec<Varna> (simplest parser, 1:1 mapping)
4. `emitter/slp1.rs` — Vec<Varna> → SLP1 text
5. `tests/roundtrip.rs` — SLP1 → Varna → byte → Varna → SLP1 identity
6. `tests/shiksha_compliance.rs` — verify feature assignments match PS.17-18, PS.38-39
7. `parser/iast.rs` + `emitter/iast.rs`
8. `parser/devanagari.rs` + `emitter/devanagari.rs`
9. `sandhi.rs` — sandhi operations on Varna sequences
10. `akshara.rs` — syllabification
11. `chandas.rs` — meter detection from kala sequences
