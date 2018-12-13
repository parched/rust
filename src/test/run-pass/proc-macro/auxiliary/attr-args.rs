// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// force-host
// no-prefer-dynamic

#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();

    assert_eq!(args, r#"text = "Hello, world!""#);

    let input = input.to_string();

    assert_eq!(input, "fn foo() { }");

    r#"
        fn foo() -> &'static str { "Hello, world!" }
    "#.parse().unwrap()
}

#[proc_macro_attribute]
pub fn identity(attr_args: TokenStream, _: TokenStream) -> TokenStream {
    attr_args
}
