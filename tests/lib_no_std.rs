#![no_std]

use core::mem::size_of;

#[test]
fn it_works_with_no_std() {
    stacklover::define_struct! {
        Iterator1,
        fn (dep1: &str, dep2: i32) -> impl Iterator<Item=i32> {
            create(dep1, dep2)
        },
        impls = (Send, Sync),
    }
    fn create(dep1: &str, dep2: i32) -> impl Iterator<Item = i32> {
        [1, 2, 3, dep1.len() as i32, dep2]
            .into_iter()
            .map(|x| x * 2)
    }
    assert_eq!(size_of::<Iterator1>(), size_of_val(&create("", 0)));
    let mut iter: Iterator1 = Iterator1::new("hello", 100);
    assert_eq!(iter.as_ref().size_hint(), (5, Some(5)));
    assert_eq!(iter.as_mut().next(), Some(2));
    let mut inner = iter.into_inner();
    assert_eq!(inner.next(), Some(4));
    assert_eq!(inner.next(), Some(6));
    assert_eq!(inner.next(), Some(10));
    assert_eq!(inner.next(), Some(200));
    assert_eq!(inner.next(), None);
}

const fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}
