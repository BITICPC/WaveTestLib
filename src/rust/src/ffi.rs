use std::cmp::Ordering;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::boxed::Box;

use crate::{Checker, Interactor};
use crate::tokenized::TokenizedRead;
use crate::cmp::compare_floats;


mod convert {
    use std::ffi::c_void;

    use crate::{Checker, Interactor, JudgeReader};

    pub unsafe fn to_raw<T>(value: &mut T) -> *mut c_void {
        value as *mut T
            as *mut c_void
    }

    pub unsafe fn to_checker(raw: *mut c_void) -> &'static mut Checker {
        (raw as *mut Checker).as_mut().unwrap()
    }

    pub unsafe fn to_interactor(raw: *mut c_void) -> &'static mut Interactor {
        (raw as *mut Interactor).as_mut().unwrap()
    }

    pub unsafe fn to_reader(raw: *mut c_void) -> &'static mut JudgeReader {
        (raw as *mut JudgeReader).as_mut().unwrap()
    }
}

use convert::*;


#[no_mangle]
pub unsafe extern fn wave_accept() {
    accept!();
}

#[no_mangle]
pub unsafe extern fn wave_accept_msg(msg: *const c_char) {
    accept!("{}", CStr::from_ptr(msg).to_str().unwrap());
}

#[no_mangle]
pub unsafe extern fn wave_reject(msg: *const c_char) {
    reject!("{}", CStr::from_ptr(msg).to_str().unwrap());
}


#[no_mangle]
pub unsafe extern fn wave_checker_create() -> *mut c_void {
    let checker = Box::new(Checker::new());
    Box::into_raw(checker) as *mut c_void
}

#[no_mangle]
pub unsafe extern fn wave_checker_release(checker: *mut c_void) {
    let checker = Box::from_raw(checker as *mut Checker);
    drop(checker);
}

#[no_mangle]
pub unsafe extern fn wave_checker_get_input_handle(checker: *mut c_void) 
    -> *mut c_void {
    to_raw(to_checker(checker).input())
}

#[no_mangle]
pub unsafe extern fn wave_checker_get_std_answer_handle(checker: *mut c_void) 
    -> *mut c_void {
    to_raw(to_checker(checker).std_answer())
}

#[no_mangle]
pub unsafe extern fn wave_checker_get_user_answer_handle(checker: *mut c_void) 
    -> *mut c_void {
    to_raw(to_checker(checker).user_answer())
}


#[no_mangle]
pub unsafe extern fn wave_interactor_create() -> *mut c_void {
    let interactor = Box::new(Interactor::new());
    Box::into_raw(interactor) as *mut c_void
}

#[no_mangle]
pub unsafe extern fn wave_interactor_release(interactor: *mut c_void) {
    let interactor = Box::from_raw(interactor as *mut Interactor);
    drop(interactor);
}

#[no_mangle]
pub unsafe extern fn wave_interactor_get_input_handle(interactor: *mut c_void) 
    -> *mut c_void {
    to_raw(to_interactor(interactor).input())
}

#[no_mangle]
pub unsafe extern fn wave_interactor_get_answer_handle(interactor: *mut c_void) 
    -> *mut c_void {
    to_raw(to_interactor(interactor).answer())
}

#[no_mangle]
pub unsafe extern fn wave_interactor_get_read_end_handle(
    interactor: *mut c_void) -> *mut c_void {
    to_raw(to_interactor(interactor).read_end())
}

#[no_mangle]
pub unsafe extern fn wave_interactor_get_write_end_handle(
    interactor: *mut c_void) -> *mut c_void {
    to_raw(to_interactor(interactor).write_end())
}


static mut LAST_READ: Option<String> = None;

unsafe fn try_copy_str(s: &str, buffer: *mut c_void, buffer_size: usize) 
    -> usize {
    let len = s.len() + 1;
    if len > buffer_size {
        // Buffer is too small to fit the token.
        return len;
    }

    let mut buffer = buffer as *mut u8;
    let bytes = LAST_READ.take().unwrap().into_bytes();
    for b in bytes {
        *buffer = b;
        buffer = buffer.add(1);
    }

    *buffer = 0;
    len + 1
}

