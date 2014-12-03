#![crate_name="proj4"]
#![crate_type="lib"]
#![allow(non_camel_case_types)]
#![feature(globs)]

extern crate libc;

use libc::{c_double, c_int};

pub mod ffi;

pub struct Projection(ffi::projPJ);

impl Drop for Projection {
    fn drop(&mut self) {
        unsafe {
            let &Projection(pj) = self;
            ffi::pj_free(pj);
        }
    }
}

pub struct Context(ffi::projCtx);

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            let &Context(c) = self;
            ffi::pj_ctx_free(c);
        }
    }
}

pub fn init_plus(definition: &str) -> Projection {
    unsafe {
        Projection(ffi::pj_init_plus(definition.to_c_str().as_ptr()))
    }
}

pub fn transform(srcdefn: &Projection, dstdefn: &Projection, x: &mut f64, y: &mut f64) -> i32 {
    transformv(srcdefn, dstdefn, &[x], &[y])
}

pub fn transformv(srcdefn: &Projection, dstdefn: &Projection, x: &[&mut f64], y: &[&mut f64]) -> i32 {
    assert!(x.len() == y.len());
    let &Projection(src) = srcdefn;
    let &Projection(dst) = dstdefn;
    let c_x = x.as_ptr() as *mut c_double;
    let c_y = y.as_ptr() as *mut c_double;
    let c_z = 0 as *mut c_double;
    unsafe {
        ffi::pj_transform(src, dst, x.len() as i64, 1, c_x, c_y, c_z) as i32
    }
}

pub fn get_def(proj: &Projection, opts: int) -> String {
    let &Projection(pj) = proj;
    unsafe {
        let allocated = ffi::pj_get_def(pj, opts as c_int);
        let def = String::from_raw_buf(allocated as *const u8);
        ffi::pj_dalloc(allocated);
        return def;
    }
}

pub fn is_latlong(proj: &Projection) -> bool {
    let &Projection(pj) = proj;
    unsafe { ffi::pj_is_latlong(pj) == 1 }
}

pub fn is_geocent(proj: &Projection) -> bool {
    let &Projection(pj) = proj;
    unsafe { ffi::pj_is_geocent(pj) == 1 }
}

pub fn get_release() -> String {
    unsafe { String::from_raw_buf(ffi::pj_get_release() as *const u8) }
}

pub fn get_default_ctx() -> Context {
    unsafe { Context(ffi::pj_get_default_ctx()) }
}

pub fn get_ctx(proj: &Projection) -> Context {
    let &Projection(pj) = proj;
    unsafe { Context(ffi::pj_get_ctx(pj)) }
}

pub fn set_ctx(proj: &Projection, ctx: &Context) {
    let &Projection(pj) = proj;
    let &Context(ct) = ctx;
    unsafe { ffi::pj_set_ctx(pj, ct) }
}

pub fn ctx_alloc() -> Context {
    unsafe { Context(ffi::pj_ctx_alloc()) }
}

pub fn ctx_get_errno(ctx: &Context) -> i32 {
    let &Context(ct) = ctx;
    unsafe { ffi::pj_ctx_get_errno(ct) as i32 }
}

pub fn ctx_set_errno(ctx: &Context, errno: i32) {
    let &Context(ct) = ctx;
    unsafe { ffi::pj_ctx_set_errno(ct, errno as c_int) }
}

pub fn ctx_set_debug(ctx: &Context, errno: i32) {
    let &Context(ct) = ctx;
    unsafe { ffi::pj_ctx_set_debug(ct, errno as c_int) }
}

#[test]
fn basic_test() {
    let p1_def = " +proj=merc +ellps=clrk66 +lat_ts=33";
    let ll_def = " +proj=latlong +ellps=clrk66";
    let p1 = init_plus(p1_def);
    let ll = init_plus(ll_def);
    assert_eq!(p1_def, get_def(&p1, 0i).as_slice());
    assert_eq!(ll_def, get_def(&ll, 0i).as_slice());
    assert_eq!(is_latlong(&p1), false);
    assert_eq!(is_latlong(&ll), true);
    let mut x: f64 = 1000000.0;
    let mut y: f64 = 1000000.0;
    let result = transform(&p1, &ll, &mut x, &mut y);
    assert_eq!(result, 0i32);
}

#[test]
fn transform_vec() {
    let p1_def = " +proj=merc +ellps=clrk66 +lat_ts=33";
    let ll_def = " +proj=latlong +ellps=clrk66";
    let p1 = init_plus(p1_def);
    let ll = init_plus(ll_def);
    let x: [&mut f64, ..3] = [&mut 0.0f64, &mut 1_000.0, &mut 1_000_000.0];
    let y: [&mut f64, ..3] = [&mut 0.0f64, &mut 1_000.0, &mut 1_000_000.0];
    let result = transformv(&p1, &ll, &x, &y);
    assert_eq!(result, 0i32);
}

#[test]
fn test_get_release() {
    // Obviously we cannot expect/require a specific release, but
    // ensure we get something that looks valid
    let r = get_release();
    assert!(r.len() > 0);
    assert_eq!(r.as_slice()[0..3], "Rel");
}
