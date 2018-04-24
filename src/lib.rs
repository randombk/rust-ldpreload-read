/*
 * Copyright (C) 2015-2017 DLP Networks
 * Copyright (C) 2015-2017 David Jia Wei Li
 * 0a1c6fb2-6e63-4a14-8c8b-47069eca245a
 *
 * LD_PRELOAD File Redirect Hook
 */

extern crate libc;

#[macro_use]
extern crate redhook;

use std::ffi::{CStr, CString};
use libc::{c_char};

static REDIRECT_ORIG_PATH: &str = "file.txt";
static REDIRECT_TARGET_PATH: &str = "file2.txt";

hook! {
    unsafe fn fopen(pathname: *const c_char, mode: *mut c_char) -> usize => my_fopen {
        if CStr::from_ptr(pathname).to_str() == Ok(REDIRECT_ORIG_PATH) {
            println!("Redirected fopen(\"{}\") to fopen(\"{}\")", REDIRECT_ORIG_PATH, REDIRECT_TARGET_PATH);

            let new_path = CString::new(REDIRECT_TARGET_PATH).unwrap();
            real!(fopen)(new_path.as_ptr(), mode)
        } else {
            real!(fopen)(pathname, mode)
        }
    }
}

hook! {
    unsafe fn fopen64(pathname: *const c_char, mode: *mut c_char) -> usize => my_fopen64 {
        if CStr::from_ptr(pathname).to_str() == Ok(REDIRECT_ORIG_PATH) {
            println!("Redirected fopen64(\"{}\") to fopen64(\"{}\")", REDIRECT_ORIG_PATH, REDIRECT_TARGET_PATH);

            let new_path = CString::new(REDIRECT_TARGET_PATH).unwrap();
            real!(fopen64)(new_path.as_ptr(), mode)
        } else {
            real!(fopen64)(pathname, mode)
        }
    }
}

hook! {
    unsafe fn open(pathname: *const c_char, mode: usize) -> usize => my_open {
        if CStr::from_ptr(pathname).to_str() == Ok(REDIRECT_ORIG_PATH) {
            println!("Redirected open(\"{}\") to open(\"{}\")", REDIRECT_ORIG_PATH, REDIRECT_TARGET_PATH);

            let new_path = CString::new(REDIRECT_TARGET_PATH).unwrap();
            real!(open)(new_path.as_ptr(), mode)
        } else {
            real!(open)(pathname, mode)
        }
    }
}

hook! {
    unsafe fn open64(pathname: *const c_char, mode: usize) -> usize => my_open64 {
        if CStr::from_ptr(pathname).to_str() == Ok(REDIRECT_ORIG_PATH) {
            println!("Redirected open64(\"{}\") to open64(\"{}\")", REDIRECT_ORIG_PATH, REDIRECT_TARGET_PATH);

            let new_path = CString::new(REDIRECT_TARGET_PATH).unwrap();
            real!(open64)(new_path.as_ptr(), mode)
        } else {
            real!(open64)(pathname, mode)
        }
    }
}

// 0a1c6fb2-6e63-4a14-8c8b-47069eca245a
