use num_complex::Complex64;
use std::f64::consts::PI;
use super::single_qubit::SingleQubit;

// Common constants
const I: Complex64 = Complex64::new(0.0, 1.0);
const SQRT2_INV: f64 = 0.7071067811865476; // 1/√2

/// Pauli-X gate (NOT gate)
/// Flips |0⟩ ↔ |1⟩
pub fn x_gate(qubit: &mut SingleQubit) {
    let matrix = [
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
    ];
    qubit.apply_gate(matrix);
}

/// Pauli-Y gate
pub fn y_gate(qubit: &mut SingleQubit) {
    let matrix = [
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
        [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)],
    ];
    qubit.apply_gate(matrix);
}

/// Pauli-Z gate
/// Applies phase flip: |0⟩ → |0⟩, |1⟩ → -|1⟩
pub fn z_gate(qubit: &mut SingleQubit) {
    let matrix = [
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
    ];
    qubit.apply_gate(matrix);
}

/// Hadamard gate
/// Creates superposition: |0⟩ → (|0⟩ + |1⟩)/√2
pub fn h_gate(qubit: &mut SingleQubit) {
    let matrix = [
        [Complex64::new(SQRT2_INV, 0.0), Complex64::new(SQRT2_INV, 0.0)],
        [Complex64::new(SQRT2_INV, 0.0), Complex64::new(-SQRT2_INV, 0.0)],
    ];
    qubit.apply_gate(matrix);
}

/// Rotation around X-axis by angle theta
pub fn rx_gate(qubit: &mut SingleQubit, theta: f64) {
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();
    let matrix = [
        [Complex64::new(cos, 0.0), Complex64::new(0.0, -sin)],
        [Complex64::new(0.0, -sin), Complex64::new(cos, 0.0)],
    ];
    qubit.apply_gate(matrix);
}

/// Rotation around Y-axis by angle theta
pub fn ry_gate(qubit: &mut SingleQubit, theta: f64) {
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();
    let matrix = [
        [Complex64::new(cos, 0.0), Complex64::new(-sin, 0.0)],
        [Complex64::new(sin, 0.0), Complex64::new(cos, 0.0)],
    ];
    qubit.apply_gate(matrix);
}

/// Rotation around Z-axis by angle theta
pub fn rz_gate(qubit: &mut SingleQubit, theta: f64) {
    let exp_neg = Complex64::new(0.0, -theta / 2.0).exp();
    let exp_pos = Complex64::new(0.0, theta / 2.0).exp();
    let matrix = [
        [exp_neg, Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), exp_pos],
    ];
    qubit.apply_gate(matrix);
}

/// Phase gate (S gate)
/// Applies: |0⟩ → |0⟩, |1⟩ → i|1⟩
pub fn s_gate(qubit: &mut SingleQubit) {
    let matrix = [
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), I],
    ];
    qubit.apply_gate(matrix);
}

/// T gate (π/8 gate)
pub fn t_gate(qubit: &mut SingleQubit) {
    let phase = Complex64::new(0.0, PI / 4.0).exp();
    let matrix = [
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), phase],
    ];
    qubit.apply_gate(matrix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_gate() {
        let mut qubit = SingleQubit::new();
        x_gate(&mut qubit);
        assert!((qubit.prob_zero() - 0.0).abs() < 1e-10);
        assert!((qubit.prob_one() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_h_gate() {
        let mut qubit = SingleQubit::new();
        h_gate(&mut qubit);
        assert!((qubit.prob_zero() - 0.5).abs() < 1e-10);
        assert!((qubit.prob_one() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_x_twice_identity() {
        let mut qubit = SingleQubit::new();
        x_gate(&mut qubit);
        x_gate(&mut qubit);
        assert!((qubit.prob_zero() - 1.0).abs() < 1e-10);
    }
}
