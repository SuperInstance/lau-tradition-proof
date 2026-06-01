# lau-tradition-proof

Mathematical proof that conservation is tradition-independent via Noether's theorem.

The same conservation invariant — Σinflows = Σoutflows + Σstored — verified through seven cultural lenses: Western, Chinese, Vedic, Islamic, Japanese, African, and Indigenous. Every tradition expresses the same law in different language. The mathematics doesn't care what you call it.

**34 tests · serde only**

---

## What This Does

`lau-tradition-proof` provides:

- A **conservation invariant** that tracks inflows, outflows, and stored quantities, with `verify(tolerance)`.
- **Seven cultural expressions** of the same conservation law, each with their own terminology, formula, and proverb.
- A **cross-tradition proof** that verifies the same invariant through all seven lenses simultaneously — proving they always agree.
- A **proof suite** that aggregates multiple proofs and reports agreement rates.
- A **Noether's theorem argument** explaining why conservation follows from symmetry, independent of cultural notation.

The key insight: conservation error is identical regardless of which tradition you use to describe it. The books must balance regardless of the language.

---

## Key Idea

**Conservation is tradition-independent because it follows from Noether's theorem.**

Noether's theorem (1918) proves that every continuous symmetry of a physical system's action yields a conservation law. Time-translation symmetry → energy conservation. This is true whether you call it "energy balance" (Western), "道衡 tao balance" (Chinese), "ऋत ṛta" (Vedic), "الجبر al-jabr" (Islamic), "和 wa" (Japanese), "ubuntu reciprocity" (African), or "seventh generation" (Indigenous).

The crate demonstrates this by defining a `ConservationInvariant` once and verifying it through all seven traditions. The error is always the same number. The boolean result is always the same.

---

## Install

```toml
[dependencies]
lau-tradition-proof = { git = "https://github.com/SuperInstance/lau-tradition-proof" }
```

Requires Rust 2021 edition. Dependencies: `serde`, `serde_json`.

---

## Quick Start

### Verify a Conservation Invariant

```rust
use lau_tradition_proof::*;

// Define a physical system's energy flows
let river = ConservationInvariant::new(
    10.0,                                       // total energy
    vec![("rainfall".into(), 5.0), ("tributary".into(), 5.0)],   // inflows = 10
    vec![("evaporation".into(), 3.0), ("downstream".into(), 4.0)], // outflows = 7
    vec![("lake".into(), 3.0)],                                  // stored = 3
);

// Verify: 10 - 7 - 3 = 0 ✓
assert!(river.verify(1e-9));
```

### Cross-Tradition Proof

```rust
use lau_tradition_proof::*;

// Same invariant, seven cultural lenses
let proof = CrossTraditionProof::new(
    ConservationInvariant::new(
        42.0,
        vec![("source".into(), 20.0), ("spring".into(), 22.0)],
        vec![("sink".into(), 30.0)],
        vec![("reservoir".into(), 12.0)],
    )
);

// All seven traditions agree
assert!(proof.all_agree(1e-9));

// Error is identical across traditions
let errors = proof.error_by_tradition();
let western_error = errors[&Tradition::Western];
let chinese_error = errors[&Tradition::Chinese];
assert_eq!(western_error, chinese_error); // identical

// Noether's theorem explanation
println!("{}", proof.noether_argument());
```

### Using Pre-Built Proofs

```rust
// Three pre-built examples
let river = the_river();          // Water flowing through a system
let build = the_build();          // Blocks in a structure
let conversation = the_conversation(); // Energy in a dialogue

// Full proof suite
let suite = default_suite();
assert!(suite.all_verified(1e-9));
println!("{}", suite.suite_summary());
```

### Custom Tradition Expressions

```rust
let custom = CrossTraditionProof::with_expressions(
    ConservationInvariant::new(
        10.0,
        vec![("in".into(), 10.0)],
        vec![("out".into(), 7.0)],
        vec![("stored".into(), 3.0)],
    ),
    vec![
        TraditionExpression::western(),
        TraditionExpression::chinese(),
        TraditionExpression::vedic(),
    ],
);
let results = custom.verify_all(1e-9);
// All three traditions return the same boolean
```

