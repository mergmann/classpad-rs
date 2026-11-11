use core::{error, ffi::c_int, fmt};

use embedded_io::{ErrorKind, ErrorType, Read, Write};
use hollyhock_sys::symbols::{fflush, read, write};

#[derive(Debug)]
pub struct StdioError;

impl error::Error for StdioError {}
impl fmt::Display for StdioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "stdio error")
    }
}

impl embedded_io::Error for StdioError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

pub struct Std<const FD: c_int>;

impl<const FD: c_int> ErrorType for Std<FD> {
    type Error = StdioError;
}

impl<const FD: c_int> Read for Std<FD> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let size = unsafe { read(FD, buf.as_mut_ptr(), buf.len()) };
        match size {
            0.. => Ok(size as usize),
            _ => Err(StdioError),
        }
    }
}

impl<const FD: c_int> Write for Std<FD> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let size = unsafe { write(FD, buf.as_ptr(), buf.len()) };
        match size {
            0.. => Ok(size as usize),
            _ => Err(StdioError),
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let ok = unsafe { fflush(FD) };
        match ok {
            0.. => Ok(()),
            _ => Err(StdioError),
        }
    }
}

pub type Stdin = Std<0>;
pub type Stdout = Std<1>;
pub type Stderr = Std<2>;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::io::__private::embedded_io::Write;
        let _ = write!($crate::io::stdio::Stdout {}, $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use $crate::io::__private::embedded_io::Write;
        let _ = writeln!($crate::io::stdio::Stdout {}, $($arg)*);
    }};
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        use $crate::io::__private::embedded_io::Write;
        let _ = write!($crate::io::stdio::Stderr {}, $($arg)*);
    }};
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {{
        use $crate::io::__private::embedded_io::Write;
        let _ = writeln!($crate::io::stdio::Stderr {}, $($arg)*);
    }};
}
