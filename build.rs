#[macro_use] extern crate duct;
extern crate cabal_rs;
extern crate walkdir;

use cabal_rs::Cabal;
use walkdir::WalkDir;
use std::env::{current_dir, set_current_dir};
use std::io;
use std::str;
use std::path::Path;

// Each os has a diferent extesion for Dynamic and Static Libraries. This compiles the library
// with the correct ones.

#[cfg(not(target_os = "windows"))]
const STATIC_EXTENSION: &'static str = ".a";

#[cfg(target_os = "windows")]
const STATIC_EXTENSION: &'static str = ".lib";

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const DYLIB_EXTENSION: &'static str = ".so";

#[cfg(target_os = "macos")]
const DYLIB_EXTENSION: &'static str = ".dylib";

#[cfg(target_os = "windows")]
const DYLIB_EXTENSION: &'static str = ".dll";

// This allows the user to choose which version of the Runtime System they want
// to use. By default it is non threaded.
//
// Temporarily this feature is not activated to choose anything else.
#[cfg(not(target_os = "windows"))]
const RTS: &'static str = "libHSrts.a";
#[cfg(target_os = "windows")]
const RTS: &'static str = "libHSrts.lib";

fn main() {

    // Do we even have the source here to compile? If not get it.
    if !Path::new("src/ghc-8.2.2").exists() {
        cmd!("git", "clone", "https://github.com/mgattozzi/ghc-static-8.2.2", "src/ghc-8.2.2")
            .run()
            .expect("ghc-rts-rs: Cloning ghc-static-8.2.2 failed");
    }

    // Has this been compiled? If yes skip.
    if !Path::new(&format!("src/ghc-8.2.2/rts/dist/build/libHSrts{}", STATIC_EXTENSION)).exists() {
        set_current_dir("src/ghc-8.2.2").expect("ghc-rts-rs: chdir to ghc src failed");
        cmd!("./boot").run().expect("ghc-rts-rs: boot failed");
        cmd!("./configure").run().expect("ghc-rts-rs: configure failed");
        cmd!("make").run().expect("ghc-rts-rs: make failed");
        // Traverse the directory to link all of the libs in ghc
        set_current_dir("../..").expect("ghc-rts-rs: chdir to top level failed");
    }

    match link_ghc_libs() {
        Err(e) => panic!("Unable to link ghc_libs: {}", e),
        Ok(_)  => {
            #[cfg(not(target_os = "windows"))]
            println!("cargo:rustc-link-lib=dylib=numa");

            Cabal::src("htest").build().unwrap();
        }
    }
}

fn link_ghc_libs() -> io::Result<()> {

    let full_path = current_dir()?;
    // Go to the libdir for ghc then traverse all the entries
    for entry in WalkDir::new("src/ghc-8.2.2/") {
        let entry = entry?;

        // For each directory in the libdir check it for .so files and
        // link them.
        let file_name = entry.file_name().to_str().unwrap().to_string();

        if  file_name.starts_with("lib") &&
            file_name.ends_with(STATIC_EXTENSION) &&
           !file_name.starts_with("libHSghc-boot-") &&
            file_name.starts_with(RTS) ||
            file_name.starts_with("libHSbase") ||
            file_name.starts_with("libHSinteger-gmp") ||
            file_name.starts_with("libHSghc-prim") ||
            file_name.contains(&format!("libCffi{}", STATIC_EXTENSION)) //hardcoded
        {
            if file_name.ends_with(DYLIB_EXTENSION) ||
               file_name.ends_with(&format!("_p{}", STATIC_EXTENSION)) ||
               file_name.ends_with(".d") ||
               file_name.ends_with(".def")
            {
                continue;
            } else {
                let mut path = entry.path().to_path_buf();
                path.pop();
                println!("cargo:rustc-link-search=native={}/{}"
                         ,full_path.display()
                         ,path.display());
                // Get rid of lib from the file name
                let temp = file_name.split_at(3).1;
                // Get rid of the .so from the file name
                let trimmed = temp.split_at(temp.len() - STATIC_EXTENSION.len()).0;
                println!("cargo:rustc-link-lib=static={}", trimmed);
            }
        }
    }
    Ok(())
}
