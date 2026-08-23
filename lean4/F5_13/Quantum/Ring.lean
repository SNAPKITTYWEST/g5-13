-- F5_13/Quantum/Ring.lean
-- Quantum parameter ring ℤ[1/12][q^{1/12}, q^{-1/12}]
-- Authors: Ahmad Ali Parr, Jessica L. Williams (SNAPKITTYWEST)
--
-- The fractional q-powers arise because C_{3,5} = -1/12 forces
-- K₃ E₅ K₃⁻¹ = q^{-1/12} E₅ — requiring q^{1/12} in the base ring.
-- This is NOT optional. It is forced by the mathematics of g₅(13).

namespace F5_13.Quantum

-- ── Quantum ring ℤ[1/12][q^{1/12}, q^{-1/12}] ────────────────────────────────
-- Implemented as Laurent polynomials with exponents in (1/12)ℤ
-- and coefficients in ℤ[1/12].
-- Exponent group: ℤ where the unit represents 1/12.
-- So q^{n/12} is represented by the exponent n : ℤ.

-- The base ring ℤ[1/12]
def ZInv12 := Localization ℤ (Submonoid.powers (12 : ℤ))

-- The quantum ring: Laurent polynomials in q^{1/12} over ℤ[1/12]
def QuantumRing := AddMonoidAlgebra ZInv12 ℤ

namespace QuantumRing

-- q^{1/12} (the generator)
def qRoot : QuantumRing := AddMonoidAlgebra.single 1 1

-- q = q^{12/12} = (q^{1/12})^{12}
def q : QuantumRing := AddMonoidAlgebra.single 12 1

-- q^{-1}
def qInv : QuantumRing := AddMonoidAlgebra.single (-12) 1

-- q^{n/12} for integer n
def qPow (n : ℤ) : QuantumRing := AddMonoidAlgebra.single n 1

-- q^{C} for C ∈ ℤ[1/12] with denominator dividing 12
-- e.g. q^{-1/12} = qPow (-1)
def qPowFrac (num : ℤ) : QuantumRing := qPow num  -- exponent = num (in 1/12 units)

-- Quantum integer [n]_q = (q^n - q^{-n}) / (q - q^{-1})
-- Lives in ℤ[q, q^{-1}] ⊂ QuantumRing
noncomputable def quantumInt (n : ℤ) : QuantumRing :=
  if n = 0 then 0 else
    (qPow (12 * n) - qPow (-12 * n)) * (qPow 12 - qPow (-12))⁻¹

-- The key fractional quantum integers for g₅(13):
-- [−1/12]_q = (q^{-1/12} - q^{1/12}) / (q - q^{-1})
noncomputable def quantumIntInv12 : QuantumRing :=
  (qPow (-1) - qPow 1) * (qPow 12 - qPow (-12))⁻¹

end QuantumRing

end F5_13.Quantum
