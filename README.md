# bit-codes

bit-codes is a Rust library for generating similarity-preserving bit codes from a variety of different types of data (strings at this point in time, eventually key-value structures, JSON...), and performing fast Hamming search on these bit codes. This allows for similarity to be computed over large and complex data structures in a time and space efficient manner.

The library uses the [hashing trick](https://en.wikipedia.org/wiki/Feature_hashing) and [random projections](https://en.wikipedia.org/wiki/Random_projection) applied to string sub-features to generate [locality-sensitive hashes](https://en.wikipedia.org/wiki/Locality-sensitive_hashing) of these items. The resulting bit codes are highly compressed similarity-preserving representations of the original items. Because bit codes are small, Hamming distance can be computed on these items quickly, which allows similar items, possible duplicates or near duplicates, to be identified rapidly. Eventually we intend to implement [Fast Exact Search in Hamming Space with Multi-Index Hashing](http://arxiv.org/pdf/1307.2982.pdf) so that these bit codes can be searched in sublinear time.

We hope this library will have application in the areas of machine learning, data de-duplication, entity resolution, and nearest neighbor search.
