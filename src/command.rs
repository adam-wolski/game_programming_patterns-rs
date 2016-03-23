//! Command Pattern
//! http://gameprogrammingpatterns.com/command.html

/// Our player character who we will be moving using our command.
pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    fn move_to(&mut self, x: i32, y: i32) {
        println!("Moving player to: ({}, {})", x, y);
        self.x = x;
        self.y = y;
    }
}

/// Command pattern command.
// We have to box the closure so the size of the return type is known on compile..
pub fn make_move_unit_cmd(x: i32, y: i32) -> Box<Fn(&mut Player)> {
    Box::new(move |unit: &mut Player| unit.move_to(x, y))
}

#[cfg(test)]
mod tests {
    use super::{Player, make_move_unit_cmd};

    #[test]
    pub fn command() {
        let mut unit = Player { x: 0, y: 0 };
        let cmd = make_move_unit_cmd(5, 10);
        cmd(&mut unit);
        assert!(unit.x == 5);
        assert!(unit.y == 10);
    }
}
