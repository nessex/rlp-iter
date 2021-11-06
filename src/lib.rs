//! # rlp-iter
//!
//! ![demo.gif](https://raw.githubusercontent.com/Nessex/rlp-iter/master/demo.gif)
//!
//! rlp-iter (Resolving Lattice Point Iterator) is an iterator that returns a space-filling permutation of integers in a given range. Specifically, it emits integers roughly in order of the most distant integer from any other previously emitted integer.
//!
//!  - Iterates over all values in a range (e.g. `0..=100`)
//!  - Follows a space filling pattern
//!  - No duplicate values
//!
//! #### Example
//!
//! Say you need an iterator over the range `0..=100`. This will emit integers in roughly the following order:
//!
//! ```text
//! [ 0, 100, 50, 25, 75, 13, 38, 63, 88, ... ]
//! ```
//!
//! ## Usage
//!
//! This iterator works on inclusive and exclusive ranges of `usize`. You can access it via:
//!
//! ```rust
//! use rlp_iter::RlpIterator;
//!
//! for i in (0..=100).rlp_iter() {
//!     println!("{}", i);
//! }
//! ```
//!
//! ## Overhead
//!
//! This requires a small constant amount of memory, plus one bit of memory per value in the sampled space (required to ensure there are no duplicate values emitted).
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!

use bit_vec::BitVec;
use std::ops::{Range, RangeInclusive};

enum State {
    Start,
    End,
    Lattice,
    Finished,
}

pub struct RlpIter {
    tested: BitVec,
    shift: usize,
    range: usize,
    numerator: usize,
    pow: usize,
    final_pow: usize,
    state: State,
}

// NOTE(nathan): This should be replaced with the builtin log2
// once it is stabilized.
//
// https://github.com/rust-lang/rust/issues/70887
fn ilog2(i: usize) -> usize {
    (i as f64).log2().round() as usize
}

impl Iterator for RlpIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let unshifted = match self.state {
            State::Start => {
                self.state = State::End;
                self.tested.set(0, true);
                Some(0)
            }
            State::End => {
                if self.range == 0 {
                    self.state = State::Finished;
                    None
                } else {
                    self.state = State::Lattice;
                    self.tested.set(self.range, true);
                    Some(self.range)
                }
            }
            State::Lattice => {
                let mut out = None;

                while self.pow <= self.final_pow {
                    // Calculate next value
                    let denominator = (1_u64 << self.pow) as usize;
                    let val = (self.range as f64 * (self.numerator as f64 / denominator as f64))
                        .round() as usize;

                    if !self.tested.get(val).unwrap() {
                        out = Some(val);
                        self.tested.set(val, true);
                    }

                    // Increment numerator / denominator
                    if self.numerator == denominator - 1 {
                        self.numerator = 1;
                        self.pow += 1;
                    } else {
                        self.numerator += 1;
                    }

                    if out.is_some() {
                        break;
                    }
                }

                if out.is_some() {
                    out
                } else {
                    // Fill gaps with simple iteration
                    // This is equivalent to doing the next pow, but with less redundant checks
                    while self.numerator <= self.range {
                        if !self.tested.get(self.numerator).unwrap() {
                            out = Some(self.numerator);
                            self.tested.set(self.numerator, true);
                            self.numerator += 1;
                            break;
                        } else {
                            self.numerator += 1;
                        }
                    }

                    // Progress to finished
                    if self.numerator > self.range {
                        self.state = State::Finished;
                    }

                    out
                }
            }
            State::Finished => None,
        };

        unshifted.map(|v| v + self.shift)
    }
}

pub trait RlpIterator {
    fn rlp_iter(&self) -> RlpIter;
}

impl RlpIterator for Range<usize> {
    fn rlp_iter(&self) -> RlpIter {
        let range = self.end - self.start - 1;
        RlpIter {
            tested: BitVec::from_elem(range + 1, false),
            shift: self.start,
            range,
            numerator: 1,
            pow: 1,
            final_pow: ilog2(range),
            state: State::Start,
        }
    }
}

impl RlpIterator for RangeInclusive<usize> {
    fn rlp_iter(&self) -> RlpIter {
        let range = self.end() - self.start();
        RlpIter {
            tested: BitVec::from_elem(range + 1, false),
            shift: *self.start(),
            range,
            numerator: 1,
            pow: 1,
            final_pow: ilog2(range),
            state: State::Start,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RlpIterator;

    #[test]
    fn inclusive_range_works() {
        let out: Vec<usize> = (0..=8).rlp_iter().collect();
        assert_eq!(out[..], [0, 8, 4, 2, 6, 1, 3, 5, 7]);
    }

    #[test]
    fn exclusive_range_works() {
        let out: Vec<usize> = (0..9).rlp_iter().collect();
        assert_eq!(out[..], [0, 8, 4, 2, 6, 1, 3, 5, 7]);
    }

    #[test]
    fn offset_works() {
        let out: Vec<usize> = (1..=9).rlp_iter().collect();
        assert_eq!(out[..], [1, 9, 5, 3, 7, 2, 4, 6, 8]);
    }

    #[test]
    fn extreme_offset_inclusive_works() {
        let out: Vec<usize> = (1_000..=1_008).rlp_iter().collect();
        assert_eq!(
            out[..],
            [1_000, 1_008, 1_004, 1_002, 1_006, 1_001, 1_003, 1_005, 1_007]
        );
    }

    #[test]
    fn extreme_offset_exclusive_works() {
        let out: Vec<usize> = (1_000..1_009).rlp_iter().collect();
        assert_eq!(
            out[..],
            [1_000, 1_008, 1_004, 1_002, 1_006, 1_001, 1_003, 1_005, 1_007]
        );
    }

    #[test]
    fn output_inclusive_is_complete() {
        let mut out: Vec<usize> = (7..=7919).rlp_iter().collect();
        let expected: Vec<usize> = (7..=7919).into_iter().collect();

        out.sort();
        assert_eq!(expected, out);
    }

    #[test]
    fn output_exclusive_is_complete() {
        let mut out: Vec<usize> = (7..7919).rlp_iter().collect();
        let expected: Vec<usize> = (7..7919).into_iter().collect();

        out.sort();
        assert_eq!(expected, out);
    }

    #[test]
    fn test_readme_example() {
        let mut out: Vec<usize> = (0..=100).rlp_iter().collect();

        assert_eq!(out[0..9], [0, 100, 50, 25, 75, 13, 38, 63, 88]);
    }
}
