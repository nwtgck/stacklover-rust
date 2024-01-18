#[macro_export]
macro_rules! stacklover {
    ($struct_name:ident, $n:expr, ( $($param:ident: $param_ty:ty )* ) -> $return_type:ty { $($body:tt)* }) => {
        struct $struct_name {
            __private: __PrivateTodo
        }

        impl $struct_name {
            #[inline(always)]
            pub fn new( $($param: $param_ty )* ) -> Self {
                Self {
                    __private: __PrivateTodo {
                        inner: unsafe {
                            ::core::mem::transmute::<_, [u8; $n]>(__PrivateTodo::create( $($param)* ))
                        },
                    },
                }
            }

            // TODO: return type
            pub fn into_entity(self) -> $return_type {
                self.__private.entity()
            }
        }

        // TODO: name with $struct_name
        struct __PrivateTodo {
            inner: [u8; $n],
        }

        impl __PrivateTodo {
            #[inline(always)]
            fn create( $($param: $param_ty )* ) -> $return_type {
                $($body)*
            }

            #[inline(always)]
            fn entity(&self) -> $return_type {
                if true {
                    unsafe { ::core::mem::transmute(self.inner) }
                } else {
                    #[inline(always)]
                    fn assert_send_sync_unpin<T: Send + Sync + Unpin>(x: T) -> T {
                        x
                    }
                    #[allow(unreachable_code)]
                    assert_send_sync_unpin(Self::create(unreachable!()))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! private_struct_name {
    ($name:ident) => {
        paste::paste! {
            [<__PrivateStacklover $name>]
        }
    }
}
