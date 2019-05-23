use std::any::{Any, TypeId};

/// Condition type between two values.
///
/// ```rust
///   assert_eq!(types::ty_cond(&0, &1), true); // true
///   assert_ne!(types::ty_cond(&0, &String::default()), true); // true
/// ```
pub fn ty_cond<T: ?Sized + Any, R: ?Sized + Any>(_s: &T, _r: &R) -> bool {
    TypeId::of::<T>() == TypeId::of::<R>()
}

macro_rules! impl_tys {
    (
        $(($type: ty, $func: ident),)*
    ) => {
        /// Type condition Sets
        ///
        /// Note: prefer to use `ty_cond`, not this.
        pub trait Conds {
            $(fn $func(self) -> bool;)*
        }

        impl<T: Sized + Any> Conds for T {
            $(
                fn $func(self) -> bool {
                    TypeId::of::<$type>() == TypeId::of::<Self>()
                }
            )*
        }
    };
}

impl_tys! (
    // ints
    (i8, is_i8),
    (i16, is_i16),
    (i32, is_i32),
    (i64, is_i64),
    (i128, is_i128),

    // uints
    (u8, is_u8),
    (u16, is_u16),
    (u32, is_u32),
    (u64, is_u64),
    (u128, is_u128),

    // floats
    (f32, is_f32),
    (f64, is_f64),
);
