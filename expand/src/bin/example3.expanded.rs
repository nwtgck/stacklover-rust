#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use stacklover::define_struct;
#[repr(transparent)]
struct Iterator1 {
    #[doc(hidden)]
    __private_inner: ::stacklover::__private_mod::ErasedStorage<
        { Iterator1::__SIZE },
        { Iterator1::__ALIGN },
    >,
}
const _: () = {
    type __StackloverWrappedType<__Inner__> = Result<__Inner__, std::io::Error>;
    #[inline(always)]
    fn __stacklover_create(
        dep1: &'static str,
        dep2: i32,
    ) -> Result<impl Iterator<Item = i32> + Clone, std::io::Error> {
        let iter = (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 20)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
            .chain([dep1.len() as i32, dep2]);
        Ok(iter)
    }
    #[allow(unused)]
    #[allow(unreachable_code)]
    fn __stacklover_inner_unreachable() -> impl Iterator<Item = i32> + Clone {
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
    impl Iterator1 {
        #[inline(always)]
        pub fn new(dep1: &'static str, dep2: i32) -> __StackloverWrappedType<Self> {
            let __stacklover_inner_to_struct_fn = |inner| Self {
                __private_inner: unsafe {
                    ::core::mem::transmute::<
                        _,
                        ::stacklover::__private_mod::ErasedStorage<
                            { Iterator1::__SIZE },
                            { Iterator1::__ALIGN },
                        >,
                    >(inner)
                },
            };
            {
                let result = __stacklover_create(dep1, dep2);
                let inner_to_struct = __stacklover_inner_to_struct_fn;
                result.map(|inner| inner_to_struct(inner))
            }
        }
    }
    const _: () = {
        if !(::core::mem::size_of::<Iterator1>() == Iterator1::__SIZE) {
            {
                ::core::panicking::panic_fmt(format_args!("invalid size"));
            }
        }
        if !(::core::mem::align_of::<Iterator1>() == Iterator1::__ALIGN) {
            {
                ::core::panicking::panic_fmt(format_args!("invalid align"));
            }
        }
    };
    const _: fn() = || {
        fn assert_static<T: 'static>(_: T) {}
        assert_static(__stacklover_inner_unreachable());
    };
    impl Iterator1 {
        #[doc(hidden)]
        const __SIZE: usize = {
            const fn size_of_return_value<R>(
                _: &(impl ::core::ops::Fn() -> R),
            ) -> usize {
                ::core::mem::size_of::<R>()
            }
            size_of_return_value(&__stacklover_inner_unreachable)
        };
        #[doc(hidden)]
        const __ALIGN: usize = {
            const fn align_of_return_value<R>(
                _: &(impl ::core::ops::Fn() -> R),
            ) -> usize {
                ::core::mem::align_of::<R>()
            }
            align_of_return_value(&__stacklover_inner_unreachable)
        };
        #[inline(always)]
        pub fn as_ref(&self) -> &(impl Iterator<Item = i32> + Clone) {
            if true {
                unsafe {
                    ::core::mem::transmute::<
                        &::stacklover::__private_mod::ErasedStorage<
                            { Iterator1::__SIZE },
                            { Iterator1::__ALIGN },
                        >,
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
        pub fn as_mut(&mut self) -> &mut (impl Iterator<Item = i32> + Clone) {
            if true {
                unsafe {
                    ::core::mem::transmute::<
                        &mut ::stacklover::__private_mod::ErasedStorage<
                            { Iterator1::__SIZE },
                            { Iterator1::__ALIGN },
                        >,
                        _,
                    >(&mut self.__private_inner)
                }
            } else {
                fn mut_unreachable<S, T>(_self: &mut S, _: T) -> &mut T {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
                #[allow(unreachable_code)]
                mut_unreachable(self, __stacklover_inner_unreachable())
            }
        }
        #[inline(always)]
        pub fn into_inner(self) -> impl Iterator<Item = i32> + Clone {
            if true {
                unsafe { ::core::mem::transmute(self) }
            } else {
                #[allow(unreachable_code)] __stacklover_inner_unreachable()
            }
        }
        #[inline(always)]
        pub fn as_pin_mut(
            self: ::core::pin::Pin<&mut Self>,
        ) -> ::core::pin::Pin<&mut (impl Iterator<Item = i32> + Clone)> {
            unsafe { self.map_unchecked_mut(Self::as_mut) }
        }
    }
    impl ::core::ops::Drop for Iterator1 {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe { ::core::ptr::drop_in_place(self.as_mut()) }
        }
    }
    const _: fn() = || {
        fn assert_trait<T: ::core::marker::Send>(_: T) {}
        assert_trait(__stacklover_inner_unreachable());
    };
    unsafe impl ::core::marker::Send for Iterator1 {}
    const _: fn() = || {
        fn assert_trait<T: ::core::marker::Sync>(_: T) {}
        assert_trait(__stacklover_inner_unreachable());
    };
    unsafe impl ::core::marker::Sync for Iterator1 {}
    impl ::core::clone::Clone for Iterator1 {
        fn clone(&self) -> Self {
            let cloned = ::core::clone::Clone::clone(Iterator1::as_ref(self));
            Self {
                __private_inner: unsafe {
                    ::core::mem::transmute::<
                        _,
                        ::stacklover::__private_mod::ErasedStorage<
                            { Iterator1::__SIZE },
                            { Iterator1::__ALIGN },
                        >,
                    >(cloned)
                },
            }
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
