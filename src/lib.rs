#[macro_export]
macro_rules! stacklover {
    // not async create
    ($struct_name:ident, fn ( $( $param:ident: $param_ty:ty ),* ) -> $return_type:ty { $($body:tt)* }) => {
        $crate::private_mod::paste! {
            struct $struct_name {
                __private: [<__Stacklover $struct_name>]
            }

            impl $struct_name {
                #[inline(always)]
                pub fn new( $($param: $param_ty ),* ) -> Self {
                    Self {
                        __private: [<__Stacklover $struct_name>] {
                            inner: unsafe {
                                ::core::mem::transmute([<__Stacklover $struct_name>]::create( $($param),* ))
                            },
                        },
                    }
                }

                pub fn into_entity(self) -> $return_type {
                    self.__private.entity()
                }
            }

            struct [<__Stacklover $struct_name>] {
                inner: [u8; [<__Stacklover $struct_name>]::SIZE],
            }

            impl [<__Stacklover $struct_name>] {
                const SIZE: usize = {
                    #[allow(non_camel_case_types)]
                    const fn return_value<$([<P_ $param>]),*, R>(_: &(impl ::core::ops::Fn($([<P_ $param>]),*) -> R)) -> R {
                        unsafe { ::core::mem::MaybeUninit::uninit().assume_init() }
                    }
                    const fn size_of_val<T>(_: &T) -> usize {
                        ::core::mem::size_of::<T>()
                    }
                    size_of_val(&::core::mem::ManuallyDrop::new(return_value(&Self::create)))
                };

                #[inline(always)]
                fn create( $($param: $param_ty ),* ) -> $return_type {
                    $($body)*
                }

                #[inline(always)]
                fn entity(&self) -> $return_type {
                    if true {
                        unsafe { ::core::mem::transmute(self.inner) }
                    } else {
                        fn assert_send_sync_unpin<T: Send + Sync + Unpin>(x: T) -> T {
                            x
                        }
                        #[allow(unreachable_code)]
                        assert_send_sync_unpin(Self::create( $( $crate::__ident_to_unreachable!($param) ),* ))
                    }
                }
            }
        }
    };
    // async create
    ($struct_name:ident, $async:ident fn ( $( $param:ident: $param_ty:ty ),* ) -> $return_type:ty { $($body:tt)* }) => {
        $crate::private_mod::paste! {
            struct $struct_name {
                __private: [<__Stacklover $struct_name>]
            }

            impl $struct_name {
                #[inline(always)]
                pub $async fn new( $($param: $param_ty ),* ) -> Self {
                    Self {
                        __private: [<__Stacklover $struct_name>] {
                            inner: unsafe {
                                ::core::mem::transmute([<__Stacklover $struct_name>]::create( $($param),* ).await)
                            },
                        },
                    }
                }

                pub fn into_entity(self) -> $return_type {
                    self.__private.entity()
                }
            }

            struct [<__Stacklover $struct_name>] {
                inner: [u8; [<__Stacklover $struct_name>]::SIZE],
            }

            impl [<__Stacklover $struct_name>] {
                const SIZE: usize = {
                    #[allow(non_camel_case_types)]
                    const fn async_return_value<$([<P_ $param>]),*, R, Fut: ::core::future::Future<Output = R>>(_: &(impl ::core::ops::Fn($([<P_ $param>]),*) -> Fut)) -> R {
                        unsafe { ::core::mem::MaybeUninit::uninit().assume_init() }
                    }
                    const fn size_of_val<T>(_: &T) -> usize {
                        ::core::mem::size_of::<T>()
                    }
                    size_of_val(&::core::mem::ManuallyDrop::new(async_return_value(&Self::create)))
                };

                #[inline(always)]
                $async fn create( $($param: $param_ty ),* ) -> $return_type {
                    $($body)*
                }

                #[inline(always)]
                fn entity(&self) -> $return_type {
                    if true {
                        unsafe { ::core::mem::transmute(self.inner) }
                    } else {
                        fn assert_send_sync_unpin<T: Send + Sync + Unpin>(x: T) -> T {
                            x
                        }
                        fn wrap_future<T: core::future::Future<Output = O>, O>(_: T) -> O {
                            unreachable!()
                        }
                        #[allow(unreachable_code)]
                        assert_send_sync_unpin(wrap_future(Self::create( $( $crate::__ident_to_unreachable!($param) ),* )))
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __ident_to_unreachable {
    ( $x:ident ) => { unreachable!() };
}

pub mod private_mod {
    pub use paste::paste;
}