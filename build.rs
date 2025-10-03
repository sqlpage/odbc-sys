use std::path::Path;
use std::process::Command;

fn main() {
    if std::env::var("CARGO_FEATURE_STATIC").is_ok() {
        assert!(
            !cfg!(target_os = "windows"),
            "odbc-sys does not currently support static linking on windows"
        );

        // Check if user wants to provide their own static library path
        if let Ok(static_path) = std::env::var("ODBC_SYS_STATIC_PATH") {
            // User-provided static library path (original behavior)
            println!("cargo:rerun-if-env-changed=ODBC_SYS_STATIC_PATH");
            println!("cargo:rustc-link-search=native={static_path}");
            println!("cargo:rustc-link-lib=static=odbc");
            println!("cargo:rustc-link-lib=static=ltdl");
            if cfg!(target_os = "macos") {
                // Homebrew's unixodbc uses the system iconv, so we can't do a fully static linking
                // but this way we at least have only dependencies on built-in libraries
                // See also https://github.com/Homebrew/homebrew-core/pull/46145
                println!("cargo:rustc-link-lib=dylib=iconv");
            }
        } else {
            // When the static feature is enabled without ODBC_SYS_STATIC_PATH,
            // compile the ODBC driver manager from source
            #[cfg(feature = "static")]
            compile_odbc_from_source();

            #[cfg(not(feature = "static"))]
            panic!("When using the 'static' feature without ODBC_SYS_STATIC_PATH, the 'cc' crate must be enabled as a build dependency");
        }
    }

    if cfg!(target_os = "macos") {
        if let Some(homebrew_lib_path) = homebrew_library_path() {
            print_paths(&homebrew_lib_path);
        }

        // if we're on Mac OS X we'll kindly add DYLD_LIBRARY_PATH to rustc's
        // linker search path
        if let Some(dyld_paths) = option_env!("DYLD_LIBRARY_PATH") {
            print_paths(dyld_paths);
        }
        // if we're on Mac OS X we'll kindly add DYLD_FALLBACK_LIBRARY_PATH to rustc's
        // linker search path
        if let Some(dyld_fallback_paths) = option_env!("DYLD_FALLBACK_LIBRARY_PATH") {
            print_paths(dyld_fallback_paths);
        }
    }
}

#[cfg(all(feature = "static", feature = "iodbc"))]
fn compile_odbc_from_source() {
    compile_iodbc();
}

#[cfg(all(feature = "static", not(feature = "iodbc")))]
fn compile_odbc_from_source() {
    compile_unixodbc();
}

#[cfg(feature = "static")]
fn ensure_configured(vendor_dir: &Path) -> std::io::Result<()> {
    let config_h = vendor_dir.join("config.h");

    // Check if config.h already exists
    if config_h.exists() {
        return Ok(());
    }

    // Generate minimal config.h without requiring autoconf/automake/libtool
    // This makes the build self-contained and eliminates external tool dependencies
    create_minimal_config_h(vendor_dir)
}

#[cfg(feature = "static")]
#[allow(dead_code)]
fn create_minimal_config_h(vendor_dir: &Path) -> std::io::Result<()> {
    let config_h_src = Path::new("vendor/odbc_build_config.h");
    let config_h_dest = vendor_dir.join("config.h");

    std::fs::copy(config_h_src, &config_h_dest)?;
    Ok(())
}

#[cfg(feature = "static")]
#[allow(dead_code)]
fn create_ltdl_stub(vendor_dir: &Path) -> std::io::Result<()> {
    let ltdl_h_src = Path::new("vendor/ltdl_stub.h");
    let ltdl_h_dest = vendor_dir.join("ltdl.h");

    std::fs::copy(ltdl_h_src, &ltdl_h_dest)?;
    Ok(())
}

