//! Game Loop Pattern
//! http://gameprogrammingpatterns.com/game-loop.html

use std::time;

static MS_PER_UPDATE: u64 = 16;


pub fn game_loop() {
    let ms_per_update = time::Duration::from_millis(MS_PER_UPDATE);

    let mut previous = time::Instant::now();
    let mut lag = time::Duration::new(0, 0);
    loop {
        let elapsed = time::Instant::now().duration_since(previous);
        previous = time::Instant::now();
        lag = lag + elapsed;

        process_input();

        while lag >= ms_per_update {
            update();
            lag = lag - ms_per_update;
        }

        render(lag);
    }
}

fn update() {
    println!("Updating the game.");
    // ...
}

fn process_input() {
    // ...
}

fn render(_state: time::Duration) {
    // ...
}
