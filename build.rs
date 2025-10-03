use std::process::Command;

fn main() {
    if std::env::var("CARGO_FEATURE_STATIC").is_ok() {
        if cfg!(target_os = "windows") {
            panic!("odbc-sys does not currently support static linking on windows");
        }

        // When the static feature is enabled, compile the ODBC driver manager from source
        compile_odbc_from_source();
    } else {
        // Dynamic linking (existing behavior)
        link_dynamic();
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

fn link_dynamic() {
    // This function handles the existing dynamic linking behavior
    // (no changes to original logic except for static linking removal)
}

fn compile_odbc_from_source() {
    let use_iodbc = std::env::var("CARGO_FEATURE_IODBC").is_ok();
    
    if use_iodbc {
        compile_iodbc();
    } else {
        compile_unixodbc();
    }
}

fn compile_unixodbc() {
    let mut build = cc::Build::new();
    
    let vendor_dir = std::path::Path::new("vendor/unixODBC");
    
    // Add include paths
    build.include(vendor_dir.join("include"));
    build.include(vendor_dir);
    
    // Add compiler flags
    build.flag_if_supported("-DHAVE_CONFIG_H");
    build.flag_if_supported("-DHAVE_STDLIB_H");
    build.flag_if_supported("-DHAVE_STRING_H");
    build.flag_if_supported("-DHAVE_UNISTD_H");
    build.flag_if_supported("-DHAVE_PWD_H");
    build.flag_if_supported("-DHAVE_SYS_TYPES_H");
    build.flag_if_supported("-DHAVE_LONG_LONG");
    build.flag_if_supported("-DSIZEOF_LONG_INT=8");
    build.flag_if_supported("-fPIC");
    
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
    
    println!("cargo:rerun-if-changed=vendor/unixODBC");
}

fn compile_iodbc() {
    let mut build = cc::Build::new();
    
    let vendor_dir = std::path::Path::new("vendor/iODBC");
    
    // Add include paths
    build.include(vendor_dir.join("include"));
    build.include(vendor_dir);
    build.include(vendor_dir.join("iodbc"));
    build.include(vendor_dir.join("iodbcinst"));
    
    // Add compiler flags
    build.flag_if_supported("-DHAVE_CONFIG_H");
    build.flag_if_supported("-DWITH_PTHREADS");
    build.flag_if_supported("-D_REENTRANT");
    build.flag_if_supported("-DHAVE_STDLIB_H");
    build.flag_if_supported("-DHAVE_STRING_H");
    build.flag_if_supported("-DHAVE_UNISTD_H");
    build.flag_if_supported("-fPIC");
    
    // Collect all source files from iodbc
    let iodbc_dir = vendor_dir.join("iodbc");
    add_c_files(&mut build, &iodbc_dir);
    
    // Collect all source files from iodbcinst
    let iodbcinst_dir = vendor_dir.join("iodbcinst");
    add_c_files(&mut build, &iodbcinst_dir);
    
    // Compile the library
    build.compile("iodbc");
    
    // Link pthread for iODBC
    println!("cargo:rustc-link-lib=pthread");
    
    println!("cargo:rerun-if-changed=vendor/iODBC");
}

fn add_c_files(build: &mut cc::Build, dir: &std::path::Path) {
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
                    
                    // Skip test files and utilities
                    if filename.starts_with("test") || 
                       filename == "dltest.c" ||
                       filename.contains("main.c") {
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
