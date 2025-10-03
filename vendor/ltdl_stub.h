/*
 * ltdl_stub.h
 * 
 * Stub implementation of libtool's ltdl.h that maps libltdl functions
 * to standard POSIX dlopen/dlsym equivalents.
 * 
 * This allows unixODBC to be built without requiring libltdl as a dependency.
 * The macro definitions here must be in a header file (cannot be done via
 * cc::Build::define()) because they use the function-like macro syntax
 * and need to map to actual function calls.
 * 
 * This is copied to ltdl.h in vendor/unixODBC during build.
 */
#ifndef _LTDL_STUB_H
#define _LTDL_STUB_H

#include <dlfcn.h>

/* Map ltdl types to standard POSIX types */
typedef void* lt_dlhandle;

/* Map ltdl functions to standard dlopen equivalents - must be macros */
#define lt_dlopen(filename) dlopen(filename, RTLD_LAZY | RTLD_GLOBAL)
#define lt_dlsym(handle, symbol) dlsym(handle, symbol)
#define lt_dlclose(handle) dlclose(handle)
#define lt_dlerror() dlerror()
#define lt_dlinit() 0
#define lt_dlexit() 0
#define lt_dlsetsearchpath(path) 0

#define LTDL_SET_PRELOADED_SYMBOLS()

#endif /* _LTDL_STUB_H */
