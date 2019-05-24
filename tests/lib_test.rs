use types::Conds;

#[test]
fn test_ty() {
    assert_eq!(types::ty_cond(&0, &1), true);
    assert_ne!(types::ty_cond(&0, &String::default()), true);
}

#[test]
fn test_ints() {
    // test ints
    assert_eq!(0_i8.is_i8(), true);
    assert_eq!(0_i16.is_i16(), true);
    assert_eq!(0_i32.is_i32(), true);
    assert_eq!(0_i64.is_i64(), true);
    assert_eq!(0_i128.is_i128(), true);

    // test uints
    assert_ne!(0_u8.is_i8(), true);
    assert_ne!(0_u16.is_i16(), true);
    assert_ne!(0_u32.is_i32(), true);
    assert_ne!(0_u64.is_i64(), true);
    assert_ne!(0_u128.is_i128(), true);
}

#[test]
fn test_uints() {
    // test ints
    assert_eq!(0_u8.is_u8(), true);
    assert_eq!(0_u16.is_u16(), true);
    assert_eq!(0_u32.is_u32(), true);
    assert_eq!(0_u64.is_u64(), true);
    assert_eq!(0_u128.is_u128(), true);

    // test uints
    assert_ne!(0_i8.is_u8(), true);
    assert_ne!(0_i16.is_u16(), true);
    assert_ne!(0_i32.is_u32(), true);
    assert_ne!(0_i64.is_u64(), true);
    assert_ne!(0_i128.is_u128(), true);
}

#[test]
fn test_floats() {
    // test ints
    assert_eq!(0_f32.is_f32(), true);
    assert_eq!(0_f64.is_f64(), true);

    // test uints
    assert_ne!(0_i32.is_f32(), true);
    assert_ne!(0_i64.is_f64(), true);
}

#[test]
fn test_size() {
    // test isize
    assert_eq!(0_isize.is_isize(), true);
    assert_ne!(0_usize.is_isize(), true);

    // test usize
    assert_eq!(0_usize.is_usize(), true);
    assert_ne!(0_isize.is_usize(), true);
}

#[test]
fn test_primitve() {
    // test str
    assert_eq!("str".is_str(), true);
    assert_ne!(b"str".is_str(), true);
    
    // test bool
    assert_eq!(true.is_bool(), true);
    assert_ne!("bool".is_bool(), true);

    // test char
    assert_eq!('c'.is_char(), true);
    assert_ne!("char".is_char(), true);
}
