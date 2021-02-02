/// Quadratic Polynomial Permutation (QPP) Interleaver.
/// Permutation is computed using the formula:
///    pi(i) = (f1 * i + f2 * i^2) mod k.
/// The equation can be rewritten as the following recursive expression:
///    pi(i+1) = (pi(i) + g(i)) mod k,
/// where
///    g(i+1) = (g(i) + (2f2 mod k)) mod k,
/// with `2f2 mod k` being constant for each iteration.
#[derive(Clone)]
pub struct Qpp {
    /// The block length.
    k: usize,
    /// The f1 parameter.
    f1: usize,
    /// The f2 parameter
    f2: usize,
}

impl Qpp {
    pub const fn new(k: usize, f1: usize, f2: usize) -> Self {
        Self { k, f1, f2 }
    }

    /// Get the interleaved index.
    /// It is slower to call this function `k` times than iterating the entire
    /// permuted sequence.
    pub const fn pi(&self, i: usize) -> usize {
        (self.f1 * i + self.f2 * i * i) % self.k
    }

    /// Get an iterator that produces the permuted sequence.
    /// It produces `k` permutations and is faster than invoking `pi` `k` times.
    pub fn iter(&self) -> QppIterator {
        QppIterator {
            k: self.k,
            two_f2_mod_k: (2 * self.f2) % self.k,
            pi: 0,
            g: (self.f1 + self.f2) % self.k,
            i: 0,
        }
    }
}

impl IntoIterator for Qpp {
    type Item = usize;

    type IntoIter = QppIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct QppIterator {
    k: usize,
    two_f2_mod_k: usize,
    pi: usize,
    g: usize,
    i: usize,
}

impl Iterator for QppIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.i < self.k {
            let pi = self.pi;
            let g = self.g;

            self.pi = (pi + g) % self.k;
            self.g = (g + self.two_f2_mod_k) % self.k;
            self.i += 1;

            Some(pi)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.k, Some(self.k))
    }
}

impl ExactSizeIterator for QppIterator {}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn iterator_and_pi_are_the_same() {
        let qpp = Qpp::new(16, 1, 4);
        for (i, int) in qpp.iter().enumerate() {
            assert_eq!(qpp.pi(i), int);
        }

        let qpp = Qpp::new(40, 3, 10);
        for (i, int) in qpp.iter().enumerate() {
            assert_eq!(qpp.pi(i), int);
        }
    }
}
