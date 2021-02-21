#[allow(dead_code)]
pub mod qtmcore {
    use std::{
        cmp::{max, min},
        num::Wrapping,
    };

    use nalgebra::{DMatrix, DVector};

    use pyo3::prelude::*;

    fn clamp_xsize<T>(num: T, min_value: T, max_value: T) -> T
    where
        T: Ord,
    {
        max(min(num, max_value), min_value)
    }

    // FIXME: ? -> safe clamp of f64
    fn clamp_f64(num: f64, min_value: f64, max_value: f64) -> f64 {
        use float_ord::FloatOrd;

        let lhs: f64;
        if FloatOrd(num) < FloatOrd(max_value) {
            lhs = num;
        } else {
            lhs = max_value;
        }

        let ret: f64;
        if FloatOrd(lhs) > FloatOrd(min_value) {
            ret = lhs;
        } else {
            ret = min_value;
        }

        ret
    }

    #[pyclass]
    pub struct Qtm {
        #[pyo3(get, set)] channel_count: usize,
        #[pyo3(get, set)] queue_size: usize,
        #[pyo3(get, set)] la: f64,
        #[pyo3(get, set)] mu: f64,
        #[pyo3(get, set)] nu: f64,
        #[pyo3(get, set)] n: isize,
        final_states: Vec<f64>,
    }

    #[pymethods]
    impl Qtm {
        #[new]
        pub fn new(
            channel_count: usize, queue_size: usize,
            la: f64, mu: f64, nu: f64,
            n: isize,
        ) -> Self {
            Qtm {
                channel_count, queue_size,
                la, mu, nu,
                n,
                final_states: vec![],
            }
        }

        pub fn final_states(&self) -> Vec<f64> {
            self.final_states.clone() // FIXME: need to return ref
        }

        pub fn calc_final_states(&mut self) -> Vec<f64> {
            let mut a = self.matrix_init();
            let total_count = self.channel_count + self.queue_size + 1;
            let m_size = total_count + 1;

            a = a.resize(m_size, m_size, 0.);

            for i in 0..m_size - 1 {
                a[(i, m_size - 1)] = 1. / total_count as f64;
                a[(m_size - 1, i)] = 1.;
            }
            a[(m_size - 1, m_size - 1)] = 0.;

            let mut b = vec![];
            for _ in 0..total_count {
                b.push(0.);
            }
            b.push(1.);

            let b = DVector::from_vec(b);

            if !a.try_inverse_mut() {
                panic!("invert error");
            };

            let fs = a * b;

            self.final_states.clear();
            // FIXME: iterate until end-1
            for item in fs.iter() {
                self.final_states.push(*item);
            }
            self.final_states.pop();

            self.final_states()
        }
    }

    // private implementation's
    impl Qtm {
        fn matrix_init(&mut self) -> DMatrix<f64> {
            let total_count = self.channel_count + self.queue_size;
            let mut matrix: DMatrix<f64> = DMatrix::from_vec(1, 1, vec![0.]);
            let mut mu_index: usize = 0;
            let max_n = self.n;

            // formation of mu and la
            for i in 0..total_count + 1 {
                matrix = matrix.resize(i + 1, total_count + 1, 0.);
                for j in 0..total_count + 1 {
                    if i == j + 1 {
                        if self.n != -1 {
                            matrix[(i, j)] =
                                clamp_f64(self.n as f64 / max_n as f64, 0., max_n as f64) * self.la;
                            self.n -= 1;
                        } else {
                            matrix[(i, j)] = self.la;
                        }
                    } else if i == (Wrapping(j) - Wrapping(1)).0 {
                        // FIXME: incorrect when i == usize::max
                        mu_index = clamp_xsize::<usize>(mu_index + 1, 0, self.channel_count);
                        matrix[(i, j)] = mu_index as f64 * self.mu;
                    } else {
                        matrix[(i, j)] = 0.;
                    }
                }
            }

            // formation of nu
            let mut nu_index = self.queue_size as isize; // FIXME: incorrect when self.queue_size > usize::max / 2
            for i in 0..total_count {
                for j in 1..total_count + 1 {
                    if i == j - 1 {
                        matrix[(i, j)] += nu_index as f64 * self.nu;
                        nu_index = clamp_xsize::<isize>(
                            nu_index - 1,
                            0,
                            self.queue_size as isize /* FIXME: incorrect when self.queue_size > usize::max / 2 */ - 1,
                        );
                    }
                }
            }

            // diagonal
            for i in 0..total_count + 1 {
                for j in 0..total_count + 1 {
                    if j != i {
                        matrix[(i, i)] += matrix[(j, i)];
                    }
                }
                matrix[(i, i)] *= -1.;
            }

            matrix
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_check_fs() {
        use super::qtmcore::Qtm;

        let comp = vec![
            0.14285714285714285,
            0.2857142857142857,
            0.2857142857142857,
            0.2857142857142857,
        ];

        let mut x = Qtm::new(2, 1, 10., 5., 0., -1);
        x.calc_final_states();
        let ret = x.final_states();

        assert_eq!(&comp[..], &ret[..]);
    }
}
