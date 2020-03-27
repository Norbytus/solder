extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Ident, FnArg, Type};
use solder::zend::{ExecuteData, Zval};

#[proc_macro_attribute]
pub fn php_function(attr: TokenStream, item: TokenStream) -> TokenStream {

    let ident: Ident = parse_macro_input!(attr as Ident);

    let args: ItemFn = parse_macro_input!(item as ItemFn);

    for input in args.sig.inputs {
        match input {
            FnArg::Typed(arg) => {
            },
            _ => {}
        }
    }

    let function = quote! {
        fn #ident(data: &ExecuteData, retval: &mut Zval) {
        }
    };

    TokenStream::from(function)
}
