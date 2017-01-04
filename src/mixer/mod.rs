//! Testing module level documentation

use errors::*;
use dispenser::Dispenser;
use std::{collections, fmt};

struct Mixer {
    pub dispensers: collections::HashMap<String, Box<Dispenser>>,
}

impl fmt::Debug for Mixer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, dispenser) in self.dispensers.iter() {
            writeln!(f,
                     "{} => \n\tmax_flow_rate: {} mL/s\n\tremaining: {} mL",
                     name,
                     dispenser.max_flow_rate(),
                     dispenser.remaining().unwrap());
        }
        Ok(())
    }
}

impl Mixer {
    fn new() -> Mixer {
        let mut dispensers = collections::HashMap::new();
        Mixer { dispensers: dispensers }
    }

    fn clear(&mut self) {
        self.dispensers.clear();
    }

    fn get(&mut self, name: &str) -> Option<&Box<Dispenser>> {
        self.dispensers.get(name)
    }

    ///
    fn remove(&mut self, name: &str) {
        self.dispensers.remove(name);
    }

    ///
    fn add(&mut self, name: &str, dispenser: Box<Dispenser>) {
        self.dispensers.insert(String::from(name), dispenser);
    }

    // fn make_recipe() {
    //
    // }
}

#[cfg(test)]
mod test {
    use dispenser::TestDispenser;
    use mixer::Mixer;

    #[test]
    fn test_create_a_mixer() {
        let mut mixer = Mixer::new();
        let mut dispenser1 = Box::new(TestDispenser::new(1.0, 3.0));
        let mut dispenser2 = Box::new(TestDispenser::new(1.0, 3.0));
        let mut dispenser3 = Box::new(TestDispenser::new(1.0, 3.0));

        mixer.add("whiskey", dispenser1);
        mixer.add("vodka", dispenser2);
        mixer.add("gin", dispenser3);

        mixer.get("whiskey").unwrap();
        mixer.get("vodka").unwrap();
        mixer.get("gin").unwrap();

        mixer.remove("whiskey");
        mixer.remove("vodka");
        mixer.remove("gin");

        assert!(mixer.get("whiskey").is_none());
        assert!(mixer.get("vodka").is_none());
        assert!(mixer.get("gin").is_none());
    }
}
