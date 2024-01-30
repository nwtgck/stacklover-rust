#[macro_export]
macro_rules! define_struct {
    // not async create
    (
        $struct_name:ident,
        $(#[$attrs:meta])*
        fn ( $( $param:ident: $param_ty:ty ),* ) -> $create_fn_return_type:ty { $($create_fn_body:tt)* },
        impls = ( $($derive_trait:ident),* $(,)? ) $(,)?
    ) => {
        $crate::define_struct!(
            $struct_name,
            $(#[$attrs])*
            fn ( $( $param: $param_ty ),* ) -> $create_fn_return_type { $($create_fn_body)* },
            impls = ( $($derive_trait),* ),
            inner_type = $create_fn_return_type,
            wrapped_type = __Inner__,
            to_wrapped_struct = |created_value, inner_to_struct| { inner_to_struct(created_value) },
        );
    };
    // not async create
    (
        $struct_name:ident,
        $(#[$attrs:meta])*
        fn ( $( $param:ident: $param_ty:ty ),* ) -> $create_fn_return_type:ty { $($create_fn_body:tt)* },
        impls = ( $($derive_trait:ident),* $(,)? ),
        inner_type = $inner_type:ty,
        // wrapped_type should include __Inner__
        wrapped_type = $wrapped_type:ty,
        to_wrapped_struct = |$created_value:ident, $inner_to_struct_fn:ident| { $($to_wrapped_struct_body:tt)* } $(,)?
    ) => {
        $crate::__define_struct!($struct_name);

        const _: () = {
            type __StackloverWrappedType<__Inner__> = $wrapped_type;

            // NOTE: prefix "__" is for avoiding name confliction. The function body should not use the function name because it will be accidentally a recursive function.
            #[inline(always)]
            $(#[$attrs])*
            fn __stacklover_create( $($param: $param_ty ),* ) -> $create_fn_return_type {
                $($create_fn_body)*
            }

            #[allow(unused)]
            #[allow(unreachable_code)]
            fn __stacklover_inner_unreachable() -> $inner_type {
                let __stacklover_inner_to_struct_fn_unreachable = |inner| -> $struct_name { ::core::unreachable!() };
                let _ = {
                    let $created_value = __stacklover_create( $( $crate::__ident_to_unreachable!($param) ),* );
                    let $inner_to_struct_fn = __stacklover_inner_to_struct_fn_unreachable;
                    // For type inference of __stacklover_inner_to_struct_fn_unreachable
                    $($to_wrapped_struct_body)*
                };
                fn __stacklover_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
                    ::core::unreachable!()
                }
                __stacklover_fn_param_unreachable(__stacklover_inner_to_struct_fn_unreachable)
            }

            const _: () = {
                fn _unused() {
                    fn assert_static<T: 'static>(_: T) {}
                    assert_static(__stacklover_inner_unreachable());
                }
            };

            impl $struct_name {
                #[doc(hidden)]
                const __SIZE: usize = {
                    #[allow(non_camel_case_types)]
                    const fn size_of_return_value<$($param,)* __StackloverR>(_: &(impl ::core::ops::Fn($($param),*) -> __StackloverWrappedType<__StackloverR>)) -> usize {
                        ::core::mem::size_of::<__StackloverR>()
                    }
                    size_of_return_value(&__stacklover_create)
                };

                #[inline(always)]
                pub fn new( $( $param: $param_ty ),* ) -> __StackloverWrappedType<Self> {
                    let __stacklover_inner_to_struct_fn = |inner| Self {
                        __private_inner: unsafe {
                            ::core::mem::transmute::<_, [::core::mem::MaybeUninit<u8>; Self::__SIZE]>(inner)
                        },
                        __phantom: ::core::marker::PhantomData,
                    };
                    {
                        let $created_value = __stacklover_create( $($param),* );
                        let $inner_to_struct_fn = __stacklover_inner_to_struct_fn;
                        $($to_wrapped_struct_body)*
                    }
                }

                #[inline(always)]
                pub fn as_ref(&self) -> &($inner_type) {
                    if true {
                        unsafe { ::core::mem::transmute::<&[::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(&self.__private_inner) }
                    } else {
                        // _self for lifetime
                        fn ref_unreachable<S, T>(_self: &S, _: T) -> &T {
                            ::core::unreachable!()
                        }
                        #[allow(unreachable_code)]
                        ref_unreachable(self, __stacklover_inner_unreachable())
                    }
                }

                #[inline(always)]
                pub fn as_mut(&mut self) -> &mut ($inner_type) {
                    if true {
                        unsafe { ::core::mem::transmute::<&mut[::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(&mut self.__private_inner) }
                    } else {
                        // _self for lifetime
                        fn mut_unreachable<S, T>(_self: &S, _: T) -> &mut T {
                            ::core::unreachable!()
                        }
                        #[allow(unreachable_code)]
                        mut_unreachable(self, __stacklover_inner_unreachable())
                    }
                }

                #[inline(always)]
                pub fn into_inner(self) -> $inner_type {
                    let inner = if true {
                        unsafe { ::core::mem::transmute::<[::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(self.__private_inner) }
                    } else {
                        #[allow(unreachable_code)]
                        __stacklover_inner_unreachable()
                    };
                    ::core::mem::forget(self);
                    inner
                }
            }

            impl ::core::ops::Drop for $struct_name {
                #[inline(always)]
                fn drop(&mut self) {
                    let _ = if true {
                        unsafe { ::core::mem::transmute::<[::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(self.__private_inner) }
                    } else {
                        #[allow(unreachable_code)]
                        __stacklover_inner_unreachable()
                    };
                }
            }

            $crate::__impl_traits!($struct_name, $($derive_trait)*);
        };
    };
    // async create
    (
        $struct_name:ident,
        $(#[$attrs:meta])*
        $async:ident fn ( $( $param:ident: $param_ty:ty ),* ) -> $create_fn_return_type:ty { $($create_fn_body:tt)* },
        impls = ( $($derive_trait:ident),* $(,)? ) $(,)?
    ) => {
        $crate::define_struct!(
            $struct_name,
            $(#[$attrs])*
            $async fn ( $( $param: $param_ty ),* ) -> $create_fn_return_type { $($create_fn_body)* },
            impls = ( $($derive_trait),* ),
            inner_type = $create_fn_return_type,
            wrapped_type = __Inner__,
            to_wrapped_struct = |created_value, inner_to_struct| { inner_to_struct(created_value) },
        );
    };
    // async create
    (
        $struct_name:ident,
        $(#[$attrs:meta])*
        $async:ident fn ( $( $param:ident: $param_ty:ty ),* ) -> $create_fn_return_type:ty { $($create_fn_body:tt)* },
        impls = ( $($derive_trait:ident),* $(,)? ),
        inner_type = $inner_type:ty,
        // wrapped_type should include __Inner__
        wrapped_type = $wrapped_type:ty,
        to_wrapped_struct = |$created_value:ident, $inner_to_struct_fn:ident| { $($to_wrapped_struct_body:tt)* } $(,)?
    ) => {
        $crate::__define_struct!($struct_name);

        const _: () = {
            type __StackloverWrappedType<__Inner__> = $wrapped_type;

            #[inline(always)]
            $(#[$attrs])*
            $async fn __stacklover_create( $($param: $param_ty ),* ) -> $create_fn_return_type {
                $($create_fn_body)*
            }

            #[allow(unused)]
            #[allow(unreachable_code)]
            fn __stacklover_inner_unreachable() -> $inner_type {
                fn __stacklover_await_future_unreachable<T: ::core::future::Future<Output = O>, O>(_: T) -> O {
                    ::core::unreachable!()
                }
                let __stacklover_inner_to_struct_fn_unreachable = |inner| -> $struct_name { ::core::unreachable!() };
                let _ = {
                    let $created_value = __stacklover_await_future_unreachable(__stacklover_create( $( $crate::__ident_to_unreachable!($param) ),* ));
                    let $inner_to_struct_fn = __stacklover_inner_to_struct_fn_unreachable;
                    // For type inference of __stacklover_inner_to_struct_fn_unreachable
                    $($to_wrapped_struct_body)*
                };
                fn __stacklover_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
                    ::core::unreachable!()
                }
                __stacklover_fn_param_unreachable(__stacklover_inner_to_struct_fn_unreachable)
            }

            const _: () = {
                fn _unused() {
                    fn assert_static<T: 'static>(_: T) {}
                    assert_static(__stacklover_inner_unreachable());
                }
            };

            impl $struct_name {
                #[doc(hidden)]
                const __SIZE: usize = {
                    #[allow(non_camel_case_types)]
                    const fn size_of_async_return_value<$($param,)* __StackloverR, Fut: ::core::future::Future<Output = __StackloverWrappedType<__StackloverR>>>(_: &(impl ::core::ops::Fn($($param),*) -> Fut)) -> usize {
                        ::core::mem::size_of::<__StackloverR>()
                    }
                    size_of_async_return_value(&__stacklover_create)
                };

                #[inline(always)]
                pub $async fn new( $($param: $param_ty ),* ) -> __StackloverWrappedType<Self> {
                    let __stacklover_inner_to_struct_fn = |inner| Self {
                        __private_inner: unsafe {
                            ::core::mem::transmute::<_, [::core::mem::MaybeUninit<u8>; Self::__SIZE]>(inner)
                        },
                        __phantom: ::core::marker::PhantomData,
                    };
                    {
                        let $created_value = __stacklover_create( $($param),* ).await;
                        let $inner_to_struct_fn = __stacklover_inner_to_struct_fn;
                        $($to_wrapped_struct_body)*
                    }
                }

                #[inline(always)]
                pub fn as_ref(&self) -> &($inner_type) {
                    if true {
                        unsafe { ::core::mem::transmute::<&[::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(&self.__private_inner) }
                    } else {
                        // _self for lifetime
                        fn ref_unreachable<S, T>(_self: &S, _: T) -> &T {
                            ::core::unreachable!()
                        }
                        #[allow(unreachable_code)]
                        ref_unreachable(self, __stacklover_inner_unreachable())
                    }
                }

                #[inline(always)]
                pub fn as_mut(&mut self) -> &mut ($inner_type) {
                    if true {
                        unsafe { ::core::mem::transmute::<&mut [::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(&mut self.__private_inner) }
                    } else {
                        // _self for lifetime
                        fn mut_unreachable<S, T>(_self: &S, _: T) -> &mut T {
                            ::core::unreachable!()
                        }
                        #[allow(unreachable_code)]
                        mut_unreachable(self, __stacklover_inner_unreachable())
                    }
                }

                #[inline(always)]
                pub fn into_inner(self) -> $inner_type {
                    let inner = if true {
                        unsafe { ::core::mem::transmute::<[::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(self.__private_inner) }
                    } else {
                        #[allow(unreachable_code)]
                        __stacklover_inner_unreachable()
                    };
                    ::core::mem::forget(self);
                    inner
                }
            }

            impl ::core::ops::Drop for $struct_name {
                #[inline(always)]
                fn drop(&mut self) {
                    let _ = if true {
                        unsafe { ::core::mem::transmute::<[::core::mem::MaybeUninit<u8>; Self::__SIZE], _>(self.__private_inner) }
                    } else {
                        #[allow(unreachable_code)]
                        __stacklover_inner_unreachable()
                    };
                }
            }

            $crate::__impl_traits!($struct_name, $($derive_trait)*);
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_struct {
    ( $struct_name:ident ) => {
        struct $struct_name {
            #[doc(hidden)]
            __private_inner: [::core::mem::MaybeUninit<u8>; $struct_name::__SIZE],
            #[doc(hidden)]
            __phantom: ::core::marker::PhantomData<(
                // for !Send + !Sync by default
                *const (),
                // for !Unpin by default
                ::core::marker::PhantomPinned,
                // for !UnwindSafe by default
                ::core::marker::PhantomData<&'static mut ()>,
                // for !Sync + !RefUnwindSafe by default
                ::core::marker::PhantomData<::core::cell::UnsafeCell<()>>,
            )>,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ident_to_unreachable {
    ( $x:ident ) => {
        ::core::unreachable!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_traits {
    ( $struct_name:ident, ) => { };
    // auto traits: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
    ( $struct_name:ident, Send $($xs:ident)* ) => {
        const _: () = {
            fn _unused() {
                fn assert_trait<T: ::core::marker::Send>(_: T) {}
                assert_trait(__stacklover_inner_unreachable());
            }
        };
        unsafe impl ::core::marker::Send for $struct_name {}
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Sync $($xs:ident)* ) => {
        const _: () = {
            fn _unused() {
                fn assert_trait<T: ::core::marker::Sync>(_: T) {}
                assert_trait(__stacklover_inner_unreachable());
            }
        };
        unsafe impl ::core::marker::Sync for $struct_name {}
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Unpin $($xs:ident)* ) => {
        const _: () = {
            fn _unused() {
                fn assert_trait<T: ::core::marker::Unpin>(_: T) {}
                assert_trait(__stacklover_inner_unreachable());
            }
        };
        impl ::core::marker::Unpin for $struct_name {}
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, UnwindSafe $($xs:ident)* ) => {
        const _: () = {
            fn _unused() {
                fn assert_trait<T: ::core::panic::UnwindSafe>(_: T) {}
                assert_trait(__stacklover_inner_unreachable());
            }
        };
        impl ::core::panic::UnwindSafe for $struct_name {}
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, RefUnwindSafe $($xs:ident)* ) => {
        const _: () = {
            fn _unused() {
                fn assert_trait<T: ::core::panic::RefUnwindSafe>(_: T) {}
                assert_trait(__stacklover_inner_unreachable());
            }
        };
        impl ::core::panic::RefUnwindSafe for $struct_name {}
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    // other traits
    ( $struct_name:ident, PartialEq $($xs:ident)* ) => {
        impl ::core::cmp::PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                ::core::cmp::PartialEq::eq($struct_name::as_ref(self), $struct_name::as_ref(other))
            }

            fn ne(&self, other: &Self) -> bool {
                ::core::cmp::PartialEq::ne($struct_name::as_ref(self), $struct_name::as_ref(other))
            }
        }
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Eq $($xs:ident)* ) => {
        impl ::core::cmp::Eq for $struct_name {}
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, PartialOrd $($xs:ident)* ) => {
        impl ::core::cmp::PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp($struct_name::as_ref(self), $struct_name::as_ref(other))
            }

            fn lt(&self, other: &Self) -> bool {
                ::core::cmp::PartialOrd::lt($struct_name::as_ref(self), $struct_name::as_ref(other))
            }

            fn le(&self, other: &Self) -> bool {
                ::core::cmp::PartialOrd::le($struct_name::as_ref(self), $struct_name::as_ref(other))
            }

            fn gt(&self, other: &Self) -> bool {
                ::core::cmp::PartialOrd::gt($struct_name::as_ref(self), $struct_name::as_ref(other))
            }

            fn ge(&self, other: &Self) -> bool {
                ::core::cmp::PartialOrd::ge($struct_name::as_ref(self), $struct_name::as_ref(other))
            }
        }
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Ord $($xs:ident)* ) => {
        impl ::core::cmp::Ord for $struct_name {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp($struct_name::as_ref(self), $struct_name::as_ref(other))
            }
        }
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Clone $($xs:ident)* ) => {
        impl ::core::clone::Clone for $struct_name {
            fn clone(&self) -> Self {
                let cloned = ::core::clone::Clone::clone($struct_name::as_ref(self));
                Self {
                    __private_inner: unsafe {
                        ::core::mem::transmute::<_, [::core::mem::MaybeUninit<u8>; Self::__SIZE]>(cloned)
                    },
                    __phantom: ::core::marker::PhantomData,
                }
            }
        }
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    // NOTE: `Copy`: the trait `Copy` cannot be implemented for this type; the type has a destructor
    ( $struct_name:ident, Hash $($xs:ident)* ) => {
        impl ::core::hash::Hash for $struct_name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash($struct_name::as_ref(self), state)
            }
        }
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Debug $($xs:ident)* ) => {
        impl ::core::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt($struct_name::as_ref(self), f)
            }
        }
        $crate::__impl_traits!($struct_name, $($xs)*);
    };
}
