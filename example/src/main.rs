extern crate annotation;
use annotation::{llvm_ptr_annotation_p0, annotate};
use std::boxed::Box;

fn main() {
    annotate!(data = Box::new(95), "BOX_ANNOTATION_95", line!());
    annotate!(data2 = Box::new(700), "BOX_ANNOTATION_700", line!());
}
