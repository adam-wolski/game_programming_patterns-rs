//! State Patterns
//! http://gameprogrammingpatterns.com/state.html

pub mod finite_state_machine;
pub mod state_pattern;

// Offscreen defines needed in examples.
#[derive(Debug, Clone, Copy)]
pub enum Input {
    PressB,
    _ReleaseB,
    PressDown,
    ReleaseDown,
}

impl Default for Input {
    fn default() -> Input {
        Input::PressB
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Graphic {
    Jump,
    Duck,
    Stand,
    Dive,
}

impl Default for Graphic {
    fn default() -> Graphic {
        Graphic::Stand
    }
}
