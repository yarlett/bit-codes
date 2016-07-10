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
    let downcase = true;
    let ngram_lengths = vec![3, 4, 5, 6];
    let num_bits = 256;
    let num_features = 500;
    let encoding_options = bit_codes::encoding_options::EncodingOptions::new(
        downcase,
        ngram_lengths,
        num_bits,
        num_features,
    );
    let bit_code = bit_codes::encoding::string_to_bit_code(&string, &encoding_options);
    println!("{:?}", bit_code);
}```

The resulting bit code will be represented by 4 64-bit unsigned integers:

```rust
BitCode { blocks: [3873682049634234218, 631966384541388943, 951393676992109676, 5760212978568696118] }
```

### Creating A Bit Code Pool

For most applications the primary data structure exposed by the library is the BitCodePool. A BitCodePool allows compact bit codes to be derived from string representations and stored in the pool so that they can subsequently be queried and retrieved.

The following example shows how a BitCodePool can be created; how bit codes can be added to the pool based on arbitrary string representations; and how a multi-index can be created for codes in the pool. See [here](http://arxiv.org/pdf/1307.2982.pdf) for further information about the type of index used.

```rust
extern crate bit_codes;

fn main() {
    // Parameters.
    let encoding_options = bit_codes::encoding_options::EncodingOptions::default();
    let num_items = 10_000;
    let string_length = 25;
    // Initialize bit code pool.
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(encoding_options);
    // Insert some bit codes into the pool.
    for id in 0..num_items {
        let string = bit_codes::utils::random_string(string_length);
        bit_code_pool.add(&string, id);
    }
}
```

### Entity Resolution

Entity resolution is the process of determining, amongst a set of records, which records refer to identical entities. In cases where a set of records are determined to refer to the same entity the records can either be identical (in which case we are essentially detecting duplicates in our data), or non-identical but similar (in which case the records represent noisy, corrupt or different descriptions of the same entity). In addition the records can be from a single homogeneous pool of records (such as a single database table), or from diverse sources (in which case we are solving a record linkage problem). We use 'entity resolution' as a cover-all term for all these cases.

The bit codes library allows entity resolution to be performed over a bit code pool, by grouping bit codes within a certain radius of one another together into an entity set. The idea is that bit codes that are surprisingly similar to one another are likely to correspond to highly similar string representations which, in turn, are likely to describe similar or identical entities. Obviously the hamming radius used to determine entity sets will vary depending on the application.

The following code, found in [examples/resolve_entities.rs](examples/resolve_entities.rs), creates a bit code pool from short randomly generated strings, and then groups the corresponding bit codes into entity sets based on a Hamming radius threshold:

```rust
extern crate bit_codes;
extern crate time;

fn main() {
    // Parameters.
    let downcase = true;
    let ngram_lengths = vec![1, 2, 3, 4, 5];
    let num_bits = 128;
    let num_features = 1_000;
    let num_items = 50_000;
    let radius = 25;
    let string_length = 5;
    // Create random strings.
    let mut strings: Vec<String> = Vec::new();
    for _ in 0..num_items {
        strings.push(bit_codes::utils::random_string(string_length));
    }
    // Create bit code pool from random strings.
    let encoding_options = bit_codes::encoding_options::EncodingOptions::new(downcase, ngram_lengths, num_bits, num_features);
    let mut bit_code_pool = bit_codes::bit_code_pool::BitCodePool::new(encoding_options);
    for i in 0..strings.len() { bit_code_pool.add(&strings[i], i as u64); }
    // Resolve entities in bit code pool.
    let t1 = time::precise_time_s();
    let entity_sets = bit_code_pool.resolve_entities(radius);
    let t2 = time::precise_time_s();
    let t_s = format!("{:.*}", 3, t2 - t1);
    for entity_set in &entity_sets {
        if entity_set.len() > 1 {
            for i in entity_set { println!("{:?}", strings[*i]); }
            println!("");
        }
    }
    println!("Resolved entities into {:} entity sets of bit code pool in {:}s.", entity_sets.len(), t_s);
}
```

The code generates output like that shown below. As can be seen, it successfully identifies random strings that are unusually similar to one another. In a real application these strings could refer to similar or identical entities.

```
"8VZWO"
"8vzWo"

"oCal1"
"OcaL1"

"JOGLe"
"JOgLe"

"rUQqi"
"YRuqq"

"3hsGI"
"3HSgi"

"SQCWo"
"osQCW"

"IF1u4"
"if1U4"

"FOyPj"
"fOYPJ"

"QFlL9"
"lqfll"

"PPcEa"
"PpCea"

"3gzsL"
"3gZsL"

"ZFzOl"
"zfzol"

"y1jai"
"y1Jai"

"KVvek"
"kVveK"

"pkzzB"
"pkZZb"

"UaOhy"
"uaoHy"

"5oaSb"
"5oasb"

"QCg9E"
"qCG9g"

"6Pxua"
"6PxUA"

"d2RBV"
"D2RBV"

"8uopm"
"SuOpM"

"Vg8HM"
"vG8hm"

"XnTUi"
"xNtUi"

"OHbdI"
"OhbDi"

"wrl7c"
"4N7bI"

"IwC43"
"IWC43"

Resolved entities into 49974 entity sets of bit code pool in 30.113s.
```
