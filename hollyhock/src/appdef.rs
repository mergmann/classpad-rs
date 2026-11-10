#[repr(C, align(4))]
pub struct Note<const N: usize, const D: usize> {
    namesz: u32,
    descsz: u32,
    kind: u32,
    name: [u8; N],
    desc: [u8; D],
}

pub const fn padded(len: usize) -> usize {
    (len + 1 + 3) & !3
}

impl<const N: usize, const D: usize> Note<N, D> {
    pub const fn new(name: &str, desc: &str) -> Self {
        Note {
            namesz: name.len() as u32 + 1,
            descsz: desc.len() as u32 + 1,
            kind: 1,
            name: copy_padded(name.as_bytes()),
            desc: copy_padded(desc.as_bytes()),
        }
    }
}

const fn copy_padded<const L: usize>(s: &[u8]) -> [u8; L] {
    let mut out = [0u8; L];
    let mut i = 0;
    while i < s.len() {
        out[i] = s[i];
        i += 1;
    }
    out
}

#[doc(hidden)]
#[macro_export]
macro_rules! hollyhock_note {
    ($ident:ident, $name:literal, $value:expr) => {
        #[unsafe(link_section = ".note.hollyhock")]
        #[used]
        static $ident: $crate::appdef::Note<
            { $crate::appdef::padded($name.len()) },
            { $crate::appdef::padded($value.len()) },
        > = $crate::appdef::Note::new($name, $value);
    };
}

/// define_app! {
///     NAME = "Example",
///     AUTHOR = "me",
///     VERSION = "0.1.0"
///     DESCRIPTION = "An example for Rust on hollyhock!"
/// }
#[macro_export]
macro_rules! define_app {
    (@priv note, $ident:ident, $name:expr, $value:expr) => {
        #[unsafe(link_section = ".note.hollyhock")]
        #[used]
        static $ident: $crate::appdef::Note<
            { $crate::appdef::padded($name.len()) },
            { $crate::appdef::padded($value.len()) },
        > = $crate::appdef::Note::new($name, $value);
    };
    ($($note:ident = $value:expr),*$(,)?) => {
        mod appdef {
            $( $crate::define_app!(@priv note, $note, stringify!($note), $value); )*
        }
    };
}

#[macro_export]
macro_rules! define_app_cargo {
    () => {
        $crate::define_app! {
            NAME = env!("CARGO_PKG_NAME"),
            AUTHOR = env!("CARGO_PKG_AUTHORS"),
            VERSION = env!("CARGO_PKG_VERSION"),
            DESCRIPTION = env!("CARGO_PKG_DESCRIPTION")
        }
    };
}
