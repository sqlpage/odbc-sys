/* Minimal config.h for unixODBC/iODBC static compilation */
#ifndef _CONFIG_H
#define _CONFIG_H

/* Define standard headers */
#define HAVE_STDLIB_H 1
#define HAVE_STRING_H 1
#define HAVE_UNISTD_H 1
#define HAVE_PWD_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_STDARG_H 1
#define HAVE_TIME_H 1
#define HAVE_ERRNO_H 1
#define HAVE_MALLOC_H 1
#define HAVE_DLFCN_H 1
#define HAVE_CTYPE_H 1
#define HAVE_LIMITS_H 1
#define HAVE_PTHREAD_H 1
#define HAVE_SYS_PARAM_H 1

/* Define standard functions */
#define HAVE_LONG_LONG 1
#define HAVE_STRTOL 1
#define HAVE_STRTOLL 1
#define HAVE_ATOLL 1
#define HAVE_STRNCASECMP 1
#define HAVE_VSNPRINTF 1
#define HAVE_SNPRINTF 1
#define HAVE_STRCASECMP 1
#define HAVE_STRDUP 1
#define HAVE_SETLOCALE 1
#define HAVE_MEMSET 1
#define HAVE_MEMCPY 1
#define HAVE_PUTENV 1
#define HAVE_STRERROR 1
#define HAVE_LOCALTIME_R 1

/* Type sizes */
#if defined(__LP64__) || defined(_WIN64)
#define SIZEOF_LONG_INT 8
#else
#define SIZEOF_LONG_INT 4
#endif

/* Threading support */
#define HAVE_LIBPTHREAD 1

/* Dynamic loading */
#define HAVE_LIBDL 1

/* Package info */
#define PACKAGE "unixODBC"
#define VERSION "2.3.12"

/* Platform detection */
#ifdef __linux__
#define PLATFORM_LINUX 1
#define SHLIBEXT ".so"
#endif

#ifdef __APPLE__
#define PLATFORM_MACOS 1
#define SHLIBEXT ".dylib"
#endif

#ifdef _WIN32
#define SHLIBEXT ".dll"
#endif

/* ODBC settings */
#define ENABLE_UNICODE_SUPPORT 1
#define SQL_WCHART_CONVERT 1

/* Disable ltdl usage - we'll use dlopen directly */
#define DISABLE_LTDL 1

/* Default system file paths */
#define SYSTEM_FILE_PATH "/etc"
#define ODBCINST_SYSTEM_INI "odbcinst.ini"
#define ODBC_SYSTEM_INI "odbc.ini"

/* Common boolean and status values */
#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

/* INI library return codes */
#define INI_SUCCESS 0
#define INI_ERROR 1

/* Logging levels */
#define LOG_CRITICAL 0
#define LOG_ERROR 1
#define LOG_WARNING 2
#define LOG_INFO 3
#define LOG_DEBUG 4

#endif /* _CONFIG_H */
