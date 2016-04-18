use rand::distributions::IndependentSample;
use rand::distributions::normal::Normal;
use rand::isaac::{Isaac64Rng};


#[derive(Debug)]
pub struct RandomProjections {
    dim_in: usize,
    dim_out: usize,
    vectors: Vec<Vec<f64>>,
}


impl RandomProjections {
    pub fn new(dim_in: usize, dim_out: usize) -> Self {
        let vectors = get_random_projection_vectors(dim_in, dim_out, None);
        RandomProjections{ dim_in: dim_in, dim_out: dim_out, vectors: vectors }
    }

    #[inline]
    pub fn dim_in(&self) -> usize {
        self.dim_in
    }

    pub fn project(&self, features: Vec<f64>) -> Option<Vec<bool>> {
        if features.len() != self.dim_in { return None; }
        let mut bits: Vec<bool> = Vec::new();
        for v in &self.vectors {
            let mut acc: f64 = 0.0;
            for (i, f) in features.iter().enumerate() { acc += v[i] * f; }
            bits.push(acc > 0.0);
        }
        Some(bits)
    }
}


pub fn get_random_projection_vectors(dim_in: usize, dim_out: usize, seed: Option<u64>) -> Vec<Vec<f64>> {
    let mut rng = Isaac64Rng::new_unseeded();
    // TODO: Write code to set seed when one is provided.
    // match seed {
    //     Some(x) => set seed of rng here!,
    //     _ => (),
    // }
    let normal = Normal::new(0.0, 1.0);
    let mut vectors: Vec<Vec<f64>> = Vec::new();
    for _ in 0..dim_out {
        let mut v: Vec<f64> = vec![0.0; dim_in];
        for i in 0..dim_in {
            v[i] = normal.ind_sample(&mut rng);
        }
        vectors.push(v);
    }
    vectors
}


#[cfg(test)]
mod tests {
    use super::{get_random_projection_vectors};

    #[test]
    fn random_projections_generate() {
        let (nd, nb) = (500, 256);
        let rps1 = get_random_projection_vectors(nd, nb, None);
        let rps2 = get_random_projection_vectors(nd, nb, None);
        for b in 0..nb {
            for d in 0..nd {
                assert_eq!(rps1[b][d], rps2[b][d]);
            }
        }
    }
}
