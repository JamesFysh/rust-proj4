use libc::{c_void, c_int, c_long, c_double, c_char};

pub type projPJ = *mut c_void;
pub type projCtx = *mut c_void;

#[link(name = "proj")]
extern {    
    pub fn pj_init_plus(definition: *const c_char) -> projPJ;
    pub fn pj_transform(srcdefn: projPJ, dstdefn: projPJ, point_count: c_long, point_offset: c_int, x: *mut c_double, y: *mut c_double, z: *mut c_double) -> c_int;
    pub fn pj_get_def(proj: projPJ, opts: c_int) -> *mut c_char;
    pub fn pj_is_latlong(proj: projPJ) -> c_int;
    pub fn pj_is_geocent(proj: projPJ) -> c_int;
    pub fn pj_free(proj: projPJ);
    pub fn pj_dalloc(allocated: *mut c_char);
    pub fn pj_get_release() -> *const c_char;
    pub fn pj_get_default_ctx() -> projCtx;
    pub fn pj_get_ctx(proj: projPJ) -> projCtx;
    pub fn pj_set_ctx(proj: projPJ, ctx: projCtx);
    pub fn pj_ctx_alloc() -> projCtx;
    pub fn pj_ctx_free(ctx: projCtx);
    pub fn pj_ctx_get_errno(ctx: projCtx) -> c_int;
    pub fn pj_ctx_set_errno(ctx: projCtx, errno: c_int);
    pub fn pj_ctx_set_debug(ctx: projCtx, errno: c_int);
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
