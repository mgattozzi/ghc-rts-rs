extern crate ghc_rts;
use ghc_rts::{start, stop};
use std::env;
use std::ffi::CStr;
use std::path::Path;
use std::os::raw::c_char;

extern {
    pub fn triple(x: i64) -> i64;
    pub fn getProgNameStr() -> *const c_char;
}

fn triple_num(x: i64) -> i64 {
    start();
    unsafe { triple(x) }
}

#[test]
fn ffi_test() {
    // TODO Use the threaded Haskell runtime to let tests run safely in
    //      parallel, allowing separate test functions.
    start();
    let prog_name = unsafe { getProgNameStr() };
    assert!(!prog_name.is_null());
    let prog_name_str = unsafe { CStr::from_ptr(prog_name) }.to_str().unwrap();
    let argv0 = env::args().nth(0).unwrap();
    let argv0_file_name = Path::new(&argv0).file_name().unwrap();
    assert_eq!(prog_name_str, argv0_file_name.to_str().unwrap());
    assert_eq!(900, triple_num(300));
    stop();
}
