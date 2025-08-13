#![feature(link_llvm_intrinsics)]

extern "C" {
    #[link_name = "llvm.ptr.annotation.p0"]
    pub fn llvm_ptr_annotation_p0(
        val: *const u8,
        annotation: *const u8,
        file: *const u8,
        line: i32,
    ) -> *const u8;
}

#[macro_export]
macro_rules! annotate {
    ($var:ident = $value:expr, $annotation:literal) => {
        let $var = $value;
        unsafe {
            llvm_ptr_annotation_p0(
                &$var as *const _ as *const u8,
                concat!($annotation, "\0").as_ptr(),
                file!().as_ptr(),
                line!() as i32,
            );
        }
    };
}
