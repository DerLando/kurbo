//! Common mathematical operations

use arrayvec::ArrayVec;

/// Find real roots of cubic equation.
///
/// Assumes c3 is nonzero.
///
/// See: <http://mathworld.wolfram.com/CubicFormula.html>
///
/// Returns values of x for which c0 + c1 x + c2 x² + c3 x³ = 0.
pub fn solve_cubic(c0: f64, c1: f64, c2: f64, c3: f64) -> ArrayVec<[f64; 3]> {
    let mut result = ArrayVec::new();
    let c3_recip = c3.recip();
    let c2 = c2 * c3_recip;
    let c1 = c1 * c3_recip;
    let c0 = c0 * c3_recip;
    let q = c1 * (1.0 / 3.0) - c2 * c2 * (1.0 / 9.0); // Q
    let r = (1.0 / 6.0) * c2 * c1 - (1.0 / 27.0) * c2.powi(3) - c0 * 0.5; // R
    let d = q.powi(3) + r * r; // D
    let x0 = c2 * (1.0 / 3.0);
    if d > 0.0 {
        let sq = d.sqrt();
        let t1 = (r + sq).cbrt() + (r - sq).cbrt();
        result.push(t1 - x0);
    } else if d == 0.0 {
        let t1 = -r.cbrt();
        let x1 = t1 - x0;
        result.push(x1);
        result.push(-2.0 * t1 - x0);
    } else {
        let sq = (-d).sqrt();
        let rho = r.hypot(sq);
        let th = sq.atan2(r) * (1.0 / 3.0);
        let cbrho = rho.cbrt();
        let c = th.cos();
        let ss3 = th.sin() * 3.0f64.sqrt();
        result.push(2.0 * cbrho * c - x0);
        result.push(-cbrho * (c + ss3) - x0);
        result.push(-cbrho * (c - ss3) - x0);
    }
    result
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;
    use crate::common::solve_cubic;

    #[test]
    fn test_solve_cubic() {
        fn verify(mut roots: ArrayVec<[f64; 3]>, expected: &[f64]) {
            //println!("{:?} {:?}", roots, expected);
            assert!(expected.len() == roots.len());
            let epsilon = 1e-6;
            roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for i in 0..expected.len() {
                assert!((roots[i] - expected[i]).abs() < epsilon);
            }
        }
        verify(solve_cubic(-5.0, 0.0, 0.0, 1.0), &[5.0f64.cbrt()]);
        verify(solve_cubic(-5.0, -1.0, 0.0, 1.0), &[1.90416085913492]);
        verify(solve_cubic(0.0, -1.0, 0.0, 1.0), &[-1.0, 0.0, 1.0]);
        verify(solve_cubic(-2.0, -3.0, 0.0, 1.0), &[-1.0, 2.0]);
        verify(solve_cubic(2.0, -3.0, 0.0, 1.0), &[-2.0, 1.0]);
        verify(solve_cubic(2.0 - 1e-12, 5.0, 4.0, 1.0), &[-2.0, -1.0, -1.0]);
        verify(solve_cubic(2.0 + 1e-12, 5.0, 4.0, 1.0), &[-2.0]);
    }
}