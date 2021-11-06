fn main() {
    use rlp_iter::RlpIterator;

    for i in (0..=100).rlp_iter() {
        println!("{}", i);
    }
}
