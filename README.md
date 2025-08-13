# LLVM-IR Annotation Research (Rust)
Experimenting with ways to annotate selected code within Rust and have it transferred to LLVM-IR through the use of intrinsics. This is built with future automation in mind instead
of relying for manual calls.

## Annotation Macro

### Sample
```rust
let data = Box::new(45)
```

### Calling LLVM Intrinsic
[**annotate!**](https://github.com/denmarkx/rust-llvm-instrinics-test/blob/main/annotation/src/lib.rs) macro will call LLVM intrinsic `llvm_ptr_annotation_p0`.
```rust
annotate!(data = Box::new(45), <annotation string>)
```
Macro expands to:
```rust
let data = Box::new(45);
unsafe {
    llvm_ptr_annotation_p0(
        &data as *const _ as *const u8,
        concat!(<annotation string>, "\0").as_ptr(),
        file!().as_ptr(),
        line!() as i32,
    );
}
```

## LLVM-IR Code:
### [Intrinsic Signature](https://llvm.org/docs/LangRef.html)

```llvm
declare ptr @llvm.ptr.annotation.p0(ptr <val>, ptr <str>, ptr <str>, i32 <int>)
```

### LLVM-IR Code
```llvm
@alloc21 = private unnamed_addr constant <{ [18 x i8] }> <{ [18 x i8] <annotation string> }>, align 1
@alloc24 = private unnamed_addr constant <{ [11 x i8] }> <{ [11 x i8] c"src\\main.rs" }>, align 1
...
%_2 = call i8* @llvm.ptr.annotation.p0i8(
    ; pointer to value
    i8* nonnull %0,

    ; pointer to the global annotation string
    i8* getelementptr inbounds (
        <{ [18 x i8] }>,
        <{ [18 x i8] }>* @alloc21,
        i64 0, i32 0, i64 0
    ),
    ; pointer to the global source filename
    i8* getelementptr inbounds (
        <{ [11 x i8] }>,
        <{ [11 x i8] }>* @alloc24,
        i64 0, i32 0, i64 
    ),
    ; line number
    i32 6,

    i8* null)
```

### Tracing Register %0
- The annotation points to %0, per the requirement of `llvm.ptr.annotation.p0`'s signature.
- Tracing %0 shows it was originally casted as an `i8*` for the purpose of `llvm.lifetime.start`.
- Watching %0 -> %_2 just shows the details of 45 being written from our original Rust code.

```llvm
; Stack frame allocation.
%data = alloca i32*, align 8

; data as i8 to comply with llvm.lifetime.start signature
%0 = bitcast i32** %data to i8*
call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %0)

; Heap Allocation (Recall Box::new)
%1 = tail call dereferenceable_or_null(4) i8* @__rust_alloc(i64 4, i64 4) #10

; ...

; Writing value 45 into heap block.
%3 = bitcast i8* %1 to i32*
store i32 45, i32* %3, align 4

; Storing the heap pointer into local var.
%4 = bitcast i32** %data to i8**
store i8* %1, i8** %4, align 8

; %_2 (annotation call on %0)
```
