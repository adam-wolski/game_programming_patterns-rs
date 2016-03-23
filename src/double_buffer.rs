//! Double Buffer pattern.
//! http://gameprogrammingpatterns.com/double-buffer.html


/// Screen and framebuffer width
const WIDTH: usize = 10;
/// Screen and framebuffer height
const HEIGHT: usize = 10;

const FRAME_BUFFER_SIZE: usize = WIDTH * HEIGHT * 4;

const NUM_BUFFERS: usize = 2;


#[derive(Default)]
pub struct FrameBuffer {
    pub data: Vec<u8>,
}

impl FrameBuffer {
    pub fn new() -> FrameBuffer {
        let mut data = Vec::with_capacity(FRAME_BUFFER_SIZE);
        for _ in 0..data.capacity() {
            data.push(0);
        }
        FrameBuffer { 
            data: data,
        }
    }

    /// Set location (x, y) in buffer to specified color tuple (R, G, B, A).
    pub fn draw(&mut self, loc: (usize, usize), color: (u8, u8, u8, u8)) {
        let pos = (loc.0 + loc.1 * WIDTH) * 4;
        self.data[pos] = color.2; // Blue
        self.data[pos + 1] = color.1; // Green
        self.data[pos + 2] = color.0; // Red
        self.data[pos + 3] = color.3; // Alpha
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.capacity() {
            self.data[i] = 0;
        }
    }

}


#[derive(Default)]
pub struct Scene {
    pub frame_buffers: Vec<FrameBuffer>,
    pub current_buffer: usize,
}

impl Scene {
    pub fn new() -> Scene {
        let mut buffers = Vec::with_capacity(NUM_BUFFERS);
        buffers.push(FrameBuffer::new());
        buffers.push(FrameBuffer::new());
        Scene {
            frame_buffers: buffers,
            current_buffer: 0,
        }
    }

    pub fn draw(&mut self) {
        {
            let buffer = &mut self.frame_buffers[self.current_buffer];

            buffer.clear();

            buffer.draw((1, 1), (255, 255, 255, 255));
            // ...
            buffer.draw((4, 3), (255, 255, 255, 255));
        }
        self.swap();
    }

    fn swap(&mut self) {
        self.current_buffer = (self.current_buffer + 1) % NUM_BUFFERS;
    }
}


#[cfg(test)]
mod tests {
    use super::{Scene};

    #[test]
    fn double_buffer() {
        let mut scene = Scene::new();
        assert!(scene.frame_buffers[0].data[0] == 0);
        assert!(scene.frame_buffers[1].data[0] == 0);
        scene.draw();
        assert!(scene.frame_buffers[0].data[45] == 255);
        assert!(scene.frame_buffers[1].data[45] == 0);
        scene.draw();
        assert!(scene.frame_buffers[0].data[45] == 255);
        assert!(scene.frame_buffers[1].data[45] == 255);
    }

}
