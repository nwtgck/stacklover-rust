use futures::{SinkExt as _, StreamExt as _};
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::mem::{align_of, align_of_val, size_of};
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

#[test]
fn it_works() {
    stacklover::define_struct! {
        // struct name to be defined
        Iterator1,
        fn (dep1: &str, dep2: i32) -> impl Iterator<Item=i32> {
            create(dep1, dep2)
        },
        impls = (Send, Sync),
    }
    fn create(dep1: &str, dep2: i32) -> impl Iterator<Item = i32> {
        vec![1, 2, 3, dep1.len() as i32, dep2]
            .into_iter()
            .map(|x| x * 2)
    }
    assert_eq!(size_of::<Iterator1>(), size_of_val(&create("", 0)));
    assert_eq!(align_of::<Iterator1>(), align_of_val(&create("", 0)));

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let iter = Iterator1::new("hello", 100);
        tx.send(iter).unwrap();
    });

    let mut iter = rx.recv().unwrap();
    assert_eq!(iter.as_ref().size_hint(), (5, Some(5)));
    assert_eq!(iter.as_mut().next(), Some(2));
    assert_eq!(
        iter.into_inner().into_iter().collect::<Vec<_>>(),
        vec![4, 6, 10, 200]
    );
}

#[test]
fn it_works_with_deriving() {
    stacklover::define_struct! {
        Tuple1,
        fn (dep1: &str, dep2: i32) -> (String, i32, bool) {
            create(dep1, dep2)
        },
        impls = ( PartialEq, Eq, Clone, Debug ),
    }
    fn create(dep1: &str, dep2: i32) -> (String, i32, bool) {
        (dep1.to_owned(), dep2, false)
    }

    let x: Tuple1 = Tuple1::new("hello", 100);
    let bare = create("hello", 100);
    assert_eq!(format!("{:?}", x), format!("{:?}", bare));
    assert_eq!(x, x);
    assert_eq!(x.clone().into_inner(), x.into_inner());
}

#[test]
fn it_works_with_deriving_all() {
    stacklover::define_struct! {
        I32,
        fn (dep2: i32) -> i32 {
            dep2
        },
        impls = ( Send, Sync, Unpin, UnwindSafe, RefUnwindSafe, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug, ),
    }
    fn assert_traits<
        T: Send
            + Sync
            + Unpin
            + UnwindSafe
            + RefUnwindSafe
            + PartialEq
            + Eq
            + PartialOrd
            + Ord
            + Clone
            + Hash
            + Debug,
    >(
        _: &T,
    ) {
    }

    let x: I32 = I32::new(100);
    let bare = 100;
    assert_traits(&x);
    assert_eq!(format!("{:?}", x), format!("{:?}", bare));
    assert_eq!(x, x);
    assert_eq!(x.clone().into_inner(), x.into_inner());
}

#[test]
fn it_works_with_wrap_params() {
    stacklover::define_struct! {
        Iterator1,
        fn (dep1: &str, dep2: i32) -> Result<impl Iterator<Item=i32>, std::io::Error> {
            let iter = create(dep1, dep2);
            Ok(iter)
        },
        impls = (Send),
        inner_type = impl Iterator<Item=i32>,
        wrapped_type = Result<__Inner__, std::io::Error>,
        to_wrapped_struct = |result, inner_to_struct| { result.map(inner_to_struct) },
    }
    fn create(dep1: &str, dep2: i32) -> impl Iterator<Item = i32> {
        vec![1, 2, 3, dep1.len() as i32, dep2]
            .into_iter()
            .map(|x| x * 2)
    }
    assert_eq!(size_of::<Iterator1>(), size_of_val(&create("", 0)));
    assert_eq!(align_of::<Iterator1>(), align_of_val(&create("", 0)));

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let result: Result<Iterator1, std::io::Error> = Iterator1::new("hello", 100);
        if let Ok(iter) = result {
            tx.send(iter).unwrap();
        } else {
            assert!(false);
        }
    });

    let mut iter = rx.recv().unwrap();
    assert_eq!(iter.as_ref().size_hint(), (5, Some(5)));
    assert_eq!(iter.as_mut().next(), Some(2));
    assert_eq!(
        iter.into_inner().into_iter().collect::<Vec<_>>(),
        vec![4, 6, 10, 200]
    );
}

