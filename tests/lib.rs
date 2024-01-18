use stacklover::stacklover;

#[test]
fn it_works() {
    stacklover!(IteratorInStack1, 32,
        (dep1: &'static str) -> impl IntoIterator<Item=i32> {
            vec![1, 2, 3, dep1.len() as i32].into_iter().map(|x| x * 2)
        }
    );

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let x = IteratorInStack1::new("hello");
        tx.send(x).unwrap();
    });

    let iter = rx.recv().unwrap().into_entity();
    assert_eq!(iter.into_iter().collect::<Vec<i32>>(), vec![2, 4, 6, 10]);
}
