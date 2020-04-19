#[derive(Debug, Default)]
pub struct Input {
    mouse_x: i32,
    mouse_y: i32,
    mouse_down: bool,
}

impl Input {
    /// Get the mouse x position.
    pub fn mouse_x(&self) -> i32 {
        self.mouse_x
    }

    /// Get the mouse y position.
    pub fn mouse_y(&self) -> i32 {
        self.mouse_y
    }

    /// Get whether a mouse button is pressed.
    pub fn mouse_down(&self) -> bool {
        self.mouse_down
    }

    /// Handle miniquad mouse button events.
    pub fn handle_mouse_button(&mut self, is_down: bool) {
        self.mouse_down = is_down;
    }

    /// Handle miniquad mouse move events.
    pub fn handle_mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
}
