//extern crate libc;
//
//use libc::{c_void, c_int, c_char, c_ulong, c_long, c_uint, c_uchar, size_t};
//use std::mem;
//use std::ptr;

#[link(name = "isl")]
extern {
    fn isl_ctx_alloc() -> Box<Ctx>;
}

// safe wrapper
fn getCtx() -> Box<Ctx>  {
    unsafe { isl_ctx_alloc() }
}

//isl_ctx *ctx = isl_ctx_alloc();

#[repr(C)]
#[derive(Clone, Copy)]
// TODO align with isl/include/isl/isl_ctx_private.h
struct Ctx {
    ref_: i32,
}

#[test]
fn it_works() {
    // see https://github.com/rust-lang/rust/issues/20204
    //let ctx = getCtx();
    unsafe {
        let ctx = isl_ctx_alloc();
    }
}
