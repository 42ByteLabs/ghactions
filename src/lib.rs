#![allow(dead_code)]
#![allow(unused_imports)]
pub mod models;
pub mod ghaction;

pub use ghaction::GHAction;


pub fn init() -> GHAction {
    GHAction::new()
}


#[cfg(test)]
mod tests {

}
