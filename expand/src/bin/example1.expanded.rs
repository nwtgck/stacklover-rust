#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use stacklover::stacklover;
struct Iterator1 {
    __private: __StackloverIterator1,
}
impl Iterator1 {
    #[inline(always)]
    pub fn new(dep1: &'static str, dep2: i32) -> Self {
        Self {
            __private: __StackloverIterator1 {
                inner: unsafe {
                    ::core::mem::transmute(__StackloverIterator1::create(dep1, dep2))
                },
            },
        }
    }
    #[inline(always)]
    pub fn as_ref(&self) -> &(impl Iterator<Item = i32>) {
        self.__private.as_ref()
    }
    #[inline(always)]
    pub fn as_mut(&mut self) -> &mut (impl Iterator<Item = i32>) {
        self.__private.as_mut()
    }
    #[inline(always)]
    pub fn into_inner(self) -> impl Iterator<Item = i32> {
        self.__private.into_inner()
    }
}
struct __StackloverIterator1 {
    inner: [u8; __StackloverIterator1::SIZE],
}
impl __StackloverIterator1 {
    const SIZE: usize = {
        #[allow(non_camel_case_types)]
        const fn size_of_return_value<P_dep1, P_dep2, R>(
            _: &(impl ::core::ops::Fn(P_dep1, P_dep2) -> R),
        ) -> usize {
            ::core::mem::size_of::<R>()
        }
        size_of_return_value(&Self::create)
    };
    #[inline(always)]
    fn create(dep1: &'static str, dep2: i32) -> impl Iterator<Item = i32> {
        (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 20)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
            .chain([dep1.len() as i32, dep2])
    }
    fn create_unreachable() -> impl Iterator<Item = i32> {
        #[allow(unreachable_code)]
        Self::create(
            ::core::panicking::panic("internal error: entered unreachable code"),
            ::core::panicking::panic("internal error: entered unreachable code"),
        )
    }
    #[inline(always)]
    fn as_ref(&self) -> &(impl Iterator<Item = i32>) {
        if true {
            unsafe { ::core::mem::transmute(&self.inner) }
        } else {
            fn ref_unreachable<S, T>(_self: &S, _: T) -> &T {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            #[allow(unreachable_code)] ref_unreachable(self, Self::create_unreachable())
        }
    }
    #[inline(always)]
    fn as_mut(&mut self) -> &mut (impl Iterator<Item = i32>) {
        if true {
            unsafe { ::core::mem::transmute(&mut self.inner) }
        } else {
            fn mut_unreachable<S, T>(_self: &S, _: T) -> &mut T {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            #[allow(unreachable_code)] mut_unreachable(self, Self::create_unreachable())
        }
    }
    #[inline(always)]
    fn into_inner(self) -> impl Iterator<Item = i32> {
        let inner = if true {
            unsafe { ::core::mem::transmute(self.inner) }
        } else {
            fn assert_traits<
                T: ::core::marker::Send + ::core::marker::Sync + ::core::marker::Unpin
                    + ::core::panic::UnwindSafe + ::core::panic::RefUnwindSafe,
            >(x: T) -> T {
                x
            }
            #[allow(unreachable_code)] assert_traits(Self::create_unreachable())
        };
        ::core::mem::forget(self);
        inner
    }
}
impl ::core::ops::Drop for __StackloverIterator1 {
    fn drop(&mut self) {
        let _ = if true {
            unsafe { ::core::mem::transmute(self.inner) }
        } else {
            #[allow(unreachable_code)] Self::create_unreachable()
        };
    }
}
fn main() {}
