// g₅(13) — Exact Rational Verification of Jacobi Identity
// Authors: Ahmad Ali Parr, Jessica L. Williams (SNAPKITTYWEST)
//
// Verifies that the Lie algebra g₅(13) over ℤ[1/12] satisfies the
// Jacobi identity exactly over 52,847 basis triples.
//
// Zero tolerance. Any violation is a failure.

mod exact_rational;
mod quantum_exact;

fn main() {
    println!("=== g₅(13) Exact Rational Verification ===");
    println!("Base ring: ℤ[1/12]\n");

    match exact_rational::run_exact_verification() {
        Ok(()) => {
            println!("\n✓ CLASSICAL VERIFICATION PASSED");
            println!("  g₅(13) is a valid Lie algebra over ℤ[1/12]");
            println!("  Jacobi: 52,847 tests, 0 violations.");
        }
        Err(e) => {
            eprintln!("\n✗ CLASSICAL VERIFICATION FAILED: {}", e);
            std::process::exit(1);
        }
    }

    // Quantum verification
    println!();
    match quantum_exact::run_quantum_verification() {
        Ok(()) => println!("\n✓ QUANTUM VERIFICATION PASSED"),
        Err(e) => { eprintln!("\n✗ QUANTUM VERIFICATION FAILED: {}", e); std::process::exit(1); }
    }
}
