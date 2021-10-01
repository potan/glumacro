extern crate proc_macro;

use gluon::{vm::api::IO, ThreadExt};

use proc_macro::{TokenStream, Literal};
use syn::token::Token;
use proc_macro::Span;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn a_proc_macro(input: TokenStream) -> TokenStream {
    //let i = format!("{:?}", input);
    //let res = format!("{:?}", i);
    //let mut iter = input.into_iter();
    //let prog_tok = iter.next().unwrap();
    //let i = format!("{:?}", prog_tok);
    //let res = format!("{:?}", i);
    let i = parse_macro_input!(input as LitStr);
    let prog = i.value();
    let vm = gluon::new_vm();
    vm.run_io(true);
    let (res, _) = vm.run_expr::<String>("example", &prog).unwrap();
    res.parse().unwrap()
}
