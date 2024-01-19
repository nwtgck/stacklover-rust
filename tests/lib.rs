use futures::{SinkExt as _, StreamExt as _};
use stacklover::stacklover;
use std::mem::{size_of, MaybeUninit};

#[test]
fn it_works() {
    stacklover! {
        // struct name to be defined
        Iterator1,
        fn (dep1: &str, dep2: i32) -> impl Iterator<Item=i32> {
            create(dep1, dep2)
        }
    }
    fn create(dep1: &str, dep2: i32) -> impl Iterator<Item = i32> {
        vec![1, 2, 3, dep1.len() as i32, dep2]
            .into_iter()
            .map(|x| x * 2)
    }
    assert_eq!(
        size_of::<Iterator1>(),
        size_of_val(&create(uninit(), uninit()))
    );

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let iter = Iterator1::new("hello", 100);
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
        async fn (dep1: &'static str, dep2: i32) -> impl Iterator<Item=i32> {
            create(dep1, dep2).await
        }
    }
    async fn create(dep1: &'static str, dep2: i32) -> impl Iterator<Item = i32> {
        vec![1, 2, 3, dep1.len() as i32, dep2]
            .into_iter()
            .map(|x| x * 2)
    }
    assert_eq!(
        size_of::<Iterator2>(),
        size_of_val(&create(uninit(), uninit()).await)
    );

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

const fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

fn uninit<T>() -> T {
    unsafe { MaybeUninit::uninit().assume_init() }
}
