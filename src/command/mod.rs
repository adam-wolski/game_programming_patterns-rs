//! http://gameprogrammingpatterns.com/command.html
//! Not much problem with this one in rust.
//! We still can't return generic function though so we have to Box it.

struct Player {
    pub x: i32,
    pub y: i32
}

impl Player {
    fn move_to(&self, x: i32, y: i32) {
        println!("Moving player to: ({}, {})", x, y);
        self.x = x;
        self.y = y;
    }
}

fn make_move_unit_cmd(unit: Player, x: i32, y:i32) -> Box<Fn()> {
    Box::new(move || unit.move_to(x, y))
}

#[cfg(test)]
mod tests {
    use super::command::*;

    #[test]
    pub fn visual() {
        println!("\n---------------------------");
        println!("Command test.\n");
        let unit = Player{x: 0, y: 0};
        let cmd = make_move_unit_cmd(unit, 5, 10);
        assert!(unit.x == 5);
        assert!(unit.y == 10);
        cmd();
    }
}
