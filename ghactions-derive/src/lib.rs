//! ghactions-derive is a library that provides derive macros for GitHub Actions in Rust.
//!
//! # Example
//!
//! ```no_run
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
//!     #[input(
//!         name = "custom",
//!         description = "My Custom Input",
//!     )]
//!     my_custom: String,
//!
//!     #[input(
//!         description = "Multi Input",
//!         split = ",",
//!     )]
//!     multi_input: Vec<String>,
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
//!     # assert_eq!(action.my_input, false);
//!     # assert_eq!(action.my_custom, "Custom Value");
//!     # assert_eq!(action.multi_input, vec!["this".to_string(), "is".to_string(), "a".to_string(), "test".to_string()]);
//!
//!     MyAction::set_output("my_output", "My Output Value")?;
//!
//!     Ok(())
//! }
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
use syn::{DeriveInput, parse_macro_input};

/// Derive macro for GitHub Actions
#[proc_macro_derive(Actions, attributes(action, input, output))]
pub fn actions(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    match derives::derive_parser(&ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
