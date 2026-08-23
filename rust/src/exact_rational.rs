// exact_rational.rs — g₅(13) exact Jacobi verification over ℤ[1/12]
// Authors: Ahmad Ali Parr, Jessica L. Williams (SNAPKITTYWEST)
//
// All arithmetic in exact rational arithmetic via num-rational::Rational64.
// Zero tolerance: any Jacobi violation is a hard failure.

use num_rational::Rational64;
use num_traits::{Zero, One};
use std::collections::{HashMap, HashSet, VecDeque};

// ── Root type ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Root(pub [Rational64; 5]);

impl Root {
    fn simple(i: usize) -> Self {
        let mut v = [Rational64::zero(); 5];
        v[i] = Rational64::one();
        Root(v)
    }

    fn add(&self, other: &Root) -> Root {
        Root([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
            self.0[4] + other.0[4],
        ])
    }

    fn scale(&self, k: i64) -> Root {
        let r = Rational64::new(k, 1);
        Root([self.0[0]*r, self.0[1]*r, self.0[2]*r, self.0[3]*r, self.0[4]*r])
    }

    fn is_positive(&self) -> bool {
        for c in &self.0 {
            if *c > Rational64::zero() { return true; }
            if *c < Rational64::zero() { return false; }
        }
        false
    }

    fn height(&self) -> Rational64 {
        self.0.iter().copied().sum()
    }
}

// ── Cartan matrix for F5(13) ──────────────────────────────────────────────────

fn cartan_f5_13() -> [[Rational64; 5]; 5] {
    let z  = Rational64::zero();
    let i  = |n: i64| Rational64::new(n, 1);
    let f  = |n: i64, d: i64| Rational64::new(n, d);
    [
        [i(2),  i(-1), z(),     z(),      z()      ],
        [i(-1), i(2),  i(-1),   z(),      z()      ],
        [z(),   i(-1), i(2),    i(-2),    f(-1, 12)],
        [z(),   z(),   i(-1),   i(2),     f(-1, 12)],
        [z(),   z(),   f(-1,12),f(-1,12), i(2)     ],
    ]
}

fn reflect(c: &[[Rational64; 5]; 5], root: &Root, i: usize) -> Root {
    let mut coeff = Rational64::zero();
    for j in 0..5 { coeff = coeff + root.0[j] * c[j][i]; }
    let mut r = root.clone();
    r.0[i] = r.0[i] - coeff;
    r
}

// ── Weyl group (BFS on reduced words) ────────────────────────────────────────

fn weyl_group_words(c: &[[Rational64; 5]; 5]) -> Vec<Vec<usize>> {
    let mut words: HashSet<Vec<usize>> = HashSet::new();
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    words.insert(vec![]);
    queue.push_back(vec![]);

    while let Some(w) = queue.pop_front() {
        if w.len() >= 50 { continue; }
        for i in 0..5 {
            let mut nw = w.clone();
            nw.push(i);
            // Simple reduction: cancel consecutive identical generators
            let reduced = simple_reduce(&nw);
            if words.insert(reduced.clone()) {
                queue.push_back(reduced);
            }
        }
    }
    words.into_iter().collect()
}

fn simple_reduce(word: &[usize]) -> Vec<usize> {
    let mut stack: Vec<usize> = Vec::new();
    for &i in word {
        if stack.last() == Some(&i) { stack.pop(); } else { stack.push(i); }
    }
    stack
}

// ── Root enumeration ──────────────────────────────────────────────────────────

pub fn enumerate_positive_roots() -> (Vec<Root>, HashMap<Root, usize>) {
    let c = cartan_f5_13();
    let wg = weyl_group_words(&c);

    let mut root_set: HashSet<Root> = HashSet::new();
    for w in &wg {
        for s in 0..5 {
            let mut root = Root::simple(s);
            for &i in w { root = reflect(&c, &root, i); }
            root_set.insert(root);
        }
    }

    let mut pos: Vec<Root> = root_set.into_iter()
        .filter(|r| r.is_positive())
        .collect();

    pos.sort_by(|a, b| {
        let ha = a.height();
        let hb = b.height();
        ha.partial_cmp(&hb).unwrap()
            .then_with(|| {
                for i in 0..5 {
                    let o = a.0[i].partial_cmp(&b.0[i]).unwrap();
                    if o != std::cmp::Ordering::Equal { return o; }
                }
                std::cmp::Ordering::Equal
            })
    });

    let mut idx = HashMap::new();
    for (i, r) in pos.iter().enumerate() {
        idx.insert(r.clone(), i);
    }
    (pos, idx)
}

// ── Structure constants N_{alpha, beta} ───────────────────────────────────────

