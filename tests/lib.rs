use futures::{SinkExt as _, StreamExt as _};
use stacklover::stacklover;
use std::mem::size_of;
use std::sync::{Arc, Mutex};

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
    assert_eq!(size_of::<Iterator1>(), size_of_val(&create("", 0)));

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let iter = Iterator1::new("hello", 100);
        tx.send(iter).unwrap();
    });

    let mut iter = rx.recv().unwrap();
    assert_eq!(iter.as_ref().size_hint(), (5, Some(5)));
    assert_eq!(iter.as_mut().next(), Some(2));
    assert_eq!(
        iter.into_inner().into_iter().collect::<Vec<i32>>(),
        vec![4, 6, 10, 200]
    );
}

#[tokio::test]
async fn it_works_without_dependency() {
    stacklover! {
        // struct name to be defined
        Iterator1,
        // empty parameters
        fn () -> impl Iterator<Item=i32> {
            vec![1, 2, 3]
            .into_iter()
            .map(|x| x * 2)
        }
    }

    stacklover! {
        // struct name to be defined
        Iterator2,
        // empty parameters
        async fn () -> impl Iterator<Item=i32> {
            vec![1, 2, 3]
            .into_iter()
            .map(|x| x * 2)
        }
    }

    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let iter = Iterator1::new();
            tx.send(iter).unwrap();
        });

        let iter = rx.recv().unwrap().into_inner();
        assert_eq!(iter.into_iter().collect::<Vec<i32>>(), vec![2, 4, 6]);
    };

    {
        let (mut tx, mut rx) = futures::channel::mpsc::channel(1);

        tokio::spawn(async move {
            let iter = Iterator2::new().await;
            tx.send(iter).await.unwrap();
        });

        let iter = rx.next().await.unwrap().into_inner();
        assert_eq!(iter.into_iter().collect::<Vec<i32>>(), vec![2, 4, 6]);
    }
}

#[test]
fn drops() {
    let dropped_count = Arc::new(Mutex::new(0));
    struct MyStruct {
        dropped: Arc<Mutex<u32>>,
    }
    impl Drop for MyStruct {
        fn drop(&mut self) {
            *self.dropped.lock().unwrap() += 1;
        }
    }

    stacklover! {
        MyStructStruct,
        fn (dropped: Arc<Mutex<u32>>) -> MyStruct {
            MyStruct{dropped}
        }
    }

    {
        let s = MyStructStruct::new(dropped_count.clone());
        assert_eq!(*dropped_count.lock().unwrap(), 0);
        let _ = s;
    }
    assert_eq!(*dropped_count.lock().unwrap(), 1);
    {
        let _ = MyStructStruct::new(dropped_count.clone());
        assert_eq!(*dropped_count.lock().unwrap(), 2);
    }
    assert_eq!(*dropped_count.lock().unwrap(), 2);
    {
        let s = MyStructStruct::new(dropped_count.clone());
        {
            assert_eq!(*dropped_count.lock().unwrap(), 2);
            let _inner = s.into_inner();
            assert_eq!(*dropped_count.lock().unwrap(), 2);
        }
        assert_eq!(*dropped_count.lock().unwrap(), 3);
    }
}

#[tokio::test]
async fn it_works_with_arc() {
    // Using Arc caused the error below in some implementation
    // error[E0080]: evaluation of constant value failed
    // using uninitialized data, but this operation requires initialized memory
    stacklover! {
        MyArc1,
        fn (dep1: &str) -> Arc<String> {
            Arc::new(dep1.to_owned())
        }
    }
    stacklover! {
        MyArc2,
        async fn (dep1: &str) -> Arc<String> {
            Arc::new(dep1.to_owned())
        }
    }
}

#[tokio::test]
async fn it_works_with_async() {
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
    assert_eq!(size_of::<Iterator2>(), size_of_val(&create("", 0).await));

    let (mut tx, mut rx) = futures::channel::mpsc::channel(1);

    tokio::spawn(async move {
        let iter = Iterator2::new("hello", 100).await;
        tx.send(iter).await.unwrap();
    });

    let mut iter = rx.next().await.unwrap();
    assert_eq!(iter.as_ref().size_hint(), (5, Some(5)));
    assert_eq!(iter.as_mut().next(), Some(2));
    assert_eq!(
        iter.into_inner().into_iter().collect::<Vec<i32>>(),
        vec![4, 6, 10, 200]
    );
}

const fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}
