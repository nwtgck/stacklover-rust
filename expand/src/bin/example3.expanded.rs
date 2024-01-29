#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use stacklover::define_struct;
struct Iterator1 {
    #[doc(hidden)]
    __private_inner: [u8; Iterator1::__SIZE],
}
const _: () = {
    type __StackloverWrappedType<__Inner__> = Result<__Inner__, std::io::Error>;
    #[inline(always)]
    fn __stacklover_create(
        dep1: &'static str,
        dep2: i32,
    ) -> Result<impl Iterator<Item = i32>, std::io::Error> {
        let iter = (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 20)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
            .chain([dep1.len() as i32, dep2]);
        Ok(iter)
    }
    #[allow(unused)]
    #[allow(unreachable_code)]
    fn __stacklover_inner_unreachable() -> impl Iterator<Item = i32> {
        let __stacklover_inner_to_struct_fn_unreachable = |inner| -> Iterator1 {
            ::core::panicking::panic("internal error: entered unreachable code")
        };
        let _ = {
            let result = __stacklover_create(
                ::core::panicking::panic("internal error: entered unreachable code"),
                ::core::panicking::panic("internal error: entered unreachable code"),
            );
            let inner_to_struct = __stacklover_inner_to_struct_fn_unreachable;
            result.map(|inner| inner_to_struct(inner))
        };
        fn __stacklover_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        __stacklover_fn_param_unreachable(__stacklover_inner_to_struct_fn_unreachable)
    }
    #[allow(unused)]
    fn __stacklover_assert_traits() {
        fn assert_traits<
            T: ::core::marker::Send + ::core::marker::Sync + ::core::marker::Unpin
                + ::core::panic::UnwindSafe + ::core::panic::RefUnwindSafe + 'static,
        >(x: T) -> T {
            x
        }
        assert_traits(__stacklover_inner_unreachable());
    }
    impl Iterator1 {
        #[doc(hidden)]
        const __SIZE: usize = {
            #[allow(non_camel_case_types)]
            const fn size_of_return_value<dep1, dep2, __StackloverR>(
                _: &(impl ::core::ops::Fn(
                    dep1,
                    dep2,
                ) -> __StackloverWrappedType<__StackloverR>),
            ) -> usize {
                ::core::mem::size_of::<__StackloverR>()
            }
            size_of_return_value(&__stacklover_create)
        };
        #[inline(always)]
        pub fn new(dep1: &'static str, dep2: i32) -> __StackloverWrappedType<Self> {
            let __stacklover_inner_to_struct_fn = |inner| Self {
                __private_inner: unsafe {
                    ::core::mem::transmute::<_, [u8; Self::__SIZE]>(inner)
                },
            };
            {
                let result = __stacklover_create(dep1, dep2);
                let inner_to_struct = __stacklover_inner_to_struct_fn;
                result.map(|inner| inner_to_struct(inner))
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
                ref_unreachable(self, __stacklover_inner_unreachable())
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
                mut_unreachable(self, __stacklover_inner_unreachable())
            }
        }
        #[inline(always)]
        pub fn into_inner(self) -> impl Iterator<Item = i32> {
            let inner = if true {
                unsafe {
                    ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner)
                }
            } else {
                #[allow(unreachable_code)] __stacklover_inner_unreachable()
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
                #[allow(unreachable_code)] __stacklover_inner_unreachable()
            };
        }
    }
};
fn main() {
    let result: Result<Iterator1, std::io::Error> = Iterator1::new("hello", 10);
    for i in result.unwrap().into_inner() {
        {
            ::std::io::_print(format_args!("i={0}\n", i));
        };
    }
}
