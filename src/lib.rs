#[macro_export]
macro_rules! define_struct {
    // not async create
    (
        $struct_name:ident,
        $(#[$attrs:meta])*
        fn ( $( $param:ident: $param_ty:ty ),* ) -> $create_fn_return_type:ty { $($create_fn_body:tt)* } $(,)?
    ) => {
        $crate::define_struct!(
            $struct_name,
            $(#[$attrs])*
            fn ( $( $param: $param_ty ),* ) -> $create_fn_return_type { $($create_fn_body)* },
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
        inner_type = $inner_type:ty,
        // wrapped_type should include __Inner__
        wrapped_type = $wrapped_type:ty,
        to_wrapped_struct = |$created_value:ident, $inner_to_struct_fn:ident| { $($to_wrapped_struct_body:tt)* } $(,)?
    ) => {
        struct $struct_name {
            #[doc(hidden)]
            __private_inner: [u8; $struct_name::__SIZE]
        }

        const _: () = {
            type __StackloverWrappedType<__Inner__> = $wrapped_type;

            // NOTE: prefix "__" is for avoiding name confliction. The function body should not use the function name because it will be accidentally a recursive function.
            #[inline(always)]
            $(#[$attrs])*
            fn __stacklover_create( $($param: $param_ty ),* ) -> $create_fn_return_type {
                $($create_fn_body)*
            }

            #[allow(unreachable_code)]
            fn __stacklover_inner_unreachable() -> $inner_type {
                let __stacklover_created_value = __stacklover_create( $( $crate::__ident_to_unreachable!($param) ),* );
                let __stacklover_inner_to_struct_fn = |inner| $struct_name {
                    __private_inner: ::core::unreachable!(),
                };
                let _ = {
                    let $created_value = __stacklover_created_value;
                    let $inner_to_struct_fn = __stacklover_inner_to_struct_fn;
                    // For type inference of __stacklover_inner_to_struct_fn
                    $($to_wrapped_struct_body)*
                };
                fn __stacklover_inner_to_struct_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
                    unreachable!()
                }
                __stacklover_inner_to_struct_fn_param_unreachable(__stacklover_inner_to_struct_fn)
            }

            #[allow(unused)]
            fn __stacklover_assert_traits() {
                // auto traits: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
                // TODO: allow user to specify traits
                fn assert_traits<T: ::core::marker::Send + ::core::marker::Sync + ::core::marker::Unpin + ::core::panic::UnwindSafe + ::core::panic::RefUnwindSafe + 'static>(x: T) -> T {
                    x
                }
                assert_traits(__stacklover_inner_unreachable());
            }

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
                    let __stacklover_created_value = __stacklover_create( $($param),* );
                    let __stacklover_inner_to_struct_fn = |inner| Self {
                        __private_inner: unsafe {
                            ::core::mem::transmute::<_, [u8; Self::__SIZE]>(inner)
                        },
                    };
                    {
                        let $created_value = __stacklover_created_value;
                        let $inner_to_struct_fn = __stacklover_inner_to_struct_fn;
                        $($to_wrapped_struct_body)*
                    }
                }

                #[inline(always)]
                pub fn as_ref(&self) -> &($inner_type) {
                    if true {
                        unsafe { ::core::mem::transmute::<&[u8; Self::__SIZE], _>(&self.__private_inner) }
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
                        unsafe { ::core::mem::transmute::<&mut[u8; Self::__SIZE], _>(&mut self.__private_inner) }
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
                        unsafe { ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner) }
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
                        unsafe { ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner) }
                    } else {
                        #[allow(unreachable_code)]
                        __stacklover_inner_unreachable()
                    };
                }
            }
        };
    };
    // async create
    (
        $struct_name:ident,
        $(#[$attrs:meta])*
        $async:ident fn ( $( $param:ident: $param_ty:ty ),* ) -> $create_fn_return_type:ty { $($create_fn_body:tt)* } $(,)?
    ) => {
        $crate::define_struct!(
            $struct_name,
            $(#[$attrs])*
            $async fn ( $( $param: $param_ty ),* ) -> $create_fn_return_type { $($create_fn_body)* },
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
        inner_type = $inner_type:ty,
        // wrapped_type should include __Inner__
        wrapped_type = $wrapped_type:ty,
        to_wrapped_struct = |$created_value:ident, $inner_to_struct_fn:ident| { $($to_wrapped_struct_body:tt)* } $(,)?
    ) => {
        struct $struct_name {
            __private_inner: [u8; $struct_name::__SIZE]
        }

        const _: () = {
            type __StackloverWrappedType<__Inner__> = $wrapped_type;

            #[inline(always)]
            $(#[$attrs])*
            $async fn __stacklover_create( $($param: $param_ty ),* ) -> $create_fn_return_type {
                $($create_fn_body)*
            }

            #[allow(unreachable_code)]
            fn __stacklover_inner_unreachable() -> $inner_type {
                fn await_future_unreachable<T: core::future::Future<Output = O>, O>(_: T) -> O {
                    ::core::unreachable!()
                }
                let __stacklover_awaited_created_value = await_future_unreachable(__stacklover_create( $( $crate::__ident_to_unreachable!($param) ),* ));
                let __stacklover_inner_to_struct_fn = |inner| $struct_name {
                    __private_inner: ::core::unreachable!(),
                };
                let _ = {
                    let $created_value = __stacklover_awaited_created_value;
                    let $inner_to_struct_fn = __stacklover_inner_to_struct_fn;
                    // For type inference of __stacklover_inner_to_struct_fn
                    $($to_wrapped_struct_body)*
                };
                fn __stacklover_inner_to_struct_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
                    unreachable!()
                }
                __stacklover_inner_to_struct_fn_param_unreachable(__stacklover_inner_to_struct_fn)
            }

            #[allow(unused)]
            fn __stacklover_assert_traits() {
                // auto traits: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
                // TODO: allow user to specify traits
                fn assert_traits<T: ::core::marker::Send + ::core::marker::Sync + ::core::marker::Unpin + ::core::panic::UnwindSafe + ::core::panic::RefUnwindSafe + 'static>(x: T) -> T {
                    x
                }
                assert_traits(__stacklover_inner_unreachable());
            }

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
                    let __stacklover_awaited_created_value = __stacklover_create( $($param),* ).await;
                    let __stacklover_inner_to_struct_fn = |inner| Self {
                        __private_inner: unsafe {
                            ::core::mem::transmute::<_, [u8; Self::__SIZE]>(inner)
                        },
                    };
                    {
                        let $created_value = __stacklover_awaited_created_value;
                        let $inner_to_struct_fn = __stacklover_inner_to_struct_fn;
                        $($to_wrapped_struct_body)*
                    }
                }

                #[inline(always)]
                pub fn as_ref(&self) -> &($create_fn_return_type) {
                    if true {
                        unsafe { ::core::mem::transmute::<&[u8; Self::__SIZE], _>(&self.__private_inner) }
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
                pub fn as_mut(&mut self) -> &mut ($create_fn_return_type) {
                    if true {
                        unsafe { ::core::mem::transmute::<&mut [u8; Self::__SIZE], _>(&mut self.__private_inner) }
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
                pub fn into_inner(self) -> $create_fn_return_type {
                    let inner = if true {
                        unsafe { ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner) }
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
                        unsafe { ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner) }
                    } else {
                        #[allow(unreachable_code)]
                        __stacklover_inner_unreachable()
                    };
                }
            }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ident_to_unreachable {
    ( $x:ident ) => {
        ::core::unreachable!()
    };
}
