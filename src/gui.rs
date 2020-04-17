use direct_gui::Gui as InternalGui;
use specs_blit::PixelBuffer;

/// A GUI system that allows us to draw nice buttons and text.
pub struct Gui {
    internal: InternalGui,
}

impl Gui {
    /// Instantiate a new gui with the proper framebuffer size.
    pub fn new(buffer_width: usize, buffer_height: usize) -> Self {
        Self {
            internal: InternalGui::new((buffer_width as i32, buffer_height as i32)),
        }
    }

    /// Render the startup GUI on the pixel buffer.
    pub fn render_startup(&mut self, buffer: &mut PixelBuffer) {
        self.internal.draw_label(
            buffer.pixels_mut(),
            self.internal.default_font(),
            "Press SPACE to play..",
            (20, 20),
        );
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
}
