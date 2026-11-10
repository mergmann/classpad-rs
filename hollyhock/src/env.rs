use core::{ffi::CStr, iter};

use hollyhock_sys::symbols::{environ, getenv};

/// Get an environment variable as a C string
pub fn var_c(name: &CStr) -> Option<&CStr> {
    let ptr = unsafe { getenv(name.as_ptr()) };
    if !ptr.is_null() {
        Some(unsafe { CStr::from_ptr(ptr) })
    } else {
        None
    }
}

/// Get an environment variable as a Rust string
/// Panics if the value is invalid UTF-8
pub fn var(name: &CStr) -> Option<&str> {
    var_c(name).map(|val| match val.to_str() {
        Ok(val) => val,
        Err(err) => panic!("Invalid var {name:?}: {err}"),
    })
}

fn raw_vars_c() -> impl Iterator<Item = &'static CStr> {
    let mut ptr = unsafe { environ };
    iter::from_fn(move || {
        if !ptr.is_null() {
            let name = unsafe { CStr::from_ptr(*ptr) };
            ptr = unsafe { ptr.add(1) };
            Some(name)
        } else {
            None
        }
    })
}

/// Iterate environment variables as bytes
pub fn vars_b() -> impl Iterator<Item = (&'static [u8], &'static [u8])> {
    raw_vars_c().flat_map(|var| {
        let bytes = var.to_bytes();
        bytes[1..]
            .iter()
            .position(|c| *c == b'=')
            .map(|idx| (&bytes[..idx + 1], &bytes[idx + 2..]))
    })
}

/// Iterate environment variables as Rust strings
pub fn vars() -> impl Iterator<Item = (&'static str, &'static str)> {
    vars_b().map(
        |(name, val)| match (str::from_utf8(name), str::from_utf8(val)) {
            (Ok(name), Ok(val)) => (name, val),
            (Err(err), _) => panic!("Invalid name: {err}"),
            (Ok(_), Err(err)) => panic!("Invalid var {name:?}: {err}"),
        },
    )
}
