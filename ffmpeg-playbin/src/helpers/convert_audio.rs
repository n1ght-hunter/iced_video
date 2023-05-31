pub trait ToType {
    fn to_type<Type>(&self) -> &[Type];
}

impl ToType for [u8] {
    fn to_type<Type>(&self) -> &[Type] {
        let len = self.len() / std::mem::size_of::<Type>();
        assert!(self.len() % std::mem::size_of::<Type>() == 0);
        let ptr = self.as_ptr() as *const Type;
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}
