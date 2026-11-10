use core::{iter, mem::DropGuard, str::Utf8Error};

use alloc::{borrow::ToOwned, boxed::Box, string::String, vec, vec::Vec};
use embedded_io::{BufRead, ErrorType, Read};

struct Buffer {
    data: Box<[u8]>,
    pos: usize,
    end: usize,
}

impl Buffer {
    pub fn new(cap: usize) -> Self {
        Self {
            data: vec![0; cap].into_boxed_slice(),
            pos: 0,
            end: 0,
        }
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        &mut self.data[self.pos..self.end]
    }

    pub fn consume(&mut self, amt: usize) {
        self.pos += amt;
    }
}

const DEFAULT_BUF_SIZE: usize = 512;

pub struct BufReader<R: Read> {
    inner: R,
    buffer: Buffer,
}

impl<R: Read> BufReader<R> {
    pub fn new(inner: R) -> Self {
        Self::with_capacity(inner, DEFAULT_BUF_SIZE)
    }

    pub fn with_capacity(inner: R, cap: usize) -> Self {
        Self {
            inner,
            buffer: Buffer::new(cap),
        }
    }
}

impl<R: Read> ErrorType for BufReader<R> {
    type Error = R::Error;
}

impl<R: Read> Read for BufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let src = self.fill_buf()?;
        let count = buf.len().min(src.len());
        buf[..count].copy_from_slice(&src[..count]);
        self.consume(count);
        Ok(count)
    }
}

impl<R: Read> BufRead for BufReader<R> {
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        let mut buf = self.buffer.get_mut();
        if buf.is_empty() {
            self.buffer.pos = 0;
            self.buffer.end = self.inner.read(&mut self.buffer.data)?;
            buf = self.buffer.get_mut();
        }
        Ok(buf)
    }

    fn consume(&mut self, amt: usize) {
        self.buffer.consume(amt);
    }
}

unsafe fn append_to_string<F, E: From<Utf8Error>>(buf: &mut String, f: F) -> Result<usize, E>
where
    F: FnOnce(&mut Vec<u8>) -> Result<usize, E>,
{
    let len_original = buf.len();
    // SAFETY: invalid UTF-8 discarded before return or unwind
    let buf_vec = unsafe { buf.as_mut_vec() };
    let mut g = DropGuard::new((len_original, buf_vec), |(len, buf)| unsafe {
        buf.set_len(len);
    });
    let ret = f(g.1);

    // SAFETY: the caller promises to only append data to `buf`
    let appended = unsafe { g.1.get_unchecked(g.0..) };
    if let Err(err) = str::from_utf8(appended) {
        ret.and_then(|_| Err(err.into()))
    } else {
        g.0 = g.1.len();
        ret
    }
}

pub trait BufReadExt: BufRead {
    /// Appends bytes including the terminator into the buffer
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize, Self::Error> {
        let mut count = 0;
        loop {
            let src = self.fill_buf()?;
            if src.is_empty() {
                break;
            }
            match memchr::memchr(byte, src) {
                Some(pos) => {
                    buf.extend_from_slice(&src[..=pos]);
                    self.consume(pos + 1);
                    count += pos + 1;

                    break;
                }
                None => {
                    let len = src.len();
                    buf.extend_from_slice(src);
                    self.consume(len);
                    count += len;
                }
            }
        }
        Ok(count)
    }

    /// Appends a line including the newline terminator to the buffer
    fn read_line(&mut self, buf: &mut String) -> Result<usize, Self::Error>
    where
        Self::Error: From<Utf8Error>,
    {
        unsafe { append_to_string(buf, |v| self.read_until(b'\n', v)) }
    }

    /// Split the stream. Terminators are not included
    fn split(&mut self, byte: u8) -> impl Iterator<Item = Result<Vec<u8>, Self::Error>>
    where
        Self::Error: From<Utf8Error>,
    {
        let mut buf = Vec::new();
        iter::from_fn(move || {
            let result = match self.read_until(byte, &mut buf) {
                Ok(0) => None,
                Ok(_) => Some(Ok(buf.trim_suffix(&[byte]).to_owned())),
                Err(err) => Some(Err(err)),
            };
            buf.clear();
            result
        })
    }

    /// Split the stream by lines. Terminators are not included
    fn lines(&mut self) -> impl Iterator<Item = Result<String, Self::Error>>
    where
        Self::Error: From<Utf8Error>,
    {
        let mut buf = String::new();
        iter::from_fn(move || {
            let result = match self.read_line(&mut buf) {
                Ok(0) => None,
                Ok(_) => Some(Ok(buf.trim_suffix('\n').to_owned())),
                Err(err) => Some(Err(err)),
            };
            buf.clear();
            result
        })
    }
}
impl<T: BufRead> BufReadExt for T {}
