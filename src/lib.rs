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

                pub fn as_ref(&self) -> &($return_type) {
                    self.__private.as_ref()
                }

                pub fn as_mut(&mut self) -> &mut ($return_type) {
                    self.__private.as_mut()
                }

                pub fn into_inner(self) -> $return_type {
                    self.__private.into_inner()
                }
            }

            struct [<__Stacklover $struct_name>] {
                inner: [u8; [<__Stacklover $struct_name>]::SIZE],
            }

            impl [<__Stacklover $struct_name>] {
                const SIZE: usize = {
                    #[allow(non_camel_case_types)]
                    const fn size_of_return_value<$([<P_ $param>]),*, R>(_: &(impl ::core::ops::Fn($([<P_ $param>]),*) -> R)) -> usize {
                        ::core::mem::size_of::<R>()
                    }
                    size_of_return_value(&Self::create)
                };

                #[inline(always)]
                fn create( $($param: $param_ty ),* ) -> $return_type {
                    $($body)*
                }

                fn create_unreachable() -> $return_type {
                    #[allow(unreachable_code)]
                    Self::create( $( $crate::__ident_to_unreachable!($param) ),* )
                }

                #[inline(always)]
                fn as_ref(&self) -> &($return_type) {
                    if true {
                        unsafe { ::core::mem::transmute(&self.inner) }
                    } else {
                        // _self for lifetime
                        fn ref_unreachable<S, T>(_self: &S, _: T) -> &T {
                            unreachable!()
                        }
                        #[allow(unreachable_code)]
                        ref_unreachable(self, Self::create_unreachable())
                    }
                }

                #[inline(always)]
                fn as_mut(&mut self) -> &mut ($return_type) {
                    if true {
                        unsafe { ::core::mem::transmute(&mut self.inner) }
                    } else {
                        // _self for lifetime
                        fn mut_unreachable<S, T>(_self: &S, _: T) -> &mut T {
                            unreachable!()
                        }
                        #[allow(unreachable_code)]
                        mut_unreachable(self, Self::create_unreachable())
                    }
                }

                #[inline(always)]
                fn into_inner(self) -> $return_type {
                    if true {
                        unsafe { ::core::mem::transmute(self.inner) }
                    } else {
                        fn assert_send_sync_unpin<T: Send + Sync + Unpin>(x: T) -> T {
                            x
                        }
                        #[allow(unreachable_code)]
                        assert_send_sync_unpin(Self::create_unreachable())
                    }
                }
            }
        }
        // TODO: impl Drop
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

                pub fn as_ref(&self) -> &($return_type) {
                    self.__private.as_ref()
                }

                pub fn as_mut(&mut self) -> &mut ($return_type) {
                    self.__private.as_mut()
                }

                pub fn into_inner(self) -> $return_type {
                    self.__private.into_inner()
                }
            }

            struct [<__Stacklover $struct_name>] {
                inner: [u8; [<__Stacklover $struct_name>]::SIZE],
            }

            impl [<__Stacklover $struct_name>] {
                const SIZE: usize = {
                    #[allow(non_camel_case_types)]
                    const fn size_of_async_return_value<$([<P_ $param>]),*, R, Fut: ::core::future::Future<Output = R>>(_: &(impl ::core::ops::Fn($([<P_ $param>]),*) -> Fut)) -> usize {
                        ::core::mem::size_of::<R>()
                    }
                    size_of_async_return_value(&Self::create)
                };

                #[inline(always)]
                $async fn create( $($param: $param_ty ),* ) -> $return_type {
                    $($body)*
                }

               fn create_unreachable() -> $return_type {
                    fn wrap_future<T: core::future::Future<Output = O>, O>(_: T) -> O {
                        unreachable!()
                    }
                    #[allow(unreachable_code)]
                    wrap_future(Self::create( $( $crate::__ident_to_unreachable!($param) ),* ))
               }

               #[inline(always)]
                fn as_ref(&self) -> &($return_type) {
                    if true {
                        unsafe { ::core::mem::transmute(&self.inner) }
                    } else {
                        // _self for lifetime
                        fn ref_unreachable<S, T>(_self: &S, _: T) -> &T {
                            unreachable!()
                        }
                        #[allow(unreachable_code)]
                        ref_unreachable(self, Self::create_unreachable())
                    }
                }

                #[inline(always)]
                fn as_mut(&mut self) -> &mut ($return_type) {
                    if true {
                        unsafe { ::core::mem::transmute(&mut self.inner) }
                    } else {
                        // _self for lifetime
                        fn mut_unreachable<S, T>(_self: &S, _: T) -> &mut T {
                            unreachable!()
                        }
                        #[allow(unreachable_code)]
                        mut_unreachable(self, Self::create_unreachable())
                    }
                }

               #[inline(always)]
               fn into_inner(self) -> $return_type {
                   if true {
                       unsafe { ::core::mem::transmute(self.inner) }
                   } else {
                       fn assert_send_sync_unpin<T: Send + Sync + Unpin>(x: T) -> T {
                           x
                       }
                       #[allow(unreachable_code)]
                       assert_send_sync_unpin(Self::create_unreachable())
                   }
               }
            }
        }
        // TODO: impl Drop
    };
}

#[macro_export]
macro_rules! __ident_to_unreachable {
    ( $x:ident ) => { unreachable!() };
}

pub mod private_mod {
    pub use paste::paste;
}