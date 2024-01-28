use stacklover::wip_define_struct;

wip_define_struct! {
    Iterator1,
    fn (dep1: &'static str, dep2: i32) -> Result<impl Iterator<Item=i32>, std::io::Error> {
        let iter = (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 20)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
            .chain([dep1.len() as i32, dep2]);
        Ok(iter)
    },
    inner_type = impl Iterator<Item=i32>,
    wrapped_type = Result<__Inner__, std::io::Error>,
    handle_wrapped = |result, inner_to_struct| { result.map(|inner| inner_to_struct(inner)) },
}

// cargo run --bin example3
fn main() {
    let result: Result<Iterator1, std::io::Error> = Iterator1::new("hello", 10);
    for i in result.unwrap().into_inner() {
        println!("i={}", i);
    }
}
