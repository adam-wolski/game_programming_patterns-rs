//! Command Pattern
//! http://gameprogrammingpatterns.com/command.html

/// Generic game unit that has it's own 2D position.
pub struct Unit {
    pub x: i32,
    pub y: i32,
}

impl Unit {
    fn move_to(&mut self, x: i32, y: i32) {
        println!("Moving player to: ({}, {})", x, y);
        self.x = x;
        self.y = y;
    }
}


// Functional Version
/// Return 2 closures one with command to move Player and other one that will undo that action.
pub fn make_move_unit_cmd(unit: &Unit, x: i32, y: i32) -> (Box<Fn(&mut Unit)>, Box<Fn(&mut Unit)>) {
    let Unit { x: last_x, y: last_y } = *unit;
    (Box::new(move |unit: &mut Unit| unit.move_to(x, y)),
     Box::new(move |unit: &mut Unit| unit.move_to(last_x, last_y)))
}


#[cfg(test)]
mod tests {
    use super::{Unit, make_move_unit_cmd};

    #[test]
    pub fn command() {
        let mut unit = Unit { x: 0, y: 0 };
        let (cmd, undo) = make_move_unit_cmd(&unit, 5, 10);
        cmd(&mut unit);
        assert!(unit.x == 5);
        assert!(unit.y == 10);
        undo(&mut unit);
        assert!(unit.x == 0);
        assert!(unit.y == 0);
    }
}
