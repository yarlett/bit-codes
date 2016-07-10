use rand::distributions::IndependentSample;
use rand::distributions::normal::Normal;
use rand::isaac::{Isaac64Rng};
use std::collections::HashMap;
use utils::FastHasher;


#[derive(Debug)]
pub struct EncodingOptions {
    downcase: bool,
    ngram_lengths: Vec<usize>,
    num_bits: usize,
    num_features: usize,
    random_projections: Vec<Vec<f64>>,
}


impl EncodingOptions {
    pub fn new(downcase: bool, ngram_lengths: Vec<usize>, num_bits: usize, num_features: usize) -> Self {
        let random_projections = get_random_projection_vectors(num_features, num_bits);
        EncodingOptions {
            downcase: downcase,
            ngram_lengths: ngram_lengths,
            num_bits: num_bits,
            num_features: num_features,
            random_projections: random_projections,
        }
    }

    pub fn default() -> Self {
        let downcase = true;
        let ngram_lengths = vec![3, 4, 5, 6];
        let num_bits = 256;
        let num_features = 10000;
        let random_projections = get_random_projection_vectors(num_features, num_bits);
        EncodingOptions {
            downcase: downcase,
            ngram_lengths: ngram_lengths,
            num_bits: num_bits,
            num_features: num_features,
            random_projections: random_projections,
        }
    }

    #[inline]
    pub fn downcase(&self) -> bool { self.downcase }

    pub fn ngram_lengths(&self) -> &Vec<usize> { &self.ngram_lengths }

    #[inline]
    pub fn num_bits(&self) -> usize { self.num_bits }

    #[inline]
    pub fn num_features(&self) -> usize { self.num_features }

    #[inline]
    pub fn project(&self, features: &HashMap<usize, f64, FastHasher>, bit: usize) -> bool {
        let mut acc: f64 = 0.0;
        for (f, w) in features.iter() {
            acc += self.random_projections[bit][*f] * w;
        }
        if acc > 0.0 { true } else { false }
    }
}


pub fn get_random_projection_vectors(num_features: usize, num_bits: usize) -> Vec<Vec<f64>> {
    let mut rng = Isaac64Rng::new_unseeded();
    let normal = Normal::new(0.0, 1.0);
    let mut vectors: Vec<Vec<f64>> = Vec::with_capacity(num_bits);
    for _ in 0..num_bits {
        let mut v: Vec<f64> = vec![0.0; num_features];
        for i in 0..num_features {
            v[i] = normal.ind_sample(&mut rng);
        }
        vectors.push(v);
    }
    vectors
}


#[cfg(test)]
mod tests {
    use super::{EncodingOptions, get_random_projection_vectors};

    #[test]
    fn random_projections() {
        let (nf, nb) = (500, 256);
        let rps1 = get_random_projection_vectors(nf, nb);
        let rps2 = get_random_projection_vectors(nf, nb);
        for b in 0..nb {
            for f in 0..nf {
                assert_eq!(rps1[b][f], rps2[b][f]);
            }
        }
    }

    #[test]
    fn encoding_options() {
        let downcase = true;
        let (nf, nb) = (500, 256);
        let ngram_lengths = vec![3, 4, 5, 6];
        let encoding_options = EncodingOptions::new(downcase, ngram_lengths, nb, nf);
        assert_eq!(encoding_options.num_bits(), nb);
        assert_eq!(encoding_options.num_features(), nf);
        assert_eq!(encoding_options.random_projections.len(), nb);
        for v in &encoding_options.random_projections {
            assert_eq!(v.len(), nf);
        }
    }
}