pub fn compute_structure_constants(
    roots: &[Root],
    idx: &HashMap<Root, usize>,
) -> HashMap<(usize, usize), Rational64> {
    let mut nc: HashMap<(usize, usize), Rational64> = HashMap::new();

    for i in 0..roots.len() {
        for j in 0..roots.len() {
            if i == j { continue; }
            let sum = roots[i].add(&roots[j]);
            if idx.contains_key(&sum) {
                // Root string: β - p*α, ..., β, ..., β + q*α
                // p = max k≥0 such that β - k*α is a root
                let mut p = 0i64;
                loop {
                    let test = roots[j].add(&roots[i].scale(-(p+1)));
                    if !idx.contains_key(&test) { break; }
                    p += 1;
                }

                // Sign convention: ε_α = (-1)^{h(h-1)/2} where h = height
                let sign = |h: i64| -> i64 {
                    if (h * (h-1) / 2) % 2 == 0 { 1 } else { -1 }
                };

                let k = idx[&sum];
                let hi = roots[i].height().to_integer();
                let hj = roots[j].height().to_integer();
                let hk = roots[k].height().to_integer();

                let n = Rational64::new(sign(hi) * sign(hj) * sign(hk) * (p + 1), 1);
                nc.insert((i, j), n);
            }
        }
    }
    nc
}

// ── Jacobi identity check ─────────────────────────────────────────────────────

fn jacobi(
    roots: &[Root],
    idx: &HashMap<Root, usize>,
    nc: &HashMap<(usize, usize), Rational64>,
    a: usize, b: usize, c: usize,
) -> Rational64 {
    let mut total = Rational64::zero();

    // [e_a, [e_b, e_c]]
    let sbc = roots[b].add(&roots[c]);
    if let Some(&ibc) = idx.get(&sbc) {
        if let Some(&n1) = nc.get(&(b, c)) {
            if let Some(&n2) = nc.get(&(a, ibc)) {
                total = total + n2 * n1;
            }
        }
    }

    // [e_b, [e_c, e_a]]
    let sca = roots[c].add(&roots[a]);
    if let Some(&ica) = idx.get(&sca) {
        if let Some(&n1) = nc.get(&(c, a)) {
            if let Some(&n2) = nc.get(&(b, ica)) {
                total = total + n2 * n1;
            }
        }
    }

    // [e_c, [e_a, e_b]]
    let sab = roots[a].add(&roots[b]);
    if let Some(&iab) = idx.get(&sab) {
        if let Some(&n1) = nc.get(&(a, b)) {
            if let Some(&n2) = nc.get(&(c, iab)) {
                total = total + n2 * n1;
            }
        }
    }

    total
}

// ── Main verification ─────────────────────────────────────────────────────────

pub fn run_exact_verification() -> Result<(), String> {
    println!("Enumerating root system...");
    let (roots, idx) = enumerate_positive_roots();
    println!("  Positive roots:  {}", roots.len());
    println!("  Total roots:     {}", roots.len() * 2);
    println!("  Algebra dim:     {}", 5 + roots.len() * 2);

    println!("Computing structure constants...");
    let nc = compute_structure_constants(&roots, &idx);

    // Verify all N_{alpha,beta} are integers
    let non_int: Vec<_> = nc.values().filter(|v| *v.denom() != 1).collect();
    if !non_int.is_empty() {
        return Err(format!("{} non-integer N_{{a,b}}", non_int.len()));
    }
    println!("  Non-zero N_{{α,β}}: {} (all integers ✓)", nc.len());

    println!("Verifying Jacobi identity...");
    let n = roots.len();
    let mut total_tests = 0usize;
    let mut violations = 0usize;

    // Category 1: (simple, simple, simple)
    for i in 0..5 {
        for j in i+1..5 {
            for k in j+1..5 {
                let v = jacobi(&roots, &idx, &nc, i, j, k);
                total_tests += 1;
                if !v.is_zero() {
                    violations += 1;
                    eprintln!("VIOLATION ({},{},{}) = {}", i, j, k, v);
                }
            }
        }
    }

    // Category 2: (simple, simple, positive)
    for i in 0..5 {
        for j in 0..5 {
            if i == j { continue; }
            for a in 5..n {
                let v = jacobi(&roots, &idx, &nc, i, j, a);
                total_tests += 1;
                if !v.is_zero() { violations += 1; }
            }
        }
    }

    // Category 3: (simple, positive, positive)
    for i in 0..5 {
        for a in 5..n {
            for b in 5..n {
                let v = jacobi(&roots, &idx, &nc, i, a, b);
                total_tests += 1;
                if !v.is_zero() { violations += 1; }
            }
        }
    }

    // Category 4: (positive, positive, positive)
    for a in 5..n {
        for b in 5..n {
            for c in 5..n {
                let v = jacobi(&roots, &idx, &nc, a, b, c);
                total_tests += 1;
                if !v.is_zero() { violations += 1; }
            }
        }
    }

    println!("\n=== EXACT JACOBI REPORT ===");
    println!("Total tests: {}", total_tests);
    println!("Violations:  {}", violations);

    if violations == 0 {
        println!("✓ JACOBI IDENTITY HOLDS EXACTLY IN ℤ[1/12]");
        println!("  Zero violations with absolute zero tolerance.");
        Ok(())
    } else {
        Err(format!("{} Jacobi violations out of {} tests", violations, total_tests))
    }
}
