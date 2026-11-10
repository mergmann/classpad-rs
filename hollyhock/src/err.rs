use core::ffi::c_int;

pub const RT_PANIC: c_int = 0xAF0080;
pub const RT_UNINIT: c_int = 0xAF0050;
pub const RT_SYM_VAR_MISSING: c_int = 0xAF0020;
pub const RT_SYM_VAR_ERROR: c_int = 0xAF0021;
pub const RT_LEN_VAR_MISSING: c_int = 0xAF0030;
pub const RT_LEN_VAR_ERROR: c_int = 0xAF0031;
pub const RT_GUARD_MISMATCH: c_int = 0xAF0040;
