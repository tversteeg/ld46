use miniquad::KeyCode;

#[derive(Debug, Default)]
pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
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

    /// Handle miniquad key events.
    pub fn handle_key(&mut self, code: KeyCode, is_down: bool) {
        match code {
            KeyCode::W | KeyCode::Comma => {
                self.up = is_down;
            }
            KeyCode::A => {
                self.left = is_down;
            }
            KeyCode::S | KeyCode::O => {
                self.down = is_down;
            }
            KeyCode::D | KeyCode::E => {
                self.right = is_down;
            }
            _ => (),
        }
    }
}