---

## API Reference

### `ConservationInvariant`

| Method | Description |
|--------|-------------|
| `new(total, inflows, outflows, stored)` | Create invariant from flows |
| `verify(tolerance)` | Returns `true` if `|inflows - outflows - stored| < tolerance` |
| `error()` | Absolute conservation error |

### `Tradition` (enum)

Seven variants: `Western`, `Chinese`, `Vedic`, `Islamic`, `Japanese`, `African`, `Indigenous`.

### `TraditionExpression`

| Field | Description |
|-------|-------------|
| `tradition` | Which tradition |
| `terminology` | HashMap of term mappings (balance, inflow, outflow, stored) |
| `verification_formula` | Cultural formula for conservation |
| `example_proverb` | Traditional saying about balance |

Static constructors: `TraditionExpression::western()`, `::chinese()`, `::vedic()`, `::islamic()`, `::japanese()`, `::african()`, `::indigenous()`, `::all_traditions()`.

### `CrossTraditionProof`

| Method | Description |
|--------|-------------|
| `new(invariant)` | Create proof with all 7 traditions |
| `with_expressions(invariant, exprs)` | Create with custom tradition subset |
| `verify_all(tolerance)` | HashMap<Tradition, bool> for each lens |
| `all_agree(tolerance)` | True if all traditions return the same boolean |
| `error_by_tradition()` | HashMap<Tradition, f64> (all identical) |
| `noether_argument()` | Human-readable Noether explanation |

### `ProofSuite`

| Method | Description |
|--------|-------------|
| `new()` | Empty suite |
| `add(proof)` | Add a cross-tradition proof |
| `all_verified(tolerance)` | True if all proofs pass |
| `tradition_agreement_rate(tolerance)` | Fraction of proofs where all traditions agree |
| `suite_summary()` | Human-readable report |

### Pre-Built Proofs

| Function | Description | Flows |
|----------|-------------|-------|
| `the_river()` | Water system | rainfall+tributary → evaporation+downstream, lake |
| `the_build()` | Block structure | delivered+salvaged → waste+recycled, in_structure |
| `the_conversation()` | Dialogue energy | attention+intention → words+gestures, understanding |
| `default_suite()` | All three proofs | — |

---

## How It Works

### Conservation Check

The invariant checks: `|Σ inflows - Σ outflows - Σ stored| < tolerance`. This is the bookkeeping identity: everything that comes in must either go out or be stored.

### Cross-Tradition Verification

Each `TraditionExpression` wraps the same invariant with different terminology. The `verify_all()` method applies the same `invariant.verify()` call and maps the result to each tradition. Since the underlying invariant is identical, every tradition returns the same boolean.

### Agreement Proof

`all_agree()` proves that all traditions return the same verification result. This is trivially true because they share the same invariant — but that's the point. Conservation doesn't depend on cultural notation.

### Noether's Theorem

The `noether_argument()` provides the theoretical justification: Emmy Noether proved (1918) that continuous symmetries of the action yield conservation laws. Time-translation symmetry → energy conservation. This is a theorem about the structure of the mathematics, not about the language used to describe it.

---

## The Math

### Conservation Law

For a system with inflows, outflows, and storage:

```
Σ inflows = Σ outflows + Δ stored

Conservation error ε = |Σ inflows - Σ outflows - Σ stored|
System is conserved iff ε < tolerance
```

### Tradition Independence

Let T be the set of traditions. For each tradition t ∈ T:

```
verify_t(invariant) = (ε < tolerance)

Since ε is computed from the same invariant regardless of t:
  ∀ t₁, t₂ ∈ T: verify_{t₁}(invariant) = verify_{t₂}(invariant)
```

This is the **tradition-independence theorem**: the conservation status of a system is independent of the cultural framework used to describe it.

### Noether's Theorem (Statement)

If S[φ] = ∫ L(φ, ∂φ/∂t) dt is the action of a physical system, and if S is invariant under a continuous symmetry transformation, then there exists a conserved current J such that:

```
dJ/dt = 0
```

For time-translation symmetry (t → t + ε), the conserved quantity is energy. The proof is constructive and does not depend on language, culture, or notation.

---

## License

MIT
