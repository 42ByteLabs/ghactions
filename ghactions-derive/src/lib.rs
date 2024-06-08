//! ghactions-derive is a library that provides derive macros for GitHub Actions in Rust.
//!
//! # Example
//!
//! ```rust
//! use ghactions::prelude::*;
//!
//! #[derive(Actions, Debug, Clone)]
//! #[action(
//!     path = "./action.yml",
//!     name = "My Action",
//!     description = "My Action Description"
//! )]
//! pub struct MyAction {
//!     /// My Input
//!     #[input(
//!         description = "My Input",
//!         default = "false"
//!     )]
//!     my_input: bool,
//!
//!     #[output(description = "Output Value")]
//!     my_output: String,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let action = MyAction::init()?;
//!
//!     println!("My Input   :: {}", action.my_input);
//!
//!     MyAction::set_output("my_output", "My Output Value")?;
//!
//!     Ok(())
//! }
//!
//! ```
#![allow(dead_code, unused_imports)]
#![deny(missing_docs)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod attributes;
mod derives;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for GitHub Actions
#[proc_macro_derive(Actions, attributes(action, input, output))]
pub fn actions(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    derives::derive_parser(&ast).unwrap().into()
}