#[test]
fn it_works_with_deriving_traits_and_wrap_params() {
    stacklover::define_struct! {
        Tuple1,
        fn (dep1: &str, dep2: i32) -> Result<impl PartialEq + Eq + Debug, std::io::Error> {
            Ok(create(dep1, dep2))
        },
        impls = ( PartialEq, Eq, Debug ),
        inner_type = impl PartialEq + Eq + Debug,
        wrapped_type = Result<__Inner__, std::io::Error>,
        to_wrapped_struct = |result, inner_to_struct| { result.map(inner_to_struct) },
    }
    fn create(dep1: &str, dep2: i32) -> impl PartialEq + Eq + Debug {
        (dep1.to_owned(), dep2, false)
    }

    let result: Result<Tuple1, std::io::Error> = Tuple1::new("hello", 100);
    let x: Tuple1 = result.unwrap();
    let bare = create("hello", 100);
    assert_eq!(format!("{:?}", x), format!("{:?}", bare));
    assert_eq!(x, x);
}

#[test]
fn it_works_with_fn() {
    stacklover::define_struct! {
        MyFn,
        fn (s: String) -> impl Fn(i32) -> i32 {
            move |i: i32| { i + s.len() as i32 }
        },
        impls = (Send),
    }
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let fn1 = MyFn::new("hello".to_owned());
        tx.send(fn1).unwrap();
    });

    let fn1 = rx.recv().unwrap().into_inner();
    assert_eq!(fn1(10), 15);
    assert_eq!(fn1(100), 105);
}

