//extern crate libc;
//
//use libc::{c_void, c_int, c_char, c_ulong, c_long, c_uint, c_uchar, size_t};
//use std::mem;
//use std::ptr;

#[link(name = "isl")]
extern {
    //fn isl_ctx_alloc() -> Box<Ctx>;
    fn isl_ctx_alloc() -> *mut i64;
}

// safe wrapper
//fn getCtx() -> Box<Ctx>  {
fn getCtx() -> *mut i64  {
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
    // used to have with boxed values: rust je_arena_dalloc_bin_locked
    let ctx = getCtx();
    unsafe {
      assert!(*ctx != 0);
    }
}
