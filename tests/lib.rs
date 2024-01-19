use futures::{SinkExt as _, StreamExt as _};
use stacklover::stacklover;

#[test]
fn it_works() {
    stacklover!(Iterator1, 32,
        (dep1: &'static str) -> impl Iterator<Item=i32> {
            vec![1, 2, 3, dep1.len() as i32].into_iter().map(|x| x * 2)
        }
    );

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let iter = Iterator1::new("hello");
        tx.send(iter).unwrap();
    });

    let iter = rx.recv().unwrap().into_entity();
    assert_eq!(iter.into_iter().collect::<Vec<i32>>(), vec![2, 4, 6, 10]);
}

#[tokio::test]
async fn works_with_async() {
    stacklover!(Iterator2, 32,
        async (dep1: &'static str) -> impl Iterator<Item=i32> {
            vec![1, 2, 3, dep1.len() as i32].into_iter().map(|x| x * 2)
        }
    );

    let (mut tx, mut rx) = futures::channel::mpsc::channel(1);

    tokio::spawn(async move {
        let iter = Iterator2::new("hello").await;
        tx.send(iter).await.unwrap();
    });

    let iter = rx.next().await.unwrap().into_entity();
    assert_eq!(iter.into_iter().collect::<Vec<i32>>(), vec![2, 4, 6, 10]);
}
