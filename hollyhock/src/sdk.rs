use core::{ffi::CStr, ptr, slice};

use hollyhock_sys::symbols::{SymbolTable, exit, getenv};

use crate::{
    err::{
        RT_GUARD_MISMATCH, RT_LEN_VAR_ERROR, RT_LEN_VAR_MISSING, RT_SYM_VAR_ERROR,
        RT_SYM_VAR_MISSING,
    },
    rt::{RtState, set_state},
};

pub static mut SYMS: SymbolTable = SymbolTable::new();

/// This needs to run before reserving the static heap space
#[used]
#[unsafe(link_section = ".init_array.00001")]
static LINK_SDK: unsafe extern "C" fn() = link_sdk;

unsafe extern "C" fn link_sdk() {
    let table = unsafe { getenv(c"HHK_SYMBOL_TABLE".as_ptr()) };
    if table.is_null() {
        unsafe { exit(RT_SYM_VAR_MISSING) };
    }
    let table = unsafe { CStr::from_ptr(table) };

    let len = unsafe { getenv(c"HHK_SYMBOL_TABLE_LEN".as_ptr()) };
    if len.is_null() {
        unsafe { exit(RT_LEN_VAR_MISSING) };
    }
    let len = unsafe { CStr::from_ptr(len) };

    let table_addr = match table.to_str() {
        Ok(text) => match usize::from_str_radix(text, 16) {
            Ok(value) => value,
            Err(_) => unsafe { exit(RT_SYM_VAR_ERROR) },
        },
        Err(_) => unsafe { exit(RT_SYM_VAR_ERROR) },
    };
    let len: usize = match len.to_str() {
        Ok(text) => match usize::from_str_radix(text, 16) {
            Ok(value) => value,
            Err(_) => unsafe { exit(RT_LEN_VAR_ERROR) },
        },
        Err(_) => unsafe { exit(RT_LEN_VAR_ERROR) },
    };
    let table_ptr = ptr::with_exposed_provenance::<u8>(table_addr);
    let mapping = unsafe { slice::from_raw_parts(table_ptr, len) };
    let linked = unsafe { (*(&raw mut SYMS)).link(mapping) };
    if !linked {
        unsafe { exit(RT_GUARD_MISMATCH) };
    }
    set_state(RtState::Linked);
}
