// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name="html5ever"]
#![crate_type="dylib"]

#![cfg_attr(feature = "unstable", feature(plugin))]
#![cfg_attr(test, deny(warnings))]
#![allow(unused_parens)]

#![cfg_attr(feature = "unstable", plugin(string_cache_plugin))]

extern crate rc;

#[macro_use]
extern crate log;

#[macro_use]
extern crate string_cache;

extern crate tendril;

#[macro_use]
extern crate mac;

extern crate phf;

extern crate time;

pub use tokenizer::Attribute;
pub use driver::{one_input, ParseOpts, parse_to, parse_fragment_to, parse, parse_fragment};

pub use serialize::serialize;

#[macro_use]
mod macros;

#[macro_use]
mod util {
    pub mod str;
    #[macro_use] pub mod smallcharset;
}

pub mod tokenizer;
pub mod tree_builder;
pub mod serialize;
pub mod driver;
pub mod rcdom;
