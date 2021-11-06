# rlp-iter

![demo.gif](https://raw.githubusercontent.com/Nessex/rlp-iter/master/demo.gif)

rlp-iter (Resolving Lattice Point Iterator) is an iterator that returns a space-filling permutation of integers in a given range. Specifically, it emits integers roughly in order of the most distant integer from any other previously emitted integer.

 - Iterates over all values in a range (e.g. `0..=100`)
 - Follows a space filling pattern
 - No duplicate values

#### Example

Say you need an iterator over the range `0..=100`. This will emit integers in roughly the following order:

```
[ 0, 100, 50, 25, 75, 13, 38, 63, 88, ... ]
```

## Usage

This iterator works on inclusive and exclusive ranges of `usize`. You can access it via:

```rust
use rlp_iter::RlpIterator;

for i in (0..=100).rlp_iter() {
    println!("{}", i);
}
```

## Overhead

This requires a small constant amount of memory, plus one bit of memory per value in the sampled space (required to ensure there are no duplicate values emitted).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
