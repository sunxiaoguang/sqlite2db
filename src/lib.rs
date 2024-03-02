extern crate libc;

use libc::{c_char, c_void};

#[no_mangle]
pub static sqlite3_version: [u8; 10] = *b"3.43.0-rs\n";

pub struct SQLite3 {}

#[no_mangle]
pub extern "C" fn sqlite3_open(_filename: *const c_char, _sqlite3: *mut *mut SQLite3) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_open16(_filename: *const c_char, _sqlite3: *mut *mut SQLite3) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_open_v2(
    _filename: *const c_char,
    _sqlite3: *mut *mut SQLite3,
    _flags: i32,
    _vfs: *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_close(_sqlite3: *mut SQLite3) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_close_v2(_sqlite3: *mut SQLite3) -> i32 {
    0
}

pub struct SQLite3Stmt {}

#[no_mangle]
pub extern "C" fn sqlite3_prepare(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _bytes: i32,
    _stmt: *mut *mut SQLite3Stmt,
    _tail: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_prepare_v2(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _bytes: i32,
    _stmt: *mut *mut SQLite3Stmt,
    _tail: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_prepare_v3(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _bytes: i32,
    _flags: u32,
    _stmt: *mut *mut SQLite3Stmt,
    _tail: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_prepare16(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _bytes: i32,
    _stmt: *mut *mut SQLite3Stmt,
    _tail: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_prepare16_v2(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _bytes: i32,
    _stmt: *mut *mut SQLite3Stmt,
    _tail: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_prepare16_v3(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _bytes: i32,
    _flags: u32,
    _stmt: *mut *mut SQLite3Stmt,
    _tail: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_finalize(_stmt: *mut SQLite3Stmt) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_reset(_stmt: *mut SQLite3Stmt) -> i32 {
    0
}

pub struct SQLite3Value {}

#[no_mangle]
pub extern "C" fn sqlite3_bind_blob(
    _stmt: *mut SQLite3Stmt,
    _idx: i32,
    _data: *const c_void,
    _len: i32,
    _destructor: *const c_void,
) -> i32 {
    0
}
#[no_mangle]
pub extern "C" fn sqlite3_bind_blob64(
    _stmt: *mut SQLite3Stmt,
    _idx: i32,
    _data: *const c_void,
    _len: i64,
    _destructor: *const c_void,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_double(_stmt: *mut SQLite3Stmt, _idx: i32, _data: f64) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_int(_stmt: *mut SQLite3Stmt, _idx: i32, _data: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_int64(_stmt: *mut SQLite3Stmt, _idx: i32, _data: i64) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_null(_stmt: *mut SQLite3Stmt, _idx: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_text(
    _stmt: *mut SQLite3Stmt,
    _idx: i32,
    _text: *const c_char,
    _len: i32,
    _destructor: *const c_void,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_text16(
    _stmt: *mut SQLite3Stmt,
    _idx: i32,
    _text: *const c_void,
    _len: i32,
    _destructor: *const c_void,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_text64(
    _stmt: *mut SQLite3Stmt,
    _idx: i32,
    _text: *const c_char,
    _len: u64,
    _destructor: *const c_void,
    _encoding: u8,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_value(
    _stmt: *mut SQLite3Stmt,
    _idx: i32,
    _value: *const SQLite3Value,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_pointer(
    _stmt: *mut SQLite3Stmt,
    _idx: i32,
    _value: *const c_void,
    _type: *const c_char,
    _destructor: *const c_void,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_zeroblob(_stmt: *mut SQLite3Stmt, _idx: i32, _len: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_bind_zeroblob64(_stmt: *mut SQLite3Stmt, _idx: i32, _len: u64) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_step(_stmt: *mut SQLite3Stmt) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_column_blob(_stmt: *mut SQLite3Stmt, _idx: i32) -> *const c_void {
    std::ptr::null::<c_void>()
}

#[no_mangle]
pub extern "C" fn sqlite3_column_double(_stmt: *mut SQLite3Stmt, _idx: i32) -> f64 {
    0.0
}
#[no_mangle]
pub extern "C" fn sqlite3_column_int(_stmt: *mut SQLite3Stmt, _idx: i32) -> i32 {
    0
}
#[no_mangle]
pub extern "C" fn sqlite3_column_int64(_stmt: *mut SQLite3Stmt, _idx: i32) -> i64 {
    0
}
#[no_mangle]
pub extern "C" fn sqlite3_column_text(_stmt: *mut SQLite3Stmt, _idx: i32) -> *const c_char {
    std::ptr::null::<c_char>()
}
#[no_mangle]
pub extern "C" fn sqlite3_column_text16(_stmt: *mut SQLite3Stmt, _idx: i32) -> *const c_char {
    std::ptr::null::<c_char>()
}

#[no_mangle]
pub extern "C" fn sqlite3_column_value(_stmt: *mut SQLite3Stmt, _idx: i32) -> *const SQLite3Value {
    std::ptr::null::<SQLite3Value>()
}
#[no_mangle]
pub extern "C" fn sqlite3_column_bytes(_stmt: *mut SQLite3Stmt, _idx: i32) -> i32 {
    0
}
#[no_mangle]
pub extern "C" fn sqlite3_column_bytes16(_stmt: *mut SQLite3Stmt, _idx: i32) -> i32 {
    0
}
#[no_mangle]
pub extern "C" fn sqlite3_column_type(_stmt: *mut SQLite3Stmt, _idx: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_exec(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _callback: *const c_void,
    _userdata: *mut c_void,
    _err: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_get_table(
    _db: *mut SQLite3,
    _sql: *const c_char,
    _result: *mut *const *const c_char,
    _rows: *mut i32,
    _columns: *mut i32,
    _err: *mut *const c_char,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn sqlite3_free_table(_result: *const *const c_char) {}
