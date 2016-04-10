# bit-codes

bit-codes is a Rust library for generating similarity-preserving bit codes from strings describing structured or unstructured data, and performing fast Hamming search on these bit codes. This allows for similarity to be computed over large and complex data structures in a time and space efficient manner.

The library uses the [hashing trick](https://en.wikipedia.org/wiki/Feature_hashing) and [random projections](https://en.wikipedia.org/wiki/Random_projection) applied to string sub-features (character ngrams) to generate [locality-sensitive hashes](https://en.wikipedia.org/wiki/Locality-sensitive_hashing) of these items. The resulting bit codes are highly compressed similarity-preserving representations of the original items. Because bit codes are small, Hamming distance can be computed on these items quickly, which allows similar items, possible duplicates or near duplicates, to be identified rapidly.

We also intend to implement [Fast Exact Search in Hamming Space with Multi-Index Hashing](http://arxiv.org/pdf/1307.2982.pdf) so that pools of bit codes can be searched in sublinear time. This would potentially permit millions or billions of bit codes to be stored in RAM and to be queried in real-time (<100ms).

We believe this library will have application to the problems of data de-duplication and entity resolution, and any time nearest neighbor search over a set of items needs to be performed. We also think the library may prove to be useful to machine learning applications with a couple of possible advantages: 1) model sizes (and hence training time and storage costs) can be reduced when the inputs are compressed bit codes, and 2) bit codes may permit machine learning models to be decoupled from data schema changes, a pervasive cause of technical debt in machine learning applications.

## Examples

### Creating BitCodes

The following example shows how a string description of some entity can be converted into a 256-bit bit code. Internally the bit codes are packed into a vector of u64s so they use minimal space.

```rust
extern crate bit_codes;

fn main() {
    let string = "Supercalifragilisticexpialidocious";
    let num_features = 500;
    let num_bits = 256;
    let random_projs = random_projections::get_random_projections(num_features, num_bits);
    let bit_code = string_to_bit_code_via_feature_vector(&string, &random_projs);
    println!("{:?}", bit_code);
}
```

The resulting bit code will be represented by 8 64-bit unsigned integers:

```rust
BitCode { blocks: [7362129119163033604, 18080254231187725207, 2073496217670817622, 15739700542835670175], nbits: 256 }
```
