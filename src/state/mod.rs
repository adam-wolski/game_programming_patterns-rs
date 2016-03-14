//! http://gameprogrammingpatterns.com/state.html

pub mod finite_state_machine;
// TODO state_pattern
// pub mod state_pattern;

// Offscreen defines needed in examples.
#[derive(Debug, Clone, Copy)]
enum Input {
    PressB,
    ReleaseB,
    PressDown,
    ReleaseDown
}

#[derive(Debug, Clone, Copy)]
enum Graphic {
    Jump,
    Duck,
    Stand,
    Dive
}
