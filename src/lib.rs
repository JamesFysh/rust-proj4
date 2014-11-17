#![crate_name="proj4"]
#![crate_type="lib"]
#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_void, c_int, c_long, c_double, c_char};

pub type projPJ = *mut c_void;
pub type projCtx = *mut c_void;

pub struct Projection {
    p: projPJ
}

impl Projection {
    pub fn new(p: projPJ) -> Projection {
        Projection {
            p: p
        }
    }
}

pub struct Context {
    c: projCtx
}

impl Context {
    pub fn new(c: projCtx) -> Context {
        Context {
            c: c
        }
    }
}

#[link(name = "proj")]
extern {    
    fn pj_init_plus(definition: *const c_char) -> projPJ;
    fn pj_transform(srcdefn: projPJ, dstdefn: projPJ, point_count: c_long, point_offset: c_int, x: *mut c_double, y: *mut c_double, z: *mut c_double) -> c_int;
    fn pj_get_def(proj: projPJ, opts: c_int) -> *mut c_char;
    fn pj_is_latlong(proj: projPJ) -> c_int;
    fn pj_is_geocent(proj: projPJ) -> c_int;
    fn pj_free(proj: projPJ);
    fn pj_dalloc(allocated: *mut c_char);
    fn pj_get_release() -> *const c_char;
    fn pj_get_default_ctx() -> projCtx;
    fn pj_get_ctx(proj: projPJ) -> projCtx;
    fn pj_set_ctx(proj: projPJ, ctx: projCtx);
    fn pj_ctx_alloc() -> projCtx;
    fn pj_ctx_free(ctx: projCtx);
    fn pj_ctx_get_errno(ctx: projCtx) -> c_int;
    fn pj_ctx_set_errno(ctx: projCtx, errno: c_int);
    fn pj_ctx_set_debug(ctx: projCtx, errno: c_int);
    fn pj_ctx_set_app_data(ctx: projCtx, data: *mut c_void);
    fn pj_ctx_get_app_data(ctx: projCtx) -> *mut c_void;
}

pub fn init_plus(definition: &str) -> projPJ {
    unsafe {
        pj_init_plus(definition.to_c_str().as_ptr())
    }
}

pub fn transform(srcdefn: projPJ, dstdefn: projPJ, x: &mut f64, y: &mut f64) -> i32 {
    unsafe {
        pj_transform(srcdefn, dstdefn, 1, 1, x as *mut c_double, y as *mut c_double, 0 as *mut c_double) as i32
    }
}

pub fn get_def(proj: projPJ, opts: int) -> String {
    unsafe {
        let allocated = pj_get_def(proj, opts as c_int);
        let def = std::string::raw::from_buf(allocated as *const u8);
        pj_dalloc(allocated);
        return def;
    }
}

pub fn is_latlong(proj: projPJ) -> bool {
    unsafe {
        pj_is_latlong(proj) == 1
    }
}

pub fn is_geocent(proj: projPJ) -> bool {
    unsafe {
        pj_is_geocent(proj) == 1
    }
}

pub fn free(proj: projPJ) {
    unsafe {
        pj_free(proj)
    }
}

pub fn get_release() -> String {
    unsafe {
        std::string::raw::from_buf(pj_get_release() as *const u8)
    }
}

pub fn get_default_ctx() -> projCtx {
    unsafe {
        pj_get_default_ctx()
    }
}

pub fn get_ctx(proj: projPJ) -> projCtx {
    unsafe {
        pj_get_ctx(proj)
    }
}

pub fn set_ctx(proj: projPJ, ctx: projCtx) {
    unsafe {
        pj_set_ctx(proj, ctx)
    }
}

pub fn ctx_alloc() -> projCtx {
    unsafe {
        pj_ctx_alloc()
    }
}

pub fn ctx_free(ctx: projCtx) {
    unsafe {
        pj_ctx_free(ctx)
    }
}

pub fn ctx_get_errno(ctx: projCtx) -> i32 {
    unsafe {
        pj_ctx_get_errno(ctx) as i32
    }
}

pub fn ctx_set_errno(ctx: projCtx, errno: i32) {
    unsafe {
        pj_ctx_set_errno(ctx, errno as c_int)
    }
}