#[tokio::test]
async fn it_works_without_dependency() {
    stacklover::define_struct! {
        // struct name to be defined
        Iterator1,
        // empty parameters
        fn () -> impl Iterator<Item=i32> {
            vec![1, 2, 3]
            .into_iter()
            .map(|x| x * 2)
        },
        impls = (Send, Sync),
    }

    stacklover::define_struct! {
        // struct name to be defined
        Iterator2,
        // empty parameters
        async fn () -> impl Iterator<Item=i32> {
            vec![1, 2, 3]
            .into_iter()
            .map(|x| x * 2)
        },
        impls = (Send, Sync),
    }

    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let iter = Iterator1::new();
            tx.send(iter).unwrap();
        });

        let iter = rx.recv().unwrap().into_inner();
        assert_eq!(iter.into_iter().collect::<Vec<_>>(), vec![2, 4, 6]);
    };

    {
        let (mut tx, mut rx) = futures::channel::mpsc::channel(1);

        tokio::spawn(async move {
            let iter = Iterator2::new().await;
            tx.send(iter).await.unwrap();
        });

        let iter = rx.next().await.unwrap().into_inner();
        assert_eq!(iter.into_iter().collect::<Vec<_>>(), vec![2, 4, 6]);
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

    stacklover::define_struct! {
        MyStructStruct,
        fn (dropped: Arc<Mutex<u32>>) -> MyStruct {
            MyStruct{dropped}
        },
        impls = (),
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
    stacklover::define_struct! {
        MyArc1,
        fn (dep1: &str) -> Arc<String> {
            Arc::new(dep1.to_owned())
        },
        impls = (),
    }
    stacklover::define_struct! {
        MyArc2,
        async fn (dep1: &str) -> Arc<String> {
            Arc::new(dep1.to_owned())
        },
        impls = (),
    }
}

#[test]
fn it_works_with_auto_enum_attribute() {
    stacklover::define_struct! {
        AutoEnumIterator,
        #[auto_enums::auto_enum(Iterator)]
        fn (x: i32) -> impl Iterator<Item=i32> {
            match x {
                0 => 1..10,
                _ => vec![5, 10].into_iter(),
            }
        },
        impls = (Send, Sync),
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let iter1 = AutoEnumIterator::new(0);
        tx.send(iter1).unwrap();
        let iter2 = AutoEnumIterator::new(1);
        tx.send(iter2).unwrap();
    });

    let iter1 = rx.recv().unwrap().into_inner();
    assert_eq!(
        iter1.into_iter().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    let iter2 = rx.recv().unwrap().into_inner();
    assert_eq!(iter2.into_iter().collect::<Vec<_>>(), vec![5, 10]);
}

#[tokio::test]
async fn it_works_with_async() {
    stacklover::define_struct! {
        Iterator2,
        async fn (dep1: &'static str, dep2: i32) -> impl Iterator<Item=i32> {
            tokio::time::sleep(tokio::time::Duration::from_nanos(0)).await;
            create(dep1, dep2).await
        },
        impls = (Send),
    }
    async fn create(dep1: &'static str, dep2: i32) -> impl Iterator<Item = i32> {
        vec![1, 2, 3, dep1.len() as i32, dep2]
            .into_iter()
            .map(|x| x * 2)
    }
    assert_eq!(size_of::<Iterator2>(), size_of_val(&create("", 0).await));
    assert_eq!(align_of::<Iterator2>(), align_of_val(&create("", 0).await));

    let (mut tx, mut rx) = futures::channel::mpsc::channel(1);

    tokio::spawn(async move {
        let iter = Iterator2::new("hello", 100).await;
        tx.send(iter).await.unwrap();
    });

    let mut iter = rx.next().await.unwrap();
    assert_eq!(iter.as_ref().size_hint(), (5, Some(5)));
    assert_eq!(iter.as_mut().next(), Some(2));
    assert_eq!(
        iter.into_inner().into_iter().collect::<Vec<_>>(),
        vec![4, 6, 10, 200]
    );
}

#[tokio::test]
async fn it_works_with_async_and_deriving() {
    stacklover::define_struct! {
        Tuple1,
        async fn (dep1: &str, dep2: i32) -> impl PartialEq + Eq + Debug {
            create(dep1, dep2)
        },
        impls = ( PartialEq, Eq, Debug ),
    }
    fn create(dep1: &str, dep2: i32) -> impl PartialEq + Eq + Debug {
        (dep1.to_owned(), dep2, false)
    }

    let x: Tuple1 = Tuple1::new("hello", 100).await;
    let bare = create("hello", 100);
    assert_eq!(format!("{:?}", x), format!("{:?}", bare));
    assert_eq!(x, x);
}

#[tokio::test]
async fn it_works_with_async_fn_and_wrap_params() {
    stacklover::define_struct! {
        Iterator1,
        async fn (dep1: &'static str, dep2: i32) -> Result<impl Iterator<Item=i32>, std::io::Error> {
            tokio::time::sleep(tokio::time::Duration::from_nanos(0)).await;
            let iter = create(dep1, dep2).await;
            Ok(iter)
        },
        impls = (Send),
        inner_type = impl Iterator<Item=i32>,
        wrapped_type = Result<__Inner__, std::io::Error>,
        to_wrapped_struct = |result, inner_to_struct| { result.map(inner_to_struct) },
    }
    async fn create(dep1: &'static str, dep2: i32) -> impl Iterator<Item = i32> {
        vec![1, 2, 3, dep1.len() as i32, dep2]
            .into_iter()
            .map(|x| x * 2)
    }
    assert_eq!(size_of::<Iterator1>(), size_of_val(&create("", 0).await));
    assert_eq!(align_of::<Iterator1>(), align_of_val(&create("", 0).await));

    let (mut tx, mut rx) = futures::channel::mpsc::channel(1);

    tokio::spawn(async move {
        let result: Result<Iterator1, std::io::Error> = Iterator1::new("hello", 100).await;
        if let Ok(iter) = result {
            tx.send(iter).await.unwrap();
        } else {
            assert!(false);
        }
    });

    let mut iter = rx.next().await.unwrap();
    assert_eq!(iter.as_ref().size_hint(), (5, Some(5)));
    assert_eq!(iter.as_mut().next(), Some(2));
    assert_eq!(
        iter.into_inner().into_iter().collect::<Vec<_>>(),
        vec![4, 6, 10, 200]
    );
}

#[tokio::test]
async fn it_works_with_async_and_deriving_traits_and_wrap_params() {
    stacklover::define_struct! {
        Tuple1,
        async fn (dep1: &str, dep2: i32) -> Result<impl PartialEq + Eq + Debug, std::io::Error> {
            Ok(create(dep1, dep2))
        },
        impls = ( PartialEq, Eq, Debug ),
        inner_type = impl PartialEq + Eq + Debug,
        wrapped_type = Result<__Inner__, std::io::Error>,
        to_wrapped_struct = |result, inner_to_struct| { result.map(inner_to_struct) },
    }
    fn create(dep1: &str, dep2: i32) -> impl PartialEq + Eq + Debug {
        (dep1.to_owned(), dep2, false)
    }

    let result: Result<Tuple1, std::io::Error> = Tuple1::new("hello", 100).await;
    let x: Tuple1 = result.unwrap();
    let bare = create("hello", 100);
    assert_eq!(format!("{:?}", x), format!("{:?}", bare));
    assert_eq!(x, x);
}

#[tokio::test]
async fn it_works_with_as_pin_mut() {
    stacklover::define_struct! {
        Future1,
        #[allow(clippy::manual_async_fn)]
        fn () -> impl Future<Output=i32> {
            async {
                tokio::time::sleep(tokio::time::Duration::from_nanos(1)).await;
                10
            }
        },
        impls = (), // NOTE: !Unpin
    }

    impl Future for Future1 {
        type Output = i32;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.as_pin_mut().poll(cx)
        }
    }

    assert_eq!(Future1::new().await, 10);
}

