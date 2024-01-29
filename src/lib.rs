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
                    let __stacklover_inner_to_struct_fn = |inner| Self {
                        __private_inner: unsafe {
                            ::core::mem::transmute::<_, [u8; Self::__SIZE]>(inner)
                        },
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

            #[allow(unused)]
            #[allow(unreachable_code)]
            fn __stacklover_inner_unreachable() -> $inner_type {
                fn await_future_unreachable<T: ::core::future::Future<Output = O>, O>(_: T) -> O {
                    ::core::unreachable!()
                }
                let __stacklover_inner_to_struct_fn_unreachable = |inner| -> $struct_name { ::core::unreachable!() };
                let _ = {
                    let $created_value = await_future_unreachable(__stacklover_create( $( $crate::__ident_to_unreachable!($param) ),* ));
                    let $inner_to_struct_fn = __stacklover_inner_to_struct_fn_unreachable;
                    // For type inference of __stacklover_inner_to_struct_fn_unreachable
                    $($to_wrapped_struct_body)*
                };
                fn __stacklover_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
                    ::core::unreachable!()
                }
                __stacklover_fn_param_unreachable(__stacklover_inner_to_struct_fn_unreachable)
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
                    let __stacklover_inner_to_struct_fn = |inner| Self {
                        __private_inner: unsafe {
                            ::core::mem::transmute::<_, [u8; Self::__SIZE]>(inner)
                        },
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
}

#[macro_export]
macro_rules! wip_define_struct {
    // not async create
    (
        $struct_name:ident,
        $(#[$attrs:meta])*
        fn ( $( $param:ident: $param_ty:ty ),* ) -> $create_fn_return_type:ty { $($create_fn_body:tt)* },
        $( derive = ( $($derive_trait:ident),* $(,)? ), )?
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
                    let __stacklover_inner_to_struct_fn = |inner| Self {
                        __private_inner: unsafe {
                            ::core::mem::transmute::<_, [u8; Self::__SIZE]>(inner)
                        },
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

            $( $crate::__derive_traits!($struct_name, $($derive_trait)*); )?
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

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_traits {
    ( $struct_name:ident, ) => { };
    ( $struct_name:ident, PartialEq $($xs:ident)* ) => {
        impl ::core::cmp::PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.as_ref().eq(other.as_ref())
            }

            fn ne(&self, other: &Self) -> bool {
                self.as_ref().ne(other.as_ref())
            }
        }
        $crate::__derive_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Eq $($xs:ident)* ) => {
        impl ::core::cmp::Eq for $struct_name {}
        $crate::__derive_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, PartialOrd $($xs:ident)* ) => {
        impl ::core::cmp::PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                self.as_ref().partial_cmp(other.as_ref())
            }

            fn lt(&self, other: &Self) -> bool {
                self.as_ref().lt(other.as_ref())
            }

            fn le(&self, other: &Self) -> bool {
                self.as_ref().le(other.as_ref())
            }

            fn gt(&self, other: &Self) -> bool {
                self.as_ref().gt(other.as_ref())
            }

            fn ge(&self, other: &Self) -> bool {
                self.as_ref().ge(other.as_ref())
            }
        }
        $crate::__derive_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Clone $($xs:ident)* ) => {
        compile_error!("Deriving Clone not supported yet");
        // The following compile error ocurred to implement:
        // * "cannot transmute between types of different sizes, or dependently-sized types"
        // * "cannot transmute_copy if Dst is larger than Src"
        $crate::__derive_traits!($struct_name, $($xs)*);
    };
    // TODO: add Copy
    ( $struct_name:ident, Hash $($xs:ident)* ) => {
        impl ::core::hash::Hash for $struct_name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.as_ref().hash(state);
            }
        }
        $crate::__derive_traits!($struct_name, $($xs)*);
    };
    ( $struct_name:ident, Debug $($xs:ident)* ) => {
        impl ::core::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.as_ref().fmt(f)
            }
        }
        $crate::__derive_traits!($struct_name, $($xs)*);
    };
}
