pub unsafe fn allocate<T>() -> *mut T {
    std::alloc::alloc(std::alloc::Layout::new::<T>()) as *mut T
}

#[allow(dead_code)]
pub unsafe fn deallocate<T>(obj: *mut T) {
    std::alloc::dealloc(obj as *mut u8, std::alloc::Layout::new::<T>())
}

#[macro_export]
macro_rules! unwrap {
    ($enum_obj: expr, $pattern: pat, $inner: expr) => {
        if let $pattern = $enum_obj {
            $inner
        } else {
            panic!()
        }
    };
}
