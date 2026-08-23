// g₅(13) — Exact Rational Verification of Jacobi Identity
// Authors: Ahmad Ali Parr, Jessica L. Williams (SNAPKITTYWEST)
//
// Verifies that the Lie algebra g₅(13) over ℤ[1/12] satisfies the
// Jacobi identity exactly over 52,847 basis triples.
//
// Zero tolerance. Any violation is a failure.

mod exact_rational;

fn main() {
    println!("=== g₅(13) Exact Rational Verification ===");
    println!("Base ring: ℤ[1/12]\n");

    match exact_rational::run_exact_verification() {
        Ok(()) => {
            println!("\n✓ VERIFICATION PASSED");
            println!("  g₅(13) is a valid Lie algebra over ℤ[1/12]");
            println!("  Jacobi identity holds exactly.");
        }
        Err(e) => {
            eprintln!("\n✗ VERIFICATION FAILED: {}", e);
            std::process::exit(1);
        }
    }
}
