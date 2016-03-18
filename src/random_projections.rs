use rand::distributions::IndependentSample;
use rand::distributions::normal::Normal;
use rand::isaac::{Isaac64Rng};


pub struct RandomProjections {
    dim_in: usize,
    rng: Isaac64Rng,
}


impl RandomProjections {
    pub fn new(dim_in: usize) -> RandomProjections {
        // Use Isaac64Rng because new_unseeded uses the same constant seed and hence the sequence of pseudorandom projection vectors from different initialized instances should be the same (avoids having to persist random projections between runs).
        RandomProjections{ dim_in: dim_in, rng: Isaac64Rng::new_unseeded() }
    }
}


impl Iterator for RandomProjections {
    type Item = Vec<f64>;
    fn next (&mut self) -> Option<Vec<f64>> {
        let mut v: Vec<f64> = vec![0.0; self.dim_in];
        for i in 0..self.dim_in {
            let normal = Normal::new(0.0, 1.0);
            v[i] = normal.ind_sample(&mut self.rng);
        }
        return Some(v)
    }
}


#[cfg(test)]
mod tests {
    use super::RandomProjections;

    #[test]
    fn yields_same_random_projections() {
        let n: usize = 30;
        let rp1 = RandomProjections::new(n);
        let rps1: Vec<Vec<f64>> = rp1.take(10).collect();
        let rp2 = RandomProjections::new(n);
        let rps2: Vec<Vec<f64>> = rp2.take(10).collect();
        for i in 0..rps1.len() {
            assert_eq!(rps1[i].len(), n);
            assert_eq!(rps2[i].len(), n);
            for j in 0..rps1[i].len() {
                assert_eq!(rps1[i][j], rps2[i][j]);
            }
        }
    }
}
