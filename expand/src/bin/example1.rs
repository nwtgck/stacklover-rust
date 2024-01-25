use stacklover::stacklover;

stacklover! {
    Iterator1,
    fn (dep1: &'static str, dep2: i32) -> impl Iterator<Item=i32> {
        (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 20)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
            .chain([dep1.len() as i32, dep2])
    }
}

// cargo run --bin example1
fn main() {}
