use core::{
    ffi::{CStr, c_char, c_int, c_void},
    mem, ptr,
};

unsafe extern "C" {
    pub fn exit(code: c_int) -> !;

    pub static environ: *const *const c_char;
    pub fn getenv(name: *const c_char) -> *const c_char;

    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);

    pub fn read(fd: c_int, buf: *mut u8, len: usize) -> isize;
    pub fn write(fd: c_int, buf: *const u8, len: usize) -> isize;
    pub fn fflush(fd: c_int) -> c_int;
}

const SAFE_GUARD_ADDR: usize = 0x814fffe0;
const SAFE_GUARD_SIZE: usize = 16;

const ADDR_SIZE: usize = mem::size_of::<*const c_void>();

#[allow(non_snake_case)]
pub mod File_Error {
    pub const FILE_OK: i32 = 0;
    pub const FILE_ENOMEM: i32 = -1;
    pub const FILE_EINVAL: i32 = -2;
    pub const FILE_EDEVFAIL: i32 = -3;
    pub const FILE_EMOUNTED: i32 = -4;
    pub const FILE_EACCES: i32 = -5;
    pub const FILE_EBADFSID: i32 = -6;
    pub const FILE_ENOVOLUME: i32 = -7;
    pub const FILE_ENOPATH: i32 = -8;
    pub const FILE_EEXIST: i32 = -9;
    pub const FILE_ENAMETOOLONG: i32 = -10;
    pub const FILE_EOUTOFBOUND: i32 = -11;
    pub const FILE_EUNFORMAT: i32 = -12;
    pub const FILE_ENOSPC: i32 = -13;
    pub const FILE_ENOENT: i32 = -14;
    pub const FILE_EISDIRECTORY: i32 = -15;
    pub const FILE_ESHARE: i32 = -16;
    pub const FILE_EMFILE: i32 = -17;
    pub const FILE_EBADF: i32 = -18;
    pub const FILE_EEOF: i32 = -19;
    pub const FILE_ENOTEMPTY: i32 = -20;
    pub const FILE_ECLUSTERSIZEMISMATCH: i32 = -40;
    pub const FILE_ESYSTEM: i32 = -99;
}

#[allow(non_snake_case)]
pub mod BFile_Error {
    pub const BFILE_OK: i32 = 0;
    pub const BFILE_ENOENT: i32 = -1;
    pub const BFILE_EINVAL: i32 = -2;
    pub const BFILE_ENOPATH: i32 = -3;
    pub const BFILE_ENOSPC: i32 = -4;
    pub const BFILE_EDATA: i32 = -6;
    pub const BFILE_ENOMEM: i32 = -7;
    pub const BFILE_EACCESS: i32 = -8;
    pub const BFILE_EEXIST: i32 = -13;
    pub const BFILE_EEOF: i32 = -16;
    pub const BFILE_ENOVOLUME: i32 = -23;
    pub const BFILE_EDEVFAIL: i32 = -31;
}

/// Set the file offset to offset.
pub const FILE_SEEK_SET: i32 = 0;
/// Set the file offset to the current position, plus offset bytes.
pub const FILE_SEEK_CUR: i32 = 1;
///Set the file offset to the end of the file, plus offset bytes.
pub const FILE_SEEK_END: i32 = 2;

/// Open the file as readable.
pub const FILE_OPEN_READ: i32 = 1 << 0;
/// Open the file as writable.
pub const FILE_OPEN_WRITE: i32 = 1 << 1;
/// Create the file, if it does not already exist.
pub const FILE_OPEN_CREATE: i32 = 1 << 2;
/// Opens the file with the file offset set to the end of the file.
pub const FILE_OPEN_APPEND: i32 = 1 << 4;

#[allow(non_camel_case_types, non_snake_case)]
pub struct File_StatInfo {
    pub unknown1: u32,
    pub fileSize: u32,
    pub creationDate: u16,
    pub creationTime: u16,
    pub lastModifiedDate: u16,
    pub lastModifiedTime: u16,
    pub unknown2: u16,
    pub lastAccessedDate: u16,
}

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct File_FindInfoType(pub u16);

