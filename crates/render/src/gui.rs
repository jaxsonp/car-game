use assets::fonts::FONT_MONO;
use wgpu_text::{
    BrushBuilder, TextBrush,
    glyph_brush::{
        BuiltInLineBreaker, HorizontalAlign, OwnedSection, Section, Text, VerticalAlign,
        ab_glyph::FontRef,
    },
};

/// Wraps all the ugly egui boilerplate.
///
/// To use, create `TextBox`es in `GuiOverlay.text_boxes`
pub struct GuiOverlay {
    brush: TextBrush<FontRef<'static>>,
    pub top_left_text: TextBox,
}

impl GuiOverlay {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> GuiOverlay {
        let brush = BrushBuilder::using_font_bytes(FONT_MONO)
            .unwrap()
            .draw_cache_position_tolerance(0.7) // tolerate liberal reuse of gylphs
            .build(device, config.width, config.height, config.format);

        let top_left_text = TextBox::new(
            (5.0, 5.0),
            (400.0, 600.0),
            HorizontalAlign::Left,
            VerticalAlign::Top,
            "FPS",
            [0.0, 0.0, 0.0, 1.0],
            30.0,
        );

        return GuiOverlay {
            brush,
            top_left_text,
        };
    }

    pub fn handle_resize(&mut self, width: u32, height: u32, queue: &wgpu::Queue) {
        self.brush.resize_view(width as f32, height as f32, queue);
    }

    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let sections = [&self.top_left_text.section];
        self.brush
            .queue(device, queue, sections)
            .expect("Failed during preparation of gui overlay");
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.brush.draw(render_pass);
    }
}

pub struct TextBox {
    section: OwnedSection,
}

impl TextBox {
    pub fn new<D: Into<(f32, f32)>>(
        pos: D,
        size: D,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        init_text: &str,
        color: [f32; 4],
        font_size: f32,
    ) -> TextBox {
        TextBox {
            section: Section::default()
                .with_bounds(size)
                .with_layout(
                    wgpu_text::glyph_brush::Layout::default()
                        .h_align(h_align)
                        .v_align(v_align)
                        .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
                )
                .with_screen_position(pos)
                .add_text(Text::new(init_text).with_color(color).with_scale(font_size))
                .to_owned(),
        }
    }

    pub fn change_text<S: Into<String>>(&mut self, new_text: S) {
        self.section.text[0].text = new_text.into();
    }
}
