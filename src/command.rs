//! http://gameprogrammingpatterns.com/command.html
//! Not much problem with this one in rust.
//! We still can't return generic function though so we have to Box it.

struct Player;

impl Player {
    fn move_to(&self, x: i32, y: i32) {
        println!("x:{} y:{}", x, y);
    }
}

fn make_move_unit_cmd(unit: Player, x: i32, y:i32) -> Box<Fn()> {
    Box::new(move || unit.move_to(x, y))
}

pub fn test() {
    println!("---------------------------");
    println!("Command test.\n");
    let unit = Player;
    let cmd = make_move_unit_cmd(unit, 5, 10);
    cmd();
}