impl File_FindInfoType {
    pub const FILE: Self = Self(0x1);
    pub const DIRECTORY: Self = Self(0x5);
}

#[repr(C, align(4))]
#[derive(Default, Clone, Copy)]
pub struct File_FindInfo {
    pub struct_size: u32,
    pub type_: File_FindInfoType,
    pub flags: u16,
    pub size: u32,
    pub unknown2: [u8; 8],
}

unsafe fn strn_eq_len(a: *const u8, b: *const u8, n: usize) -> (bool, usize) {
    for i in 0..n {
        let (x, y) = unsafe { (*a.add(i), *b.add(i)) };
        if x != y {
            return (false, i);
        }
        if x == 0 {
            return (true, i);
        }
    }
    (true, n)
}

macro_rules! make_symbol_table {
    (@priv symbol ty, fn($($arg:ident : $ty:ty),*, ...) $(-> $ret:ty)?) => {
        unsafe extern "C" fn($($arg: $ty,)* ...) $(-> $ret)?
    };
    (@priv symbol ty, fn($($arg:ident : $ty:ty),* $(,)?) $(-> $ret:ty)?) => {
        unsafe extern "C" fn($($arg: $ty,)*) $(-> $ret)?
    };
    (@priv symbol ty, $ty:ty) => {
        $ty
    };
    (@priv symbol def, $sym:ident, fn($($arg:ident : $ty:ty),*, ...) $(-> $ret:ty)?) => {
        {
            unsafe extern "C" fn unlinked($(_ : $ty,)* _: ...) $(-> $ret)? {
                panic!("Symbol {} not linked", stringify!($sym))
            }
            unlinked
        }
    };
    (@priv symbol def, $sym:ident, fn($($arg:ident : $ty:ty),*$(,)?) $(-> $ret:ty)?) => {
        {
            unsafe extern "C" fn unlinked($(_ : $ty,)*) $(-> $ret)? {
                panic!("Symbol {} not linked", stringify!($sym))
            }
            unlinked
        }
    };
    (@priv symbol def, $sym:ident, *const $ty:ty) => {
        core::ptr::null()
    };
    (@priv symbol def, $sym:ident, *mut $ty:ty) => {
        core::ptr::null_mut()
    };
    (
        struct $table:ident;
        $( $sym:ident : ($($tt:tt)*); )*
    ) => {
        $(
            #[allow(non_camel_case_types)]
            pub type $sym = make_symbol_table!(@priv symbol ty, $($tt)*);
        )*

        // `repr(C)` so it can be treated as [<addr>; N]
        #[repr(C)]
        pub struct $table {
            $(
                #[allow(non_snake_case)]
                pub $sym: $sym,
            )*
        }

        const _: () = {
            $( assert!(mem::size_of::<$sym>() == ADDR_SIZE); )*
            assert!(mem::size_of::<$table>() == $table::COUNT * ADDR_SIZE);
        };

        impl $table {
            /// Field names in declaration order, each terminated by a NUL.
            const NAMES: &'static [u8] = concat!($(stringify!($sym), "\0"),*).as_bytes();
            const COUNT: usize = [$(stringify!($sym)),*].len();

            /// Linear search so it doesn't produce 32KB of code lol
            unsafe fn assign_symbol(&mut self, name: &[u8], addr: *const u8) {
                let slots = (&raw mut *self).cast::<*const ()>();
                let mut names = Self::NAMES;
                let mut index = 0;
                while let Some(end) = names.iter().position(|&b| b == 0) {
                    if &names[..end] == name {
                        let value = unsafe { addr.cast::<*const ()>().read_unaligned() };
                        unsafe { slots.add(index).write(value) };
                        return;
                    }
                    names = &names[end + 1..];
                    index += 1;
                }
            }

            pub const fn new() -> Self {
                Self {
                    $( $sym: make_symbol_table!(@priv symbol def, $sym, $($tt)*), )*
                }
            }
        }
    };
}

