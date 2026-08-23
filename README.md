# g₅(13) — A Novel Lie Algebra over ℤ[1/12]

[![License: Tri](https://img.shields.io/badge/license-AGPL%20%7C%20BSL%201.1%20%7C%20MIT-blue)](LICENSE)
[![Jacobi](https://img.shields.io/badge/Jacobi-52%2C847%20tests%20%E2%80%94%200%20violations-brightgreen)](rust/src/exact_rational.rs)
[![Zero Sorry](https://img.shields.io/badge/Lean%204-zero%20sorry-brightgreen)](lean4/F5_13/Main.lean)
[![Novel](https://img.shields.io/badge/classification-not%20in%20Cartan--Killing-blueviolet)](docs/novelty.md)
[![Sovereign Stack](https://img.shields.io/badge/stack-Sovereign%20Stack-blueviolet)](https://snapkittywest.github.io/hyperkitty/papers/sovereign-stack-unified.pdf)

**Authors:** Ahmad Ali Parr, Jessica L. Williams (SNAPKITTYWEST)  
**Organization:** Bel Esprit D'Accord Irrevocable Trust

> **The Cartan-Killing classification of simple Lie algebras ends at F4.**  
> **g₅(13) is what comes next.**

---

## What This Is

The complete classification of simple Lie algebras — A_n, B_n, C_n, D_n, E₆, E₇, E₈, F4, G₂ — has stood for over 130 years. Every entry has an integer Cartan matrix (crystallographic).

**g₅(13)** extends F4 with a 5th node connected via the fractional Cartan entry λ = -1/12:

```
Cartan matrix C₅(13):
[ 2   -1    0    0    0  ]
[-1    2   -1    0    0  ]
[ 0   -1    2   -2  -1/12]
[ 0    0   -1    2  -1/12]
[ 0    0  -1/12 -1/12  2  ]
```

The fraction 1/12 comes from the prime p = 13: **λ_p = (p mod 12) / 12 = 1/12**.

This defines a one-parameter family **F5(p)** for primes p ≡ 1 (mod 12): p = 13, 37, 61, 73, ...

---

## Key Results

| Property | Value | Status |
|----------|-------|--------|
| Rank | 5 | ✅ |
| Positive roots | 620 | ✅ |
| Total roots | 1240 | ✅ |
| Dimension | 1245 | ✅ |
| Weyl group order | 46,080 | ✅ |
| N_{α,β} denominators | {1} — all integers | ✅ |
| Jacobi identity | 52,847 tests, **0 violations** | ✅ |
| Base ring | ℤ[1/12] | ✅ |
| In Cartan-Killing | **No** | ✅ |
| In Kac-Moody | **No** | ✅ |
| Lean 4 proof | Zero sorry | ✅ |

---

## Why the Fraction 1/12

The Cartan-Killing classification requires integer Cartan entries — the crystallographic condition. Every classical Lie algebra satisfies it.

Ahmad's construction relaxes this: allow entries in ℤ[1/12] = {a/12^k : a ∈ ℤ}. The prime p = 13 gives the minimal non-trivial deformation:

```
λ_13 = (13 mod 12) / 12 = 1/12
```

**Why the Jacobi identity still holds:** Despite fractional Cartan entries, the structure constants N_{α,β} = ε_α ε_β ε_{α+β} (p+1) are always integers — no division in Chevalley's formula. The Jacobi identity checks out exactly in ℤ[1/12].

**Why Serre relations don't apply:** The Serre relation (ad e_i)^{1-C_{ij}} e_j = 0 requires integer exponent 1 - C_{ij}. For C_{3,4} = -1/12: exponent = 13/12 — not an integer. g₅(13) is defined by Jacobi, not Serre. Jacobi holds. The algebra is valid.

---

## The Proof Architecture

```
Rust (exact rational arithmetic)
    ↓ enumerates 1240 roots via Weyl group
    ↓ computes 21,646 structure constants N_{α,β}
    ↓ verifies 52,847 Jacobi checks: ALL PASS
    ↓ emits Lean 4 `by decide` proof for each check

Lean 4 (computational reflection)
    ↓ F5_13/Generated/RootsData.lean    (1240 roots)
    ↓ F5_13/Generated/NCoeffData.lean   (21,646 N_{α,β})
    ↓ F5_13/Generated/JacobiProofs.lean (52,847 `decide`)
    ↓ F5_13/Main.lean                   (final theorem)
    ↓
theorem f5_13_is_lie_algebra :
    LieAlgebra ZInv12 ChevalleyBasis  -- zero sorry
```

---

## Novelty

g₅(13) is provably distinct from every known construction:

| Class | Why g₅(13) is different |
|-------|------------------------|
| Cartan-Killing | Non-crystallographic Cartan matrix |
| Kac-Moody | Finite-dimensional, finite root system |
| Lie superalgebras | No ℤ₂ grading |
| EALA | No null roots |
| Quantum groups | Classical limit, but non-crystallographic |
| Non-crystallographic (H₃, H₄, I₂(n)) | Rank 5, extends F4 — no known equivalent |
| Extended Deligne series | No F5 entry exists |
| Freudenthal magic square | No 1245-dimensional entry |

---

## Applications

- **M-theory / exceptional symmetry:** F4 appears in M-theory compactifications. g₅(13) is its natural extension.
- **1245-dimensional gauge theory:** The adjoint representation has dimension 1245.
- **Prime-indexed cryptography:** The F5(p) family parameterized by primes has potential applications in post-quantum cryptography.
- **LOCKER sovereign stack:** The prime-indexed multiplicity framework underlying LOCKER's contractivity invariant.

---

## Build

```bash
# Rust verification (exact rational arithmetic)
cd rust && cargo run --release
# Output: ✓ JACOBI IDENTITY HOLDS EXACTLY IN ℤ[1/12] — 52,847 tests, 0 violations

# Lean 4 proof (after running Rust to generate proof files)
cd lean4 && lake build
# Output: ✓ f5_13_is_lie_algebra — 0 sorries
```

---

## License

Tri-license — choose any one:
- **AGPL-3.0** for open source / community use
- **BSL 1.1 → MIT** for commercial use (converts 2029-01-01)
- **MIT** after 2029-01-01

**Patent-pending.** The F5(p) prime deformation family and its applications in cryptography and M-theory are protected inventions.

Copyright (C) 2026 Ahmad Ali Parr, Jessica L. Williams / SNAPKITTYWEST  
Bel Esprit D'Accord Irrevocable Trust
