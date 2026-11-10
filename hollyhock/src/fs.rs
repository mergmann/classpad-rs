use core::{error, fmt};

use alloc::{borrow::ToOwned, string::String};
use embedded_io::ErrorKind;
use encoding_rs::{DecoderResult, EncoderResult, SHIFT_JIS};
use hollyhock_sys::symbols::{
    BFile_Error::{self, BFILE_EEOF, BFILE_EINVAL},
    File_Error, File_FindInfo, File_FindInfoType,
};

use crate::sdk::SYMS;

#[derive(Debug)]
pub struct FugueError(pub i32);

impl FugueError {
    pub fn name(&self) -> &'static str {
        match self.0 {
            File_Error::FILE_ENOMEM => "Out of memory",
            File_Error::FILE_EINVAL => "Invalid input",
            File_Error::FILE_EDEVFAIL => "Input/Output error",
            File_Error::FILE_EMOUNTED => "Already mounted",
            File_Error::FILE_EACCES => "Access",
            File_Error::FILE_EBADFSID => "Bad fs",
            File_Error::FILE_ENOVOLUME => "Volume not found",
            File_Error::FILE_ENOPATH => "Path not found",
            File_Error::FILE_EEXIST => "Already exists",
            File_Error::FILE_ENAMETOOLONG => "Name too long",
            File_Error::FILE_EOUTOFBOUND => "Out of bounds",
            File_Error::FILE_EUNFORMAT => "Unformatted",
            File_Error::FILE_ENOSPC => "No space",
            File_Error::FILE_ENOENT => "Entry not found",
            File_Error::FILE_EISDIRECTORY => "Is a directory",
            File_Error::FILE_ESHARE => "Sharing violation",
            File_Error::FILE_EMFILE => "Too many open files",
            File_Error::FILE_EBADF => "Bad file descriptor",
            File_Error::FILE_EEOF => "End of file",
            File_Error::FILE_ENOTEMPTY => "Directory not empty",
            File_Error::FILE_ECLUSTERSIZEMISMATCH => "Cluster size mismatch",
            File_Error::FILE_ESYSTEM => "System",
            _ => "Unknown",
        }
    }
}

impl error::Error for FugueError {}
impl fmt::Display for FugueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.0)
    }
}

impl embedded_io::Error for FugueError {
    fn kind(&self) -> ErrorKind {
        match self.0 {
            File_Error::FILE_ENOMEM => ErrorKind::OutOfMemory,
            File_Error::FILE_EINVAL => ErrorKind::InvalidInput,
            File_Error::FILE_EDEVFAIL => ErrorKind::Other,
            File_Error::FILE_EMOUNTED => ErrorKind::Other,
            File_Error::FILE_EACCES => ErrorKind::PermissionDenied,
            File_Error::FILE_EBADFSID => ErrorKind::InvalidInput,
            File_Error::FILE_ENOVOLUME => ErrorKind::NotFound,
            File_Error::FILE_ENOPATH => ErrorKind::NotFound,
            File_Error::FILE_EEXIST => ErrorKind::AlreadyExists,
            File_Error::FILE_ENAMETOOLONG => ErrorKind::InvalidInput,
            File_Error::FILE_EOUTOFBOUND => ErrorKind::InvalidInput,
            File_Error::FILE_EUNFORMAT => ErrorKind::InvalidData,
            File_Error::FILE_ENOSPC => ErrorKind::Other,
            File_Error::FILE_ENOENT => ErrorKind::NotFound,
            File_Error::FILE_EISDIRECTORY => ErrorKind::InvalidInput,
            File_Error::FILE_ESHARE => ErrorKind::PermissionDenied,
            File_Error::FILE_EMFILE => ErrorKind::OutOfMemory,
            File_Error::FILE_EBADF => ErrorKind::InvalidInput,
            File_Error::FILE_EEOF => ErrorKind::WriteZero,
            File_Error::FILE_ENOTEMPTY => ErrorKind::Other,
            File_Error::FILE_ECLUSTERSIZEMISMATCH => ErrorKind::InvalidData,
            File_Error::FILE_ESYSTEM => ErrorKind::Other,
            _ => ErrorKind::Other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BFileError(pub i32);

impl BFileError {
    pub fn name(&self) -> &'static str {
        match self.0 {
            BFile_Error::BFILE_ENOENT => "Entry not found",
            BFile_Error::BFILE_EINVAL => "Invalid input",
            BFile_Error::BFILE_ENOPATH => "Path not found",
            BFile_Error::BFILE_ENOSPC => "No space",
            BFile_Error::BFILE_EDATA => "Invalid data",
            BFile_Error::BFILE_ENOMEM => "Out of memory",
            BFile_Error::BFILE_EACCESS => "Access error",
            BFile_Error::BFILE_EEXIST => "Already exists",
            BFile_Error::BFILE_EEOF => "End of file",
            BFile_Error::BFILE_ENOVOLUME => "Volume not found",
            BFile_Error::BFILE_EDEVFAIL => "Input/Output error",
            _ => "Unknown",
        }
    }
}

impl error::Error for BFileError {}
impl fmt::Display for BFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.0)
    }
}