#[cfg(feature = "static")]
#[allow(dead_code)]
fn compile_unixodbc() {
    let vendor_dir = Path::new("vendor/unixODBC");

    // Ensure config.h exists
    if let Err(e) = ensure_configured(vendor_dir) {
        println!("cargo:warning=Failed to configure unixODBC: {}", e);
    }

    // Create ltdl.h stub
    if let Err(e) = create_ltdl_stub(vendor_dir) {
        println!("cargo:warning=Failed to create ltdl.h stub: {}", e);
    }

    let mut build = cc::Build::new();

    // Add include paths
    build.include(vendor_dir.join("include"));
    build.include(vendor_dir.join("DriverManager"));
    build.include(vendor_dir.join("odbcinst"));
    build.include(vendor_dir.join("ini"));
    build.include(vendor_dir.join("log"));
    build.include(vendor_dir.join("lst"));
    build.include(vendor_dir);

    // Add common compiler flags
    build.flag_if_supported("-fPIC");
    build.flag_if_supported("-std=gnu99"); // Use C99 standard for modern C features
    build.flag_if_supported("-Wno-error"); // Don't treat warnings as errors
    build.flag_if_supported("-Wno-implicit-function-declaration"); // Allow implicit function declarations
    build.flag_if_supported("-Wno-int-conversion");
    build.flag_if_supported("-w"); // Suppress warnings from vendor code

    // Define common macros
    build.define("HAVE_CONFIG_H", None);
    build.define("UNIXODBC_SOURCE", None); // Required for internal headers

    // Define standard headers
    build.define("HAVE_STDLIB_H", "1");
    build.define("HAVE_STRING_H", "1");
    build.define("HAVE_UNISTD_H", "1");
    build.define("HAVE_PWD_H", "1");
    build.define("HAVE_SYS_TYPES_H", "1");
    build.define("HAVE_STDARG_H", "1");
    build.define("HAVE_TIME_H", "1");
    build.define("HAVE_ERRNO_H", "1");
    build.define("HAVE_MALLOC_H", "1");
    build.define("HAVE_DLFCN_H", "1");
    build.define("HAVE_CTYPE_H", "1");
    build.define("HAVE_LIMITS_H", "1");
    build.define("HAVE_PTHREAD_H", "1");
    build.define("HAVE_SYS_PARAM_H", "1");

    // Define standard functions
    build.define("HAVE_LONG_LONG", "1");
    build.define("HAVE_STRTOL", "1");
    build.define("HAVE_STRTOLL", "1");
    build.define("HAVE_ATOLL", "1");
    build.define("HAVE_STRNCASECMP", "1");
    build.define("HAVE_VSNPRINTF", "1");
    build.define("HAVE_SNPRINTF", "1");
    build.define("HAVE_STRCASECMP", "1");
    build.define("HAVE_STRDUP", "1");
    build.define("HAVE_SETLOCALE", "1");
    build.define("HAVE_MEMSET", "1");
    build.define("HAVE_MEMCPY", "1");
    build.define("HAVE_PUTENV", "1");
    build.define("HAVE_STRERROR", "1");
    build.define("HAVE_LOCALTIME_R", "1");

    // Threading and dynamic loading
    build.define("HAVE_LIBPTHREAD", "1");
    build.define("HAVE_LIBDL", "1");

    // Package info
    build.define("PACKAGE", "\"unixODBC\"");
    build.define("VERSION", "\"2.3.12\"");

    // ODBC settings
    build.define("ENABLE_UNICODE_SUPPORT", "1");
    build.define("SQL_WCHART_CONVERT", "1");
    build.define("DISABLE_LTDL", "1");

    // INI library return codes
    build.define("INI_SUCCESS", "0");
    build.define("INI_ERROR", "1");

    // Logging levels
    build.define("LOG_CRITICAL", "0");
    build.define("LOG_ERROR", "1");
    build.define("LOG_WARNING", "2");
    build.define("LOG_INFO", "3");
    build.define("LOG_DEBUG", "4");

    // Platform-specific defines
    if cfg!(target_os = "linux") {
        build.define("DEFLIB_PATH", "\"/usr/lib:/usr/local/lib\"");
        build.define("SYSTEM_FILE_PATH", "\"/etc\"");
        build.define("ODBCINST_SYSTEM_INI", "\"odbcinst.ini\"");
        build.define("ODBC_SYSTEM_INI", "\"odbc.ini\"");
    } else if cfg!(target_os = "macos") {
        build.define("DEFLIB_PATH", "\"/usr/lib:/usr/local/lib\"");
        build.define("SYSTEM_FILE_PATH", "\"/etc\"");
        build.define("ODBCINST_SYSTEM_INI", "\"odbcinst.ini\"");
        build.define("ODBC_SYSTEM_INI", "\"odbc.ini\"");
    }

    // Collect all source files from DriverManager
    let driver_manager_dir = vendor_dir.join("DriverManager");
    add_c_files(&mut build, &driver_manager_dir);

    // Collect all source files from odbcinst
    let odbcinst_dir = vendor_dir.join("odbcinst");
    add_c_files(&mut build, &odbcinst_dir);

    // Collect all source files from ini
    let ini_dir = vendor_dir.join("ini");
    add_c_files(&mut build, &ini_dir);

    // Collect all source files from log
    let log_dir = vendor_dir.join("log");
    add_c_files(&mut build, &log_dir);

    // Collect all source files from lst
    let lst_dir = vendor_dir.join("lst");
    add_c_files(&mut build, &lst_dir);

    // Compile the library
    build.compile("odbc");

    // Link additional dependencies
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=iconv");
    }
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    println!("cargo:rerun-if-changed=vendor/unixODBC");
}

