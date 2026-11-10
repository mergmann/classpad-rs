use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let ident = &input.sig.ident;

    quote! {
        #[doc(hidden)]
        #[unsafe(no_mangle)]
        unsafe extern "C" fn main(
            argc: ::core::ffi::c_int,
            argv: *const *const ::core::ffi::c_char,
            _envp: *const *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int {
            #input

            unsafe {
                ::hollyhock::rt::start_rt(argc, argv, || ::hollyhock::rt::Termination::report(#ident()) )
            }
        }
    }
    .into()
}
