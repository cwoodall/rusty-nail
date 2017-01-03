//! Testing module level documentation

use errors::*;
use dispenser::Dispenser;
use std::collections;

struct Mixer {
    dispensers: collections::HashMap<String, Box<Dispenser>>,
}

impl Mixer {
    fn clear(&mut self) {
        self.dispensers.clear();
    }

    ///
    fn remove_dispenser(&mut self, name: &str) {
        self.dispensers.remove(name);
    }

    ///
    fn add_dispenser(&mut self, name: &str, dispenser: Box<Dispenser>) {
        self.dispensers.insert(String::from(name), dispenser);
    }

    // fn make_recipe() {
    //
    // }
}
