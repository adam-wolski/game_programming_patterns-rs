//! http://gameprogrammingpatterns.com/command.html
//! Not much problem with this one in rust.
//! We still can't return closure though so we have to Box it.

struct Player {
    pub x: i32,
    pub y: i32
}

impl Player {
    fn move_to(&mut self, x: i32, y: i32) {
        println!("Moving player to: ({}, {})", x, y);
        self.x = x;
        self.y = y;
    }
}

fn make_move_unit_cmd(x: i32, y:i32) -> Box<Fn(&mut Player)> {
    Box::new(move |unit: &mut Player| unit.move_to(x, y))
}

#[cfg(test)]
mod tests {
    use super::{ Player, make_move_unit_cmd};

    #[test]
    pub fn visual() {
        println!("\n---------------------------");
        println!("Command test.\n");
        let mut unit = Player {x: 0, y: 0};
        let cmd = make_move_unit_cmd(5, 10);
        cmd(&mut unit);
        assert!(unit.x == 5);
        assert!(unit.y == 10);
    }
}
