//! Type Object Pattern
//! http://gameprogrammingpatterns.com/type-object.html

pub trait Monster {
    fn get_health(&self) -> u32;
    fn get_attack(&self) -> String;
    fn attack(&self) {
        println!("{}", self.get_attack());
    }
}


pub struct Breed {
    health: u32,
    attack: String,
}

impl Breed {
    /// Create new breed. When given parent health and attack will be inherited.
    pub fn new(parent: Option<&Breed>, health: u32, attack: String) -> Breed {
        if let Some(p) = parent {
            Breed {
                health: p.get_health(),
                attack: p.get_attack(),
            }
        } else {
            Breed {
                health: health,
                attack: attack,
            }
        }
    }
}

impl Monster for Breed {
    fn get_health(&self) -> u32 {
        self.health
    }
    fn get_attack(&self) -> String {
        self.attack.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_object() {
        let parent = Breed::new(None, 15, "ARGH!".to_owned());
        assert!(parent.get_attack() == "ARGH!");
        assert!(parent.get_health() == 15);
        let child = Breed::new(Some(&parent), 0, "".to_owned());
        assert!(child.get_attack() == "ARGH!");
        assert!(child.get_health() == 15);
        child.attack();
    }
}
