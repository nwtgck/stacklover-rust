#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[repr(transparent)]
struct Iterator1 {
    #[doc(hidden)]
    __private_inner: ::stacklover::__private_mod::ErasedStorage<
        { Iterator1::__SIZE },
        { Iterator1::__ALIGN },
    >,
}
const _: () = {
    type __StackloverWrappedType<__Inner__> = __Inner__;
    #[inline(always)]
    async fn __stacklover_create(
        dep1: &'static str,
        dep2: i32,
    ) -> impl Iterator<Item = i32> + Clone {
        (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 20)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
            .chain([dep1.len() as i32, dep2])
    }
    #[allow(unused)]
    #[allow(unreachable_code)]
    fn __stacklover_inner_unreachable() -> impl Iterator<Item = i32> + Clone {
        fn __stacklover_await_future_unreachable<
            T: ::core::future::Future<Output = O>,
            O,
        >(_: T) -> O {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        let __stacklover_inner_to_struct_fn_unreachable = |inner| -> Iterator1 {
            ::core::panicking::panic("internal error: entered unreachable code")
        };
        let _ = {
            let created_value = __stacklover_await_future_unreachable(
                __stacklover_create(
                    ::core::panicking::panic("internal error: entered unreachable code"),
                    ::core::panicking::panic("internal error: entered unreachable code"),
                ),
            );
            let inner_to_struct = __stacklover_inner_to_struct_fn_unreachable;
            inner_to_struct(created_value)
        };
        fn __stacklover_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        __stacklover_fn_param_unreachable(__stacklover_inner_to_struct_fn_unreachable)
    }
    impl Iterator1 {
        #[inline(always)]
        pub async fn new(
            dep1: &'static str,
            dep2: i32,
        ) -> __StackloverWrappedType<Self> {
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
                let created_value = __stacklover_create(dep1, dep2).await;
                let inner_to_struct = __stacklover_inner_to_struct_fn;
                inner_to_struct(created_value)
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
    impl ::core::ops::Drop for Iterator1 {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe { ::core::ptr::drop_in_place(self.as_mut()) }
        }
    }
};
fn main() {}
