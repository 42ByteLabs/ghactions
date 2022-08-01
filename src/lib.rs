#![allow(dead_code)]
#![allow(unused_imports)]
mod models;
mod ghaction;

use ghaction::GHAction;


pub fn init() -> GHAction {
    GHAction::new()
}


#[cfg(test)]
mod tests {
}
