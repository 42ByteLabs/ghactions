#![allow(dead_code)]
#![allow(unused_imports)]
mod models;
mod ghaction;

use crate::ghaction::GHAction;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
