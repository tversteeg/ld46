use crate::{input::Input, upgrade::Upgrades};
use direct_gui::{
    controls::{Button, ControlState},
    Gui as InternalGui,
};
use specs_blit::{blit::Color, PixelBuffer};

/// A GUI system that allows us to draw nice buttons and text.
pub struct Gui {
    internal: InternalGui,
}

impl Gui {
    /// Instantiate a new gui with the proper framebuffer size.
    pub fn new(buffer_width: usize, buffer_height: usize) -> Self {
        let mut internal = InternalGui::new((buffer_width as i32, buffer_height as i32));

        for (pos, size) in Upgrades::buttons() {
            internal.register(Button::new(size, Color::from_u32(0x333333)).with_pos(pos.0, pos.1));
        }

        Self { internal }
    }

    /// Draw a label.
    pub fn draw_label<S: Into<String>>(
        &mut self,
        buffer: &mut PixelBuffer,
        text: S,
        x: i32,
        y: i32,
    ) {
        self.internal.draw_label(
            buffer.pixels_mut(),
            self.internal.default_font(),
            text,
            (x, y),
        );
    }

    /// Draw everything.
    pub fn draw(&mut self, buffer: &mut PixelBuffer, input: &Input) {
        let mut cs = ControlState {
            ..ControlState::default()
        };

        cs.mouse_pos = (input.mouse_x() - 1, input.mouse_y() - 1);
        cs.mouse_down = input.mouse_down();

        self.internal.update(&cs);
        self.internal.draw_to_buffer(buffer.pixels_mut());
    }
}