make_symbol_table! {
    struct SymbolTable;

    // debug
    DEBUG_FONTBASE: (*mut u16);
    Debug_GetCursorPosition: (fn(x: *mut u32, y: *mut u32));
    Debug_Printf: (fn(x: u32, y: u32, invert: bool, zero: i32, format: *const c_char, ...));
    Debug_PrintNumberHex_Byte: (fn(value: u8, x: u32, y: u32));
    Debug_PrintNumberHex_Dword: (fn(value: u32, x: u32, y: u32));
    Debug_PrintNumberHex_Nibble: (fn(value: u8, x: u32, y: u32));
    Debug_PrintNumberHex_Word: (fn(value: u16, x: u32, y: u32));
    Debug_PrintString: (fn(string: *const c_char, invert: bool) -> bool);
    Debug_SetCursorPosition: (fn(x: u32, y: u32) -> i32);
    Debug_WaitKey: (fn() -> i32);

    // file
    File_Close: (fn(fd: i32) -> i32);
    File_FindClose: (fn(find_handle: i32) -> i32);
    File_FindNext: (fn(find_handle: i32, name: *mut u16, find_info_buf: *mut File_FindInfo) -> i32);
    File_FindFirst: (fn(path: *const u16, find_handle: *mut i32, name: *mut u16, find_info_buf: *mut File_FindInfo) -> i32);
    File_Fstat: (fn(fd: i32, buf: *mut File_Stat) -> i32);
    File_GetAddr: (fn(fd: i32, offset: i32, addr: *mut *const c_void) -> i32);
    File_Lseek: (fn(fd: i32, offset: i32, whence: i32) -> i32);
    File_Mkdir: (fn(path: *const c_char) -> i32);
    File_Open: (fn(path: *const c_char, flags: i32) -> i32);
    File_Read: (fn(fd: i32, buf: *mut c_void, count: i32) -> i32);
    File_Remove: (fn(path: *const c_char) -> i32);
    File_Rename: (fn(old_path: *const c_char, new_path: *const c_char) -> i32);
    File_Stat: (fn(path: *const c_char, buf: *mut File_StatInfo) -> i32);
    File_Write: (fn(fd: i32, buf: *const c_void, count: i32) -> i32);
    File_Flush: (fn(fd: i32) -> i32);

    // gui
    GUIButton_ctor: (*const c_void);
    GUIDropDownMenu_ctor: (*const c_void);
    GUIDropDownMenuItem_ctor: (*const c_void);
    GUIDialog_ctor: (*const c_void);
    GUILabel_ctor: (*const c_void);
    GUILongLabel_ctor: (*const c_void);
    GUIRadioButton_ctor: (*const c_void);
    GUITextBox_ctor: (*const c_void);
    GUI_DisplayMessageBox: (*const c_void);
    GUI_DisplayMessageBox_Internal: (*const c_void);

    // input
    Input_GetKeyState: (fn(scancode: *mut u16) -> bool);
    Input_IsAnyKeyDown: (fn() -> bool);
    // GetInput: (fn(event: *mut Input_Event, unknown1: u32, unknown2: u32) -> i32);

    // lcd
    LCD_ClearScreen: (fn());
    LCD_GetPixel: (fn(x: u32, y: u32) -> u16);
    LCD_GetSize: (fn(width: *mut u32, height: *mut u32));
    LCD_GetVRAMAddress: (fn() -> *mut u16);
    LCD_Refresh: (fn());
    LCD_SetPixel: (fn(x: u32, y: u32, color: u16));
    LCD_SetPixelFromPalette: (fn(x: u32, y: u32, index: u8));
    LCD_VRAMBackup: (fn());
    LCD_VRAMRestore: (fn());
    LCD_SendCommand: (fn(command: u16));
    LCD_SetDrawingBounds: (fn(xstart: u32, xend: u32, ystart: u32, yend: u32));

    // mcs
    MCS_CreateFolder: (fn(folder: *const c_char, folder_index: *mut u8) -> i32);
    MCS_GetVariable: (fn(folder: *const c_char, name: *const c_char, variable_type: *mut u8, name2: *mut *mut c_char, data: *mut *mut c_void, size: *mut u32) -> i32);
    MCS_List_Create: (fn(folder: *const c_char, name: *const c_char, size: u32, length: u16, variable_type: u8) -> i32);
    MCS_List_Set: (fn(folder: *const c_char, name: *const c_char, size: u32, index: u32, variable_type: u8, data: *mut c_void) -> i32);
    MCS_SetVariable: (fn(folder: *const c_char, name: *const c_char, variable_type: u8, size: u32, data: *mut c_void) -> i32);

    // mem
    // free_list_head: (*const *mut Mem_FreeListBlock);
    Mem_Free: (fn(ptr: *mut c_void));
    Mem_Malloc: (fn(size: usize) -> *mut c_void);
    Mem_Memcpy: (fn(destination: *mut c_void, source: *const c_void, num: usize) -> *mut c_void);
    Mem_Memset: (fn(ptr: *mut c_void, value: i32, num: usize) -> *mut c_void);
    Mem_Sbrk: (fn(size: i32) -> *mut c_void);
    Mem_SbrkMode: (fn(new_mode: i8) -> i8);

    // serial
    // Serial_Open: (fn(mode: *const Serial_Settings) -> i32);
    Serial_Close: (fn(abort_pending: bool) -> i32);
    Serial_ReadSingle: (fn(out: *mut u8) -> i32);
    Serial_Read: (fn(out: *mut u8, sz: i32, count: *mut i16) -> i32);
    Serial_Peek: (fn(idx: i32, out: *mut u8) -> i32);
    Serial_WriteSingle: (fn(x: u8) -> i32);
    Serial_WriteUnbuffered: (fn(x: u8) -> i32);
    Serial_Write: (fn(buf: *const u8, count: i32) -> i32);
    Serial_PollRX: (fn() -> i32);
    Serial_PollTX: (fn() -> i32);
    Serial_ClearRX: (fn() -> i32);
    Serial_ClearTX: (fn() -> i32);
    Serial_IsOpen: (fn() -> i32);

    // string
    String_Strcat: (fn(dest: *mut c_char, src: *const c_char) -> *mut c_char);
    String_Strchr: (fn(str: *const c_char, c: c_char) -> *const c_char);
    String_Strcmp: (fn(str1: *const c_char, str2: *const c_char) -> i32);
    String_Strcpy: (fn(destination: *mut c_char, source: *const c_char) -> *mut c_char);
    String_Strlen: (fn(str: *const c_char) -> u32);

    // kernel
    _kernel_reset: (fn());
    _kernel_cnfgtbl: (*mut c_void);
    _kernel_sysmt: (*mut c_void);

    // misc
    Battery_GetVoltage: (fn() -> i32);
}

unsafe impl Sync for SymbolTable {}

impl SymbolTable {
    pub unsafe fn link(&mut self, mapping: &[u8]) -> bool {
        let safe_guard: *const u8 = ptr::with_exposed_provenance(SAFE_GUARD_ADDR);
        let (same, len) = unsafe { strn_eq_len(mapping.as_ptr(), safe_guard, SAFE_GUARD_SIZE) };
        if !same {
            return false;
        }

        let mut rest = &mapping[len + 1..];

        while !rest.is_empty() {
            let name = CStr::from_bytes_until_nul(rest).expect("Missing null terminator");
            let len = name.to_bytes_with_nul().len();
            let addr = rest[len..len + ADDR_SIZE].as_ptr();

            unsafe { self.assign_symbol(name.to_bytes(), addr) };

            rest = &rest[len + ADDR_SIZE..];
        }

        true
    }
}
