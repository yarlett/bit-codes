use bit_code::BitCode;
use random_projections::RandomProjections;


/// Encodes an input string as a BitCode.
pub fn string_to_bit_code(string: &str, random_projections: &RandomProjections) -> BitCode {
    // Make allocations.
    let mut features: Vec<f64> = vec![0.0; random_projections.dim_in()];
    let mut bools: Vec<bool> = vec![false; random_projections.dim_out()];
    // Make and return bit code.
    string_to_bit_code_no_allocation(string, random_projections, &mut features, &mut bools)
}


/// Encodes an input string as a BitCode without unnecessary memory allocation.
pub fn string_to_bit_code_no_allocation(string: &str, random_projections: &RandomProjections, features: &mut Vec<f64>, bools: &mut Vec<bool>) -> BitCode {
    // Set feature vector.
    random_projections.set_feature_vector(string, features);
    // Set bools via random projections of the features.
    random_projections.set_bool_vector(features, bools);
    // Make and return the bit code.
    BitCode::from_bools(bools)
}


#[cfg(test)]
mod tests {
    use random_projections::{RandomProjections};
    use super::string_to_bit_code;
    use test::Bencher;
    use utils::random_string;


    /// Deriving bit code from a string on 2 occasions should yield the same bit code.
    #[test]
    fn test_string_to_bit_code() {
        let string = "Supercalifragilisticexpialidocious";
        let num_features = 500;
        let num_bits = 256;
        let random_projs = RandomProjections::new(num_features, num_bits);
        let bit_code_1 = string_to_bit_code(&string, &random_projs);
        let bit_code_2 = string_to_bit_code(&string, &random_projs);
        println!("{:?}", bit_code_1);
        println!("{:?}", bit_code_2);
        assert_eq!(bit_code_1.hamming_distance(&bit_code_2), 0);
    }

    #[bench]
    fn encode_string(b: &mut Bencher) {
        // Generate random string.
        let random_string = random_string(100);
        let random_projs = RandomProjections::new(500, 256);
        // Benchmark time to encode the strings as bit codes.
        b.iter(|| {
            string_to_bit_code(&random_string, &random_projs)
        });
    }
}