#[test]
fn it_works_with_as_pin_drop() {
    struct Trap {
        data: String,
        ptr: *const String,
        _marker: std::marker::PhantomPinned,
    }
    impl Drop for Trap {
        fn drop(&mut self) {
            let this = unsafe { Pin::new_unchecked(self) };
            if !this.ptr.is_null() {
                // If ptr is initialized, we know that self has been pinned and hasn't moved. Assert that this is still the case.
                assert_eq!(this.ptr, &this.data, "ptr should point to our own data");
                // Simulate a read if the ptr was initialized to trap miri if the pointer provenance was somehow disabled.
                let _ = unsafe { &*this.ptr }.to_string();
            }
        }
    }
    trait Init {
        fn init(self: Pin<&mut Self>);
    }
    impl Init for Trap {
        fn init(self: Pin<&mut Self>) {
            let self_ptr = &self.data as *const String;
            let this = unsafe { self.get_unchecked_mut() };
            this.ptr = self_ptr;
        }
    }
    fn create() -> Trap {
        Trap {
            data: "foobar".to_string(),
            ptr: core::ptr::null(),
            _marker: std::marker::PhantomPinned,
        }
    }
    stacklover::define_struct! {
        PinUpType,
        fn () -> impl Init {
            create()
        },
        impls = (), // NOTE: !Unpin
    }
    let mut pinned = PinUpType::new();
    let pinned = ::core::pin::pin!(pinned);
    pinned.as_pin_mut().init();
    // ... pinned is dropped here, which activates the trap in miri.
}

#[tokio::test]
async fn it_works_with_async_auto_enum_attribute() {
    stacklover::define_struct! {
        AutoEnumIterator,
        #[auto_enums::auto_enum(Iterator)]
        async fn (x: i32) -> impl Iterator<Item=i32> {
            match x {
                0 => 1..10,
                _ => vec![5, 10].into_iter(),
            }
        },
        impls = (Send),
    }

    let (mut tx, mut rx) = futures::channel::mpsc::channel(1);
    tokio::spawn(async move {
        let iter1 = AutoEnumIterator::new(0).await;
        tx.send(iter1).await.unwrap();
        let iter2 = AutoEnumIterator::new(1).await;
        tx.send(iter2).await.unwrap();
    });

    let iter1 = rx.next().await.unwrap().into_inner();
    assert_eq!(
        iter1.into_iter().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    let iter2 = rx.next().await.unwrap().into_inner();
    assert_eq!(iter2.into_iter().collect::<Vec<_>>(), vec![5, 10]);
}

const fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}