#[no_mangle]
pub unsafe extern fn wave_read_token(
    handle: *mut c_void, buffer: *mut c_void, buffer_size: usize) -> usize {
    let reader = to_reader(handle);
    if LAST_READ.is_none() {
        LAST_READ = reader.inner_reader().read_token();
        if LAST_READ.is_none() {
            return 0;
        }
    }

    try_copy_str(LAST_READ.take().unwrap().as_str(), buffer, buffer_size)    
}

#[no_mangle]
pub unsafe extern fn wave_read_line(
    handle: *mut c_void, buffer: *mut c_void, buffer_size: usize) -> usize {
    let reader = to_reader(handle);
    if LAST_READ.is_none() {
        LAST_READ = reader.inner_reader().read_line();
        if LAST_READ.is_none() {
            return 0;
        }
    }

    try_copy_str(LAST_READ.take().unwrap().as_str(), buffer, buffer_size)
}

#[no_mangle]
pub unsafe extern fn wave_expect_i8(handle: *mut c_void) -> i8 {
    to_reader(handle).expect_type::<i8>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_u8(handle: *mut c_void) -> u8 {
    to_reader(handle).expect_type::<u8>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_i16(handle: *mut c_void) -> i16 {
    to_reader(handle).expect_type::<i16>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_u16(handle: *mut c_void) -> u16 {
    to_reader(handle).expect_type::<u16>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_i32(handle: *mut c_void) -> i32 {
    to_reader(handle).expect_type::<i32>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_u32(handle: *mut c_void) -> u32 {
    to_reader(handle).expect_type::<u32>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_i64(handle: *mut c_void) -> i64 {
    to_reader(handle).expect_type::<i64>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_u64(handle: *mut c_void) -> u64 {
    to_reader(handle).expect_type::<u64>()
}

#[no_mangle]
pub unsafe extern fn wave_expect_token(
    handle: *mut c_void, expected: *mut c_char, ignore_case: i32) {
    to_reader(handle).expect_token(
        CStr::from_ptr(expected).to_str().unwrap(), ignore_case != 0);
}

#[no_mangle]
pub unsafe extern fn wave_expect_signed(handle: *mut c_void, expected: i64) {
    to_reader(handle).expect_eq::<i64, i64>(&expected);
}

#[no_mangle]
pub unsafe extern fn wave_expect_unsigned(handle: *mut c_void, expected: u64) {
    to_reader(handle).expect_eq::<u64, u64>(&expected);
}

#[no_mangle]
pub unsafe extern fn wave_expect_fp(
    handle: *mut c_void, expected: f64, tolerance: f64) {
    to_reader(handle).expect_float_eq(expected, tolerance);
}

#[no_mangle]
pub unsafe extern fn wave_expect_eof(handle: *mut c_void) {
    to_reader(handle).expect_eof();
}


const EQUAL: i32 = 0;
const LESS: i32 = -1;
const GREATER: i32 = 1;
const NOT_COMPARABLE: i32 = std::i32::MAX;

#[no_mangle]
pub unsafe extern fn wave_cmp_fp(actual: f64, expected: f64, tolerance: f64) 
    -> i32 {
    match compare_floats(expected, actual, tolerance) {
        Some(Ordering::Less) => LESS,
        Some(Ordering::Equal) => EQUAL,
        Some(Ordering::Greater) => GREATER,
        None => NOT_COMPARABLE
    }
}

#[no_mangle]
pub unsafe extern fn wave_cmp_str(actual: *const c_char, 
    expected: *const c_char) -> i32 {
    let actual = CStr::from_ptr(actual).to_str().unwrap();
    let expected = CStr::from_ptr(expected).to_str().unwrap();

    match actual.cmp(&expected) {
        Ordering::Less => LESS,
        Ordering::Equal => EQUAL,
        Ordering::Greater => GREATER
    }
}

const TRUE: i32 = 1;
const FALSE: i32 = 0;

#[no_mangle]
pub unsafe extern fn wave_cmp_str_eq(
    actual: *const c_char, expected: *const c_char, ignore_case: bool) -> i32 {
    let actual = CStr::from_ptr(actual).to_str().unwrap();
    let expected = CStr::from_ptr(expected).to_str().unwrap();

    let eq =
        if ignore_case {
            actual.eq_ignore_ascii_case(expected)
        } else {
            actual.eq(expected)
        };
    
    if eq {
        TRUE
    } else {
        FALSE
    }
}
