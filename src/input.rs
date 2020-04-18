use miniquad::KeyCode;

#[derive(Debug, Default)]
pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,

    mouse_x: i32,
    mouse_y: i32,
    mouse_down: bool,
}

impl Input {
    /// Get whether the up key is pressed or not.
    pub fn up_pressed(&self) -> bool {
        self.up
    }

    /// Get whether the down key is pressed or not.
    pub fn down_pressed(&self) -> bool {
        self.down
    }

    /// Get whether the left key is pressed or not.
    pub fn left_pressed(&self) -> bool {
        self.left
    }

    /// Get whether the right key is pressed or not.
    pub fn right_pressed(&self) -> bool {
        self.right
    }

    /// Get the mouse x position.
    pub fn mouse_x(&self) -> i32 {
        self.mouse_x
    }

    /// Get the mouse y position.
    pub fn mouse_y(&self) -> i32 {
        self.mouse_y
    }

    /// Handle miniquad key events.
    pub fn handle_key(&mut self, code: KeyCode, is_down: bool) {
        match code {
            KeyCode::Up | KeyCode::W | KeyCode::Comma => {
                self.up = is_down;
            }
            KeyCode::A => {
                self.left = is_down;
            }
            KeyCode::Down | KeyCode::S | KeyCode::O => {
                self.down = is_down;
            }
            KeyCode::D | KeyCode::E => {
                self.right = is_down;
            }
            _ => (),
        }
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