#[cfg(feature = "static")]
#[allow(dead_code)]
fn compile_iodbc() {
    let vendor_dir = Path::new("vendor/iODBC");

    // Ensure config.h exists
    if let Err(e) = ensure_configured(vendor_dir) {
        println!("cargo:warning=Failed to configure iODBC: {}", e);
    }

    let mut build = cc::Build::new();

    // Add include paths
    build.include(vendor_dir.join("include"));
    build.include(vendor_dir.join("iodbc"));
    build.include(vendor_dir.join("iodbcinst"));
    build.include(vendor_dir.join("iodbcadm"));
    build.include(vendor_dir);

    // Add common compiler flags
    build.flag_if_supported("-fPIC");
    build.flag_if_supported("-std=gnu99"); // Use C99 standard for modern C features
    build.flag_if_supported("-Wno-error"); // Don't treat warnings as errors
    build.flag_if_supported("-Wno-implicit-function-declaration"); // Allow implicit function declarations
    build.flag_if_supported("-Wno-int-conversion");
    build.flag_if_supported("-w"); // Suppress warnings from vendor code

    // Define common macros for iODBC
    build.define("HAVE_CONFIG_H", None);

    // Define standard headers
    build.define("HAVE_STDLIB_H", "1");
    build.define("HAVE_STRING_H", "1");
    build.define("HAVE_UNISTD_H", "1");
    build.define("HAVE_PWD_H", "1");
    build.define("HAVE_SYS_TYPES_H", "1");
    build.define("HAVE_STDARG_H", "1");
    build.define("HAVE_TIME_H", "1");
    build.define("HAVE_ERRNO_H", "1");
    build.define("HAVE_MALLOC_H", "1");
    build.define("HAVE_DLFCN_H", "1");
    build.define("HAVE_CTYPE_H", "1");
    build.define("HAVE_LIMITS_H", "1");
    build.define("HAVE_PTHREAD_H", "1");
    build.define("HAVE_SYS_PARAM_H", "1");

    // Define standard functions
    build.define("HAVE_LONG_LONG", "1");
    build.define("HAVE_STRTOL", "1");
    build.define("HAVE_STRTOLL", "1");
    build.define("HAVE_ATOLL", "1");
    build.define("HAVE_STRNCASECMP", "1");
    build.define("HAVE_VSNPRINTF", "1");
    build.define("HAVE_SNPRINTF", "1");
    build.define("HAVE_STRCASECMP", "1");
    build.define("HAVE_STRDUP", "1");
    build.define("HAVE_SETLOCALE", "1");
    build.define("HAVE_MEMSET", "1");
    build.define("HAVE_MEMCPY", "1");
    build.define("HAVE_PUTENV", "1");
    build.define("HAVE_STRERROR", "1");
    build.define("HAVE_LOCALTIME_R", "1");

    // Threading and dynamic loading
    build.define("HAVE_LIBPTHREAD", "1");
    build.define("HAVE_LIBDL", "1");

    // Package info
    build.define("PACKAGE", "\"iODBC\"");
    build.define("VERSION", "\"3.52.16\"");

    // ODBC settings
    build.define("ENABLE_UNICODE_SUPPORT", "1");
    build.define("SQL_WCHART_CONVERT", "1");

    // Platform-specific defines
    if cfg!(target_os = "linux") {
        build.define("DEFLIB_PATH", "\"/usr/lib:/usr/local/lib\"");
        build.define("SYSTEM_FILE_PATH", "\"/etc\"");
        build.define("ODBCINST_SYSTEM_INI", "\"odbcinst.ini\"");
        build.define("ODBC_SYSTEM_INI", "\"odbc.ini\"");
    } else if cfg!(target_os = "macos") {
        build.define("DEFLIB_PATH", "\"/usr/lib:/usr/local/lib\"");
        build.define("SYSTEM_FILE_PATH", "\"/etc\"");
        build.define("ODBCINST_SYSTEM_INI", "\"odbcinst.ini\"");
        build.define("ODBC_SYSTEM_INI", "\"odbc.ini\"");
    }

    // Collect all source files from iodbc
    let iodbc_dir = vendor_dir.join("iodbc");
    add_c_files(&mut build, &iodbc_dir);

    // Collect trace files from iodbc/trace
    let trace_dir = vendor_dir.join("iodbc/trace");
    add_c_files(&mut build, &trace_dir);

    // Collect all source files from iodbcinst
    let iodbcinst_dir = vendor_dir.join("iodbcinst");
    add_c_files(&mut build, &iodbcinst_dir);

    // Compile the library
    build.compile("iodbc");

    // Link pthread for iODBC
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=iconv");
    }

    println!("cargo:rerun-if-changed=vendor/iODBC");
}

#[cfg(feature = "static")]
#[allow(dead_code)]
fn add_c_files(build: &mut cc::Build, dir: &Path) {
    if !dir.exists() {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "c" {
                    // Skip certain files that shouldn't be compiled
                    let filename = path.file_name().unwrap().to_str().unwrap();

                    // Skip test files, utilities, and GUI components
                    if filename.starts_with("test")
                        || filename == "dltest.c"
                        || filename == "isql.c"
                        || filename == "iusql.c"
                        || filename == "odbcinst.c"
                        || filename == "odbc-config.c"
                        || filename == "slencheck.c"
                    {
                        continue;
                    }

                    build.file(&path);
                }
            }
        }
    }
}

fn print_paths(paths: &str) {
    for path in paths.split(':').filter(|x| !x.is_empty()) {
        println!("cargo:rustc-link-search=native={path}")
    }
}

fn homebrew_library_path() -> Option<String> {
    let output = Command::new("brew").arg("--prefix").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let prefix =
        String::from_utf8(output.stdout).expect("brew --prefix must yield utf8 encoded response");
    // brew returns also a linebreak (`\n`), we want to get rid of that.
    let prefix = prefix.trim();
    let lib_path = prefix.to_owned() + "/lib";
    Some(lib_path)
}
