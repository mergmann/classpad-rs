//! Bindings to the hollyhock-3 SDK

#![no_std]
#![no_main]
#![cfg_attr(feature = "heap-hack", feature(asm_experimental_arch))]

use core::{
    ffi::{CStr, c_char, c_int},
    panic::PanicInfo,
    ptr,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

pub use hollyhock_macros::main;
use hollyhock_sys::symbols::exit;

use crate::{
    err::RT_PANIC,
    rt::{RtState, state},
    sdk::SYMS,
};

pub mod appdef;
#[cfg(feature = "heap-hack")]
mod big_app;
pub mod env;
pub mod err;
pub mod io;
mod mem;
pub mod rt;
pub mod sdk;

extern crate alloc;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if state() == RtState::Running {
        eprintln!("Panic occured: {info}");
    }
    if state() >= RtState::Linked {
        unsafe { (SYMS.Debug_WaitKey)() };
    }
    unsafe { exit(RT_PANIC) }
}

static ARGS: Args = Args::new();

struct Args {
    argc: AtomicUsize,
    argv: AtomicPtr<*const c_char>,
}

impl Args {
    const fn new() -> Self {
        Self {
            argc: AtomicUsize::new(0),
            argv: AtomicPtr::new(ptr::null_mut()),
        }
    }

    unsafe fn set(&self, argc: c_int, argv: *const *const c_char) {
        self.argc.store(argc.max(0) as usize, Ordering::Relaxed);
        self.argv.store(argv.cast_mut(), Ordering::Relaxed);
    }

    fn get(&self) -> (usize, *const *const c_char) {
        (
            self.argc.load(Ordering::Relaxed),
            self.argv.load(Ordering::Relaxed),
        )
    }
}

/// Get the arguments as C strings
pub fn args_c() -> impl ExactSizeIterator<Item = &'static CStr> {
    let (argc, argv) = ARGS.get();
    (0..argc).map(move |i| unsafe { CStr::from_ptr(*argv.add(i)) })
}

/// Get the arguments as Rust string (UTF-8).
/// Panics if an argument is not valid UTF-8
pub fn args() -> impl ExactSizeIterator<Item = &'static str> {
    args_c().enumerate().map(|(i, arg)| match arg.to_str() {
        Ok(arg) => arg,
        Err(err) => panic!("Invalid argument {i}: {err}"),
    })
}

pub mod prelude {
    pub use crate::alloc::{boxed::Box, string::String, vec::Vec};
    pub use crate::{eprint, eprintln, print, println};
}