pub fn ctx_set_debug(ctx: projCtx, errno: i32) {
    unsafe {
        pj_ctx_set_debug(ctx, errno as c_int)
    }
}

#[test]
fn basic_test() {
    let p1_def = " +proj=merc +ellps=clrk66 +lat_ts=33";
    let ll_def = " +proj=latlong +ellps=clrk66";
    let p1 = init_plus(p1_def);
    let ll = init_plus(ll_def);
    assert_eq!(p1_def, get_def(p1, 0i).as_slice());
    assert_eq!(ll_def, get_def(ll, 0i).as_slice());
    assert_eq!(is_latlong(p1), false);
    assert_eq!(is_latlong(ll), true);
    free(p1);
    free(ll);
}

/*
#ifndef PROJ_API_H
#define PROJ_API_H

#include <math.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

#define PJ_VERSION 480

extern char const pj_release[];

#define RAD_TO_DEG	57.29577951308232
#define DEG_TO_RAD	.0174532925199432958


extern int pj_errno;

#if !defined(PROJECTS_H)
    typedef struct { double u, v; } projUV;
    typedef void *projPJ;
    #define projXY projUV
    #define projLP projUV
    typedef void *projCtx;
#else
    typedef PJ *projPJ;
    typedef projCtx_t *projCtx;
#   define projXY	XY
#   define projLP       LP
#endif


projXY pj_fwd(projLP, projPJ);
projLP pj_inv(projXY, projPJ);

int pj_transform( projPJ src, projPJ dst, long point_count, int point_offset,
                  double *x, double *y, double *z );
int pj_datum_transform( projPJ src, projPJ dst, long point_count, int point_offset,
                        double *x, double *y, double *z );
int pj_geocentric_to_geodetic( double a, double es,
                               long point_count, int point_offset,
                               double *x, double *y, double *z );
int pj_geodetic_to_geocentric( double a, double es,
                               long point_count, int point_offset,
                               double *x, double *y, double *z );
int pj_compare_datums( projPJ srcdefn, projPJ dstdefn );
int pj_apply_gridshift( projCtx, const char *, int, 
                        long point_count, int point_offset,
                        double *x, double *y, double *z );
void pj_deallocate_grids(void);
void pj_clear_initcache(void);
int pj_is_latlong(projPJ);
int pj_is_geocent(projPJ);
void pj_get_spheroid_defn(projPJ defn, double *major_axis, double *eccentricity_squared);
void pj_pr_list(projPJ);
void pj_free(projPJ);
void pj_set_finder( const char *(*)(const char *) );
void pj_set_searchpath ( int count, const char **path );
projPJ pj_init(int, char **);
projPJ pj_init_plus(const char *);
projPJ pj_init_ctx( projCtx, int, char ** );
projPJ pj_init_plus_ctx( projCtx, const char * );
char *pj_get_def(projPJ, int);
projPJ pj_latlong_from_proj( projPJ );
void *pj_malloc(size_t);
void pj_dalloc(void *);
char *pj_strerrno(int);
int *pj_get_errno_ref(void);
const char *pj_get_release(void);
void pj_acquire_lock(void);
void pj_release_lock(void);
void pj_cleanup_lock(void);

projCtx pj_get_default_ctx(void);
projCtx pj_get_ctx( projPJ );
void pj_set_ctx( projPJ, projCtx );
projCtx pj_ctx_alloc(void);
void    pj_ctx_free( projCtx );
int pj_ctx_get_errno( projCtx );
void pj_ctx_set_errno( projCtx, int );
void pj_ctx_set_debug( projCtx, int );
void pj_ctx_set_logger( projCtx, void (*)(void *, int, const char *) );
void pj_ctx_set_app_data( projCtx, void * );
void *pj_ctx_get_app_data( projCtx );

void pj_log( projCtx ctx, int level, const char *fmt, ... );
void pj_stderr_logger( void *, int, const char * );

#define PJ_LOG_NONE        0
#define PJ_LOG_ERROR       1
#define PJ_LOG_DEBUG_MAJOR 2
#define PJ_LOG_DEBUG_MINOR 3

#ifdef __cplusplus
}
#endif

#endif 

*/
