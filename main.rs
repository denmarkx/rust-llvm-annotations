#![feature(link_llvm_intrinsics)]
// https://llvm.org/docs/LangRef.html#llvm-annotation-intrinsic
// https://llvm.org/docs/LangRef.html#metadata
// https://www.cs.cornell.edu/~asampson/blog/llvm.html

unsafe extern "C" {
    #[link_name = "llvm.annotation.i32"]
    // annotation: *const u8
    // @alloc_... = private unnamed_addr constant [21 x i8] c"llvm_annotation_test\00", align 1

    // file: *const u8
    // @alloc = private unnamed_addr constant [8 x i8] c"main.rs\00", align 1

    // Actual annotation:
    // %x = call i32 @llvm.annotation.i32.p0(i32 15, ptr @alloc_1be9d07cbf3189baa909ef2f3e478e5e,
    //                                 ptr @alloc_fb681ad8228c3f7ebbf1c03e060f49b7, i32 14) #5
    fn llvm_annotation_i32(val: i32, annotation: *const u8, file: *const u8, line: i32) -> i32;
}

fn main() {
    let test = 15;
    let x = unsafe {
        llvm_annotation_i32(
            test,
            b"llvm_annotation_test\0".as_ptr(),
            b"main.rs\0".as_ptr(),
            line!() as i32
        )
    };
}
