#[macro_export]
macro_rules! stacklover {
    // not async create
    ($struct_name:ident, $n:expr, ( $($param:ident: $param_ty:ty )* ) -> $return_type:ty { $($body:tt)* }) => {
        $crate::private_mod::paste! {
            struct $struct_name {
                __private: [<__Stacklover $struct_name>]
            }

            impl $struct_name {
                #[inline(always)]
                pub fn new( $($param: $param_ty )* ) -> Self {
                    Self {
                        __private: [<__Stacklover $struct_name>] {
                            inner: unsafe {
                                ::core::mem::transmute::<_, [u8; $n]>([<__Stacklover $struct_name>]::create( $($param)* ))
                            },
                        },
                    }
                }

                pub fn into_entity(self) -> $return_type {
                    self.__private.entity()
                }
            }

            struct [<__Stacklover $struct_name>] {
                inner: [u8; $n],
            }

            impl [<__Stacklover $struct_name>] {
                #[inline(always)]
                fn create( $($param: $param_ty )* ) -> $return_type {
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
                        // TODO: repeat unreachable!()
                        assert_send_sync_unpin(Self::create(unreachable!()))
                    }
                }
            }
        }
    };
    // async create
    ($struct_name:ident, $n:expr, $async:ident ( $($param:ident: $param_ty:ty )* ) -> $return_type:ty { $($body:tt)* }) => {
        $crate::private_mod::paste! {
            struct $struct_name {
                __private: [<__Stacklover $struct_name>]
            }

            impl $struct_name {
                #[inline(always)]
                pub $async fn new( $($param: $param_ty )* ) -> Self {
                    Self {
                        __private: [<__Stacklover $struct_name>] {
                            inner: unsafe {
                                ::core::mem::transmute::<_, [u8; $n]>([<__Stacklover $struct_name>]::create( $($param)* ).await)
                            },
                        },
                    }
                }

                pub fn into_entity(self) -> $return_type {
                    self.__private.entity()
                }
            }

            struct [<__Stacklover $struct_name>] {
                inner: [u8; $n],
            }

            impl [<__Stacklover $struct_name>] {
                #[inline(always)]
                $async fn create( $($param: $param_ty )* ) -> $return_type {
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
                        // TODO: repeat unreachable!()
                        assert_send_sync_unpin(wrap_future(Self::create(unreachable!())))
                    }
                }
            }
        }
    };
}

pub mod private_mod {
    pub use paste::paste;
}