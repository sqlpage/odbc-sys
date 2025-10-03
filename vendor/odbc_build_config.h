/* 
 * odbc_build_config.h
 * 
 * Minimal config.h replacement for unixODBC/iODBC static compilation.
 * 
 * This file contains ONLY preprocessor directives that cannot be expressed
 * using cc::Build::define() in build.rs, specifically:
 * - Platform-dependent conditionals (#ifdef __linux__, #ifdef __APPLE__)
 * - Target-dependent type sizes (#if defined(__LP64__))
 * - Compatibility guards (#ifndef TRUE/FALSE)
 * 
 * All other configuration is done via build.define() calls in build.rs.
 * This is copied to config.h in the vendor directories during build.
 */
#ifndef _ODBC_BUILD_CONFIG_H
#define _ODBC_BUILD_CONFIG_H

/* Type sizes - requires preprocessor evaluation of __LP64__ */
#if defined(__LP64__) || defined(_WIN64)
#define SIZEOF_LONG_INT 8
#else
#define SIZEOF_LONG_INT 4
#endif

/* Platform detection - requires preprocessor evaluation of platform macros */
#ifdef __linux__
#define PLATFORM_LINUX 1
#define SHLIBEXT ".so"
#endif

#ifdef __APPLE__
#define PLATFORM_MACOS 1
#define SHLIBEXT ".dylib"
#endif

/* Common boolean values - use #ifndef to allow override */
#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

#endif /* _ODBC_BUILD_CONFIG_H */
