#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use stacklover::stacklover;
struct Iterator1 {
    #[doc(hidden)]
    __private_inner: [u8; Iterator1::__SIZE],
}
const _: () = {
    #[inline(always)]
    fn __stacklover_create(dep1: &'static str, dep2: i32) -> impl Iterator<Item = i32> {
        (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 20)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
            .chain([dep1.len() as i32, dep2])
    }
    fn __stacklover_create_unreachable() -> impl Iterator<Item = i32> {
        #[allow(unreachable_code)]
        __stacklover_create(
            ::core::panicking::panic("internal error: entered unreachable code"),
            ::core::panicking::panic("internal error: entered unreachable code"),
        )
    }
    impl Iterator1 {
        #[doc(hidden)]
        const __SIZE: usize = {
            #[allow(non_camel_case_types)]
            const fn size_of_return_value<dep1, dep2, __StackloverR>(
                _: &(impl ::core::ops::Fn(dep1, dep2) -> __StackloverR),
            ) -> usize {
                ::core::mem::size_of::<__StackloverR>()
            }
            size_of_return_value(&__stacklover_create)
        };
        #[inline(always)]
        pub fn new(dep1: &'static str, dep2: i32) -> Self {
            Self {
                __private_inner: unsafe {
                    ::core::mem::transmute(__stacklover_create(dep1, dep2))
                },
            }
        }
        #[inline(always)]
        pub fn as_ref(&self) -> &(impl Iterator<Item = i32>) {
            if true {
                unsafe {
                    ::core::mem::transmute::<
                        &[u8; Self::__SIZE],
                        _,
                    >(&self.__private_inner)
                }
            } else {
                fn ref_unreachable<S, T>(_self: &S, _: T) -> &T {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
                #[allow(unreachable_code)]
                ref_unreachable(self, __stacklover_create_unreachable())
            }
        }
        #[inline(always)]
        pub fn as_mut(&mut self) -> &mut (impl Iterator<Item = i32>) {
            if true {
                unsafe {
                    ::core::mem::transmute::<
                        &mut [u8; Self::__SIZE],
                        _,
                    >(&mut self.__private_inner)
                }
            } else {
                fn mut_unreachable<S, T>(_self: &S, _: T) -> &mut T {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
                #[allow(unreachable_code)]
                mut_unreachable(self, __stacklover_create_unreachable())
            }
        }
        #[inline(always)]
        pub fn into_inner(self) -> impl Iterator<Item = i32> {
            let inner = if true {
                unsafe {
                    ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner)
                }
            } else {
                fn assert_traits<
                    T: ::core::marker::Send + ::core::marker::Sync
                        + ::core::marker::Unpin + ::core::panic::UnwindSafe
                        + ::core::panic::RefUnwindSafe,
                >(x: T) -> T {
                    x
                }
                #[allow(unreachable_code)]
                assert_traits(__stacklover_create_unreachable())
            };
            ::core::mem::forget(self);
            inner
        }
    }
    impl ::core::ops::Drop for Iterator1 {
        #[inline(always)]
        fn drop(&mut self) {
            let _ = if true {
                unsafe {
                    ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner)
                }
            } else {
                #[allow(unreachable_code)] __stacklover_create_unreachable()
            };
        }
    }
};
fn main() {}
