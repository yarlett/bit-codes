use rand::distributions::IndependentSample;
use rand::distributions::normal::Normal;
use rand::isaac::{Isaac64Rng};
use string_features::get_string_features;


#[derive(Debug)]
pub struct RandomProjections {
    dim_in: usize,
    dim_out: usize,
    ngram_lengths: Vec<usize>,
    vectors: Vec<Vec<f64>>,
}


impl RandomProjections {
    pub fn new(dim_in: usize, dim_out: usize, ngram_lengths: Vec<usize>) -> Self {
        let vectors = get_random_projection_vectors(dim_in, dim_out);
        RandomProjections{ dim_in: dim_in, dim_out: dim_out, ngram_lengths: ngram_lengths, vectors: vectors }
    }

    pub fn default(dim_in: usize, dim_out: usize) -> Self {
        let ngram_lengths = vec![3, 4, 5, 6, 7, 8];
        let vectors = get_random_projection_vectors(dim_in, dim_out);
        RandomProjections{ dim_in: dim_in, dim_out: dim_out, ngram_lengths: ngram_lengths, vectors: vectors }
    }

    #[inline]
    pub fn dim_in(&self) -> usize {
        self.dim_in
    }

    #[inline]
    pub fn dim_out(&self) -> usize {
        self.dim_out
    }

    #[inline]
    pub fn set_bool_vector(&self, features: &Vec<f64>, bools: &mut Vec<bool>) {
        for i in 0..self.vectors.len() {
            let mut acc: f64 = 0.0;
            for j in 0..self.vectors[i].len() { acc += self.vectors[i][j] * features[j]; }
            bools[i] = acc > 0.0;
        }
    }

    #[inline]
    pub fn set_feature_vector(&self, string: &str, features: &mut Vec<f64>) {
        let n = features.len();
        let nu64 = n as u64;
        let ngram_lengths = &vec![3,4,5,6,7,8];
        // Reset features.
        for i in 0..n { features[i] = 0.0; }
        // Update features using (hash_value, weight) tuples.
        for (hash_value, weight) in get_string_features(string, ngram_lengths) {
            let bin = (hash_value % nu64) as usize;
            features[bin] += weight;
        }
    }
}


pub fn get_random_projection_vectors(dim_in: usize, dim_out: usize) -> Vec<Vec<f64>> {
    let mut rng = Isaac64Rng::new_unseeded();
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
        let rps1 = get_random_projection_vectors(nd, nb);
        let rps2 = get_random_projection_vectors(nd, nb);
        for b in 0..nb {
            for d in 0..nd {
                assert_eq!(rps1[b][d], rps2[b][d]);
            }
        }
    }
}
