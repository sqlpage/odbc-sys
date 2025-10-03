/* Minimal config.h for unixODBC/iODBC static compilation */
#ifndef _CONFIG_H
#define _CONFIG_H

/* Type sizes - needs preprocessor evaluation */
#if defined(__LP64__) || defined(_WIN64)
#define SIZEOF_LONG_INT 8
#else
#define SIZEOF_LONG_INT 4
#endif

/* Platform detection - needs preprocessor evaluation */
#ifdef __linux__
#define PLATFORM_LINUX 1
#define SHLIBEXT ".so"
#endif

#ifdef __APPLE__
#define PLATFORM_MACOS 1
#define SHLIBEXT ".dylib"
#endif

/* Common boolean and status values - use #ifndef for compatibility */
#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

#endif /* _CONFIG_H */