impl embedded_io::Error for BFileError {
    fn kind(&self) -> ErrorKind {
        match self.0 {
            BFile_Error::BFILE_ENOENT => ErrorKind::NotFound,
            BFile_Error::BFILE_EINVAL => ErrorKind::InvalidInput,
            BFile_Error::BFILE_ENOPATH => ErrorKind::NotFound,
            BFile_Error::BFILE_ENOSPC => ErrorKind::Other,
            BFile_Error::BFILE_EDATA => ErrorKind::InvalidData,
            BFile_Error::BFILE_ENOMEM => ErrorKind::OutOfMemory,
            BFile_Error::BFILE_EACCESS => ErrorKind::PermissionDenied,
            BFile_Error::BFILE_EEXIST => ErrorKind::AlreadyExists,
            BFile_Error::BFILE_EEOF => ErrorKind::WriteZero,
            BFile_Error::BFILE_ENOVOLUME => ErrorKind::NotFound,
            BFile_Error::BFILE_EDEVFAIL => ErrorKind::Other,
            _ => ErrorKind::Other,
        }
    }
}

pub fn check_fugue(ret: i32) -> Result<u32, FugueError> {
    match ret {
        0.. => Ok(ret as u32),
        _ => Err(FugueError(ret)),
    }
}

pub fn check_bfile(ret: i32) -> Result<u32, BFileError> {
    match ret {
        0.. => Ok(ret as u32),
        _ => Err(BFileError(ret)),
    }
}

#[derive(Clone, Copy)]
pub struct DirEntry {
    name: [u16; 0x10A],
    info: File_FindInfo,
}

impl Default for DirEntry {
    fn default() -> Self {
        Self {
            name: [0; 0x10A],
            info: Default::default(),
        }
    }
}

impl DirEntry {
    /// The entry name as a 16-bit wcstr
    pub fn name_w(&self) -> &[u16] {
        let end = self
            .name
            .iter()
            .position(|x| *x == 0)
            .unwrap_or(self.name.len());
        &self.name[..end]
    }

    pub fn name_to_string(&self) -> Option<String> {
        let mut buf = [0; 512];
        from_wcstr(self.name_w(), &mut buf)
            .map(|s| s.to_owned())
            .ok()
    }

    pub fn is_file(&self) -> bool {
        self.info.type_ == File_FindInfoType::FILE
    }

    pub fn is_dir(&self) -> bool {
        self.info.type_ == File_FindInfoType::DIRECTORY
    }

    pub fn size(&self) -> u32 {
        self.info.size
    }
}

pub struct ReadDir {
    handle: i32,
    entry: DirEntry,
    error: Option<BFileError>,
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry, BFileError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.error {
            None => Some(Ok(self.entry)),
            Some(BFileError(BFILE_EEOF)) => None,
            Some(err) => Some(Err(err)),
        };
        self.error = check_bfile(unsafe {
            (SYMS.File_FindNext)(
                self.handle,
                self.entry.name.as_mut_ptr(),
                &mut self.entry.info,
            )
        })
        .err();
        result
    }
}

impl Drop for ReadDir {
    fn drop(&mut self) {
        unsafe { (SYMS.File_FindClose)(self.handle) };
    }
}

/// Iterate over entries in a directory
pub fn read_dir(path: &str) -> Result<ReadDir, BFileError> {
    let mut wcstr = [0; 0x10A];
    to_wcstr(path, &mut wcstr).map_err(|_| BFileError(BFILE_EINVAL))?;

    let mut handle: i32 = 0;
    let mut name = [0_u16; 0x10A];
    let mut info = File_FindInfo::default();
    check_bfile(unsafe {
        (SYMS.File_FindFirst)(wcstr.as_ptr(), &mut handle, name.as_mut_ptr(), &mut info)
    })?;
    Ok(ReadDir {
        handle,
        entry: DirEntry { name, info },
        error: None,
    })
}

pub fn to_wcstr(s: &str, dst: &mut [u16]) -> Result<usize, ()> {
    let mut enc = SHIFT_JIS.new_encoder();
    let mut n = 0;
    for ch in s.chars() {
        let mut utf8 = [0u8; 4];
        let mut sjis = [0u8; 2];
        let (res, _, written) =
            enc.encode_from_utf8_without_replacement(ch.encode_utf8(&mut utf8), &mut sjis, false);
        if res != EncoderResult::InputEmpty {
            return Err(());
        }
        let unit = match written {
            1 => sjis[0] as u16,
            2 => (sjis[0] as u16) << 8 | sjis[1] as u16,
            _ => return Err(()),
        };
        *dst.get_mut(n).ok_or(())? = unit;
        n += 1;
    }
    *dst.get_mut(n).ok_or(())? = 0;
    Ok(n)
}

pub fn from_wcstr<'a>(src: &[u16], dst: &'a mut [u8]) -> Result<&'a str, ()> {
    let mut n = 0;
    for &unit in src.iter().take_while(|&&u| u != 0) {
        let pair = unit.to_be_bytes();
        let bytes: &[u8] = match pair[0] {
            0 => &pair[1..],
            0x81..=0x9F | 0xE0..=0xFC => &pair,
            _ => return Err(()),
        };
        let mut dec = SHIFT_JIS.new_decoder_without_bom_handling();
        let out = dst.get_mut(n..).ok_or(())?;
        let (res, read, written) = dec.decode_to_utf8_without_replacement(bytes, out, true);
        match res {
            DecoderResult::InputEmpty if read == bytes.len() => n += written,
            DecoderResult::OutputFull => return Err(()),
            _ => return Err(()),
        }
    }
    str::from_utf8(&dst[..n]).map_err(|_| ())
}
