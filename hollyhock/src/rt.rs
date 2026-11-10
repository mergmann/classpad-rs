use core::{
    ffi::{c_char, c_int},
    sync::atomic::{AtomicU8, Ordering},
};

use crate::{ARGS, eprintln, err::RT_UNINIT};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub enum RtState {
    Uninit = 0,
    Linked = 1,
    Running = 2,
    Exit = 3,
}

static STATE: AtomicU8 = AtomicU8::new(RtState::Uninit as u8);

pub fn state() -> RtState {
    match STATE.load(Ordering::Relaxed) {
        0 => RtState::Uninit,
        1 => RtState::Linked,
        2 => RtState::Running,
        _ => RtState::Exit,
    }
}

pub(crate) fn set_state(state: RtState) {
    STATE.store(state as u8, Ordering::Relaxed);
}

pub trait Termination {
    fn report(self) -> c_int;
}
impl Termination for () {
    fn report(self) -> c_int {
        0
    }
}
impl Termination for c_int {
    fn report(self) -> c_int {
        self
    }
}
impl<E: core::fmt::Debug> Termination for Result<(), E> {
    fn report(self) -> c_int {
        match self {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("Error: {e:?}");
                1
            }
        }
    }
}

#[doc(hidden)]
pub unsafe fn start_rt(
    argc: c_int,
    argv: *const *const c_char,
    main: impl FnOnce() -> c_int,
) -> c_int {
    if state() != RtState::Linked {
        return RT_UNINIT;
    }
    unsafe { ARGS.set(argc, argv) };
    set_state(RtState::Running);
    let code = main();
    set_state(RtState::Exit);
    code
}
