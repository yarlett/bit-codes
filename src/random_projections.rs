use rand::distributions::IndependentSample;
use rand::distributions::normal::Normal;
use rand::isaac::{Isaac64Rng};


/// Random Projections struct allows random projection vectors to be generated via an iterator.
///
/// The struct needs to store the input dimensionality of the feature vectors that will be fed into it, so it knows how long the random projection vectors it generates need to be.
///
/// The struct also needs to store an instance of the RNG used to generate the random projection values. We use Isaac64Rng because it guarantees to generate identical sequences when seeded with constant initial values. This means that we only have to use a fixed seed to initialize the struct (which is what ::new_unsseded() does) to avoid having to persist the generated random projections between runs.
pub struct RandomProjections {
    dim_in: usize,
    rng: Isaac64Rng,
}

impl RandomProjections {
    pub fn new(dim_in: usize) -> RandomProjections {
        RandomProjections{ dim_in: dim_in, rng: Isaac64Rng::new_unseeded() }
    }
}


/// Implement Iterator for RandomProjections so we can use it as a generator.
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


/// Return a fixed set of random projections (useful so we don't have to generate the random projections every time we want to encode a string to a bit vector).
pub fn get_random_projections(dim_in: usize, dim_out: usize) -> Vec<Vec<f64>> {
    let mut random_projections: Vec<Vec<f64>> = Vec::new();
    for rp in RandomProjections::new(dim_in).take(dim_out) {
        random_projections.push(rp);
    }
    random_projections
}


#[cfg(test)]
mod tests {
    use super::{get_random_projections, RandomProjections};

    #[test]
    fn random_projections_generate() {
        let (nd, nb) = (500, 256);
        let rps1 = get_random_projections(nd, nb);
        let rps2 = get_random_projections(nd, nb);
        for b in 0..nb {
            for d in 0..nd {
                assert_eq!(rps1[b][d], rps2[b][d]);
            }
        }
    }

    #[test]
    fn random_projections_iterate() {
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
