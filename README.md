# bit-codes

bit-codes is a Rust library for generating similarity-preserving bit codes from strings describing structured or unstructured data, and performing fast Hamming search on these bit codes. This allows for similarity to be computed over large and complex data structures in a time and space efficient manner.

The library uses the [hashing trick](https://en.wikipedia.org/wiki/Feature_hashing) and [random projections](https://en.wikipedia.org/wiki/Random_projection) applied to string sub-features (character ngrams) to generate [locality-sensitive hashes](https://en.wikipedia.org/wiki/Locality-sensitive_hashing) of these items. The resulting bit codes are highly compressed similarity-preserving representations of the original items. Because bit codes are small, Hamming distance can be computed on these items quickly, which allows similar items, possible duplicates or near duplicates, to be identified rapidly.

We also implement [Fast Exact Search in Hamming Space with Multi-Index Hashing](http://arxiv.org/pdf/1307.2982.pdf) so that pools of bit codes can be searched in sublinear time. This potentially permits millions or billions of bit codes to be stored in RAM and to be queried in real-time (<100ms).

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
    let random_projs = bit_codes::random_projections::RandomProjections::new(num_features, num_bits);
    let bit_code = bit_codes::encoders::string_to_bit_code(&string, &random_projs).unwrap();
    println!("{:?}", bit_code);
}
```

The resulting bit code will be represented by 4 64-bit unsigned integers:

```rust
BitCode { blocks: [7362129119163033604, 18080254231187725207, 2073496217670817622, 15739700542835670175] }
```

### Creating A Bit Code Pool

For most applications the primary data structure exposed by the library is the BitCodePool. A BitCodePool allows compact bit codes to be derived from string representations and stored in the pool so that they can subsequently be queried and retrieved.

The following example shows how a BitCodePool can be created; how bit codes can be added to the pool based on arbitrary string representations; and how a multi-index can be created for codes in the pool. See [here](http://arxiv.org/pdf/1307.2982.pdf) for further information about the type of index used.

```rust
extern crate bit_codes;

fn main() {
    // Initialize bit code pool.
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(1_000, 256);
    // Insert some bit codes into the pool.
    for id in 0..10_000 {
        let string = bit_codes::utils::random_string(100);
        bit_code_pool.add(&string, id);
    }
    // Index the bit pool using multi-index hashing.
    bit_code_pool.index();
    println!("{:?}", bit_code_pool.index_show());
}
```
