stacklover::define_struct! {
    I32,
    fn (dep2: i32) -> i32 {
        dep2
    },
    derive = ( PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug ),
}

// cargo run --bin example4
fn main() {}
