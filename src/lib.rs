pub mod derg_resonite {
    use std::marker::PhantomData;

    mod native {
        use super::*;

        #[link(wasm_import_module = "env")]
        extern "C" {
            pub fn slot__root_slot() -> RefId<Slot>;
            pub fn slot__get_parent(slot: RefId<Slot>) -> RefId<Slot>;
            pub fn slot__get_active_user(slot: RefId<Slot>) -> RefId<User>;
            pub fn slot__get_active_user_root(slot: RefId<Slot>) -> RefId<UserRoot>;
            pub fn slot__get_object_root(slot: RefId<Slot>, only_explicit: i32) -> RefId<Slot>;
            pub fn slot__get_name(slot: RefId<Slot>) -> *mut std::ffi::c_char;
            pub fn value_field__bool__set_value(field: ValueFieldComponent<bool>, value: i32);
            pub fn value_field__bool__get_value(field: ValueFieldComponent<bool>) -> i32;
        }
    }

    #[repr(C)]
    pub struct RefLow<T>(u32, PhantomData<T>);

    impl<T> Copy for RefLow<T> {}

    impl<T> Clone for RefLow<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }

    impl<T> Default for RefLow<T> {
        fn default() -> Self {
            Self(Default::default(), Default::default())
        }
    }

    #[repr(C)]
    pub struct RefHigh<T>(u32, PhantomData<T>);

    impl<T> Clone for RefHigh<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }

    impl<T> Copy for RefHigh<T> {}

    impl<T> Default for RefHigh<T> {
        fn default() -> Self {
            Self(Default::default(), Default::default())
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct RefId<T>(RefLow<T>, RefHigh<T>, PhantomData<T>);

    impl<T> Default for RefId<T> {
        fn default() -> Self {
            Self(Default::default(), Default::default(), Default::default())
        }
    }

    impl<T> RefId<T> {
        pub fn valid(&self) -> bool {
            self.0 .0 != 0 && self.1 .0 != 0
        }

        pub fn low(&self) -> RefLow<T> {
            self.0
        }

        pub fn high(&self) -> RefHigh<T> {
            self.1
        }
    }

    impl<T> RefId<T>
    where
        T: From<RefId<T>>,
    {
        fn filter_invalid(self) -> Option<T> {
            if self.0 .0 != 0 && self.1 .0 != 0 {
                Some(self.into())
            } else {
                None
            }
        }
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct User(RefId<User>);

    impl From<RefId<Self>> for User {
        fn from(value: RefId<Self>) -> Self {
            Self(value)
        }
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct UserRoot(RefId<UserRoot>);

    impl From<RefId<Self>> for UserRoot {
        fn from(value: RefId<Self>) -> Self {
            Self(value)
        }
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Slot(RefId<Slot>);

    impl From<RefId<Self>> for Slot {
        fn from(value: RefId<Self>) -> Self {
            Self(value)
        }
    }

    impl Slot {
        pub fn get_root_slot() -> Slot {
            unsafe { native::slot__root_slot() }
                .filter_invalid()
                .unwrap()
        }

        pub fn get_parent(&self) -> Option<Slot> {
            unsafe { native::slot__get_parent(self.0.clone()) }.filter_invalid()
        }

        pub fn get_active_user(&self) -> Option<User> {
            unsafe { native::slot__get_active_user(self.0.clone()) }.filter_invalid()
        }

        pub fn get_active_user_root(&self) -> Option<UserRoot> {
            unsafe { native::slot__get_active_user_root(self.0.clone()) }.filter_invalid()
        }

        pub fn get_object_root(&self, only_explicit: bool) -> Option<Slot> {
            unsafe {
                native::slot__get_object_root(self.0.clone(), if only_explicit { 1 } else { 0 })
            }
            .filter_invalid()
        }

        pub fn get_name(&self) -> Option<Box<str>> {
            let _str = unsafe { native::slot__get_name(self.0.clone()) };
            None
            /*if str.is_null() { None } else {
                 let cstr = unsafe { std::ffi::CStr::from_ptr(str) };
                std::str::from_boxed_utf8_unchecked(Box::from_raw(cstr.to_bytes_with_nul()))
            }
            */
        }
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Component(u64);

    #[repr(C)]
    #[derive(Clone)]
    pub struct ValueFieldComponent<T>(Component, PhantomData<T>);

    pub trait ValueField<T> {
        fn get_value(&self) -> T;
        fn set_value(&self, value: T);
    }

    impl ValueField<bool> for ValueFieldComponent<bool> {
        fn get_value(&self) -> bool {
            unsafe { native::value_field__bool__get_value(self.clone()) != 0 }
        }
        fn set_value(&self, value: bool) {
            unsafe {
                native::value_field__bool__set_value(self.clone(), if value { 1 } else { 0 });
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn do_stuff() {
    let root = derg_resonite::Slot::get_root_slot();
    println!("Hello, world from {:?}", root.get_name());
}
