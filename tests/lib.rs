use futures::{SinkExt as _, StreamExt as _};
use stacklover::stacklover;

#[test]
fn it_works() {
    const STRUCT_SIZE: usize = 32;
    stacklover! {
        Iterator1,
        STRUCT_SIZE,
        fn (dep1: String, dep2: i32) -> impl Iterator<Item=i32> {
            vec![1, 2, 3, dep1.len() as i32, dep2].into_iter().map(|x| x * 2)
        }
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let iter = Iterator1::new("hello".to_string(), 100);
        tx.send(iter).unwrap();
    });

    let iter = rx.recv().unwrap().into_entity();
    assert_eq!(
        iter.into_iter().collect::<Vec<i32>>(),
        vec![2, 4, 6, 10, 200]
    );
}

#[tokio::test]
async fn works_with_async() {
    stacklover! {
        Iterator2,
        32,
        async fn (dep1: &'static str, dep2: i32) -> impl Iterator<Item=i32> {
            vec![1, 2, 3, dep1.len() as i32, dep2].into_iter().map(|x| x * 2)
        }
    }

    let (mut tx, mut rx) = futures::channel::mpsc::channel(1);

    tokio::spawn(async move {
        let iter = Iterator2::new("hello", 100).await;
        tx.send(iter).await.unwrap();
    });

    let iter = rx.next().await.unwrap().into_entity();
    assert_eq!(
        iter.into_iter().collect::<Vec<i32>>(),
        vec![2, 4, 6, 10, 200]
    );
}
