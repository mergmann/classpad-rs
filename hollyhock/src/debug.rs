use crate::sdk::SYMS;

pub fn wait_key() {
    unsafe { (SYMS.Debug_WaitKey)() };
}
