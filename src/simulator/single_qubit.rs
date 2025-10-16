use num_complex::Complex64;

/// single qubit quantum state: α|0⟩ + β|1⟩
#[derive(Debug, Clone)]
pub struct SingleQubit {
    pub alpha: Complex64,
    pub beta: Complex64,
}

impl SingleQubit {
    pub fn new() -> Self {
        Self {
            alpha: Complex64::new(1.0, 0.0),
            beta: Complex64::new(0.0, 0.0),
        }
    }

    pub fn new_one() -> Self {
        Self {
            alpha: Complex64::new(0.0, 0.0),
            beta: Complex64::new(1.0, 0.0),
        }
    }

    /// will normalize
    pub fn from_amplitudes(alpha: Complex64, beta: Complex64) -> Self {
        let mut qubit = Self { alpha, beta };
        qubit.normalize();
        qubit
    }

    /// ensure |α|² + |β|² = 1
    pub fn normalize(&mut self) {
        let norm = (self.alpha.norm_sqr() + self.beta.norm_sqr()).sqrt();
        if norm > 1e-10 {
            self.alpha /= norm;
            self.beta /= norm;
        }
    }

    pub fn prob_zero(&self) -> f64 {
        self.alpha.norm_sqr()
    }

    pub fn prob_one(&self) -> f64 {
        self.beta.norm_sqr()
    }

    pub fn apply_gate(&mut self, matrix: [[Complex64; 2]; 2]) {
        let new_alpha = matrix[0][0] * self.alpha + matrix[0][1] * self.beta;
        let new_beta = matrix[1][0] * self.alpha + matrix[1][1] * self.beta;
        self.alpha = new_alpha;
        self.beta = new_beta;
    }

    /// state in ket notation
    pub fn display(&self) {
        println!("State: {:.3}|0⟩ + {:.3}|1⟩", self.alpha, self.beta);
        println!(
            "Probabilities: |0⟩: {:.1}%, |1⟩: {:.1}%",
            self.prob_zero() * 100.0,
            self.prob_one() * 100.0
        );
    }

    pub fn display_with_message(&self, message: &str) {
        println!("\n{}", message);
        self.display();
    }
}

impl Default for SingleQubit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let qubit = SingleQubit::new();
        assert!((qubit.prob_zero() - 1.0).abs() < 1e-10);
        assert!(qubit.prob_one().abs() < 1e-10);
    }

    #[test]
    fn test_normalization() {
        let mut qubit = SingleQubit {
            alpha: Complex64::new(3.0, 0.0),
            beta: Complex64::new(4.0, 0.0),
        };
        qubit.normalize();
        let total_prob = qubit.prob_zero() + qubit.prob_one();
        assert!((total_prob - 1.0).abs() < 1e-10);
    }
}
