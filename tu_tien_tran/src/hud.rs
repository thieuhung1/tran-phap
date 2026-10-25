use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};
use crate::theme::RealmTheme;

pub struct HudState {
    pub message: String,
    pub message_timer: f32,
    pub show_help: bool,
}

impl HudState {
    pub fn new() -> Self {
        Self {
            message: "PHAP TRAN KHOI DONG - BAM [H] DE XEM HUONG DAN".to_string(),
            message_timer: 5.0,
            show_help: false,
        }
    }

    pub fn notify(&mut self, text: impl Into<String>) {
        self.message = text.into();
        self.message_timer = 3.5;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn update(&mut self, dt: f32) {
        if self.message_timer > 0.0 {
            self.message_timer -= dt;
        }
    }

    pub fn draw(
        &self,
        pixmap: &mut Pixmap,
        _w: f32,
        h: f32,
        theme: &RealmTheme,
        click_through: bool,
        speed_name: &str,
    ) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        // 1. Draw temporary notification message banner if timer > 0
        if self.message_timer > 0.0 {
            let alpha = (self.message_timer / 0.5).min(1.0).clamp(0.0, 1.0);
            let alpha_u8 = (alpha * 240.0) as u8;

            let banner_y = h - 55.0;
            // Draw background pill
            let text_w = (self.message.len() as f32) * 8.0 + 30.0;
            let start_x = (pixmap.width() as f32 - text_w) / 2.0;

            let mut bg_path = PathBuilder::new();
            bg_path.push_rect(tiny_skia::Rect::from_xywh(start_x, banner_y - 12.0, text_w, 24.0).unwrap());
            if let Some(path) = bg_path.finish() {
                let mut bg_paint = Paint::default();
                bg_paint.set_color_rgba8(10, 12, 18, (alpha * 190.0) as u8);
                pixmap.fill_path(&path, &bg_paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

                let mut stroke_paint = Paint::default();
                stroke_paint.set_color_rgba8(pr, pg, pb, (alpha * 140.0) as u8);
                let stroke = Stroke { width: 1.2, ..Default::default() };
                pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
            }

            draw_text(
                pixmap,
                start_x + 15.0,
                banner_y - 5.0,
                &self.message,
                Color::from_rgba8(cr, cg, cb, alpha_u8),
                1.3,
            );
        }

        // 2. Draw Help Panel if toggled on
        if self.show_help {
            let panel_w = 420.0;
            let panel_h = 290.0;

            let start_x = (pixmap.width() as f32 - panel_w) / 2.0;
            let start_y = (h - panel_h) / 2.0;

            let mut bg_path = PathBuilder::new();
            bg_path.push_rect(tiny_skia::Rect::from_xywh(start_x, start_y, panel_w, panel_h).unwrap());
            if let Some(path) = bg_path.finish() {
                let mut bg_paint = Paint::default();
                bg_paint.set_color_rgba8(8, 10, 16, 225);
                pixmap.fill_path(&path, &bg_paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

                let mut border_paint = Paint::default();
                border_paint.set_color_rgba8(pr, pg, pb, 200);
                let stroke = Stroke { width: 1.5, ..Default::default() };
                pixmap.stroke_path(&path, &border_paint, &stroke, Transform::identity(), None);
            }

            let lines = [
                "=== PHAP TRAN TU TIEN - DIEU KHIEN ===",
                "",
                "[CHUOT TRAI]: KEO DI CHUYEN PHAP TRAN",
                "[CHUOT PHAI / SPACE]: DOI CANH GIOI",
                "[G / ALT+G / C]: DOI CHE DO DONG HO (FULL / MINIMAL / OFF)",
                "[CHUOT GIUA / S / ALT+S]: DOI TOC DO QUAY",
                "[PHIM 1-5]: CHON 5 MUC TOC DO (TINH TAM -> BAO PHAT)",
                "[PHIM [ / ] HOAC UP/DOWN]: TANG / GIAM TOC DO",
                "[SHIFT + CUON CHUOT]: DIEU CHINH TOC DO",
                "[CUON CHUOT / +-]: PHONG TO / THU NHO",
                "[T / ALT+T]: XUYEN THAU CHUOT (CLICK-THROUGH)",
                "[P]: BAT / TAT LINH KHI DOM DOM",
                "[H]: DONG BANG HUONG DAN NAY",
                "[ESC / Q / ALT+Q]: THU HOI PHAP TRAN (THOAT)",
            ];

            let mut line_y = start_y + 18.0;
            for (idx, line) in lines.iter().enumerate() {
                let color = if idx == 0 {
                    Color::from_rgba8(pr, pg, pb, 255)
                } else if line.starts_with('[') {
                    Color::from_rgba8(cr, cg, cb, 230)
                } else {
                    Color::from_rgba8(180, 190, 210, 200)
                };
                draw_text(pixmap, start_x + 20.0, line_y, line, color, 1.2);
                line_y += 18.0;
            }
        }

        // 3. Small status tags at top of window
        let click_str = if click_through { "XUYEN THAU: BAT" } else { "XUYEN THAU: TAT" };
        let status_text = format!("CANH GIOI: {} | TOC DO: {} | {}", theme.short_name(), speed_name, click_str);
        let tag_w = (status_text.len() as f32) * 6.5 + 20.0;
        let tag_x = (pixmap.width() as f32 - tag_w) / 2.0;


        let mut tag_bg = PathBuilder::new();
        tag_bg.push_rect(tiny_skia::Rect::from_xywh(tag_x, 15.0, tag_w, 20.0).unwrap());
        if let Some(path) = tag_bg.finish() {
            let mut p = Paint::default();
            p.set_color_rgba8(12, 14, 20, 180);
            pixmap.fill_path(&path, &p, tiny_skia::FillRule::Winding, Transform::identity(), None);

            let mut border = Paint::default();
            border.set_color_rgba8(pr, pg, pb, 90);
            pixmap.stroke_path(&path, &border, &Stroke { width: 1.0, ..Default::default() }, Transform::identity(), None);
        }

        draw_text(
            pixmap,
            tag_x + 10.0,
            20.0,
            &status_text,
            Color::from_rgba8(cr, cg, cb, 200),
            1.1,
        );
    }
}

/// Simple 5x7 bitmap font drawer for crisp, zero-dependency text
pub fn draw_text(
    pixmap: &mut Pixmap,
    x: f32,
    y: f32,
    text: &str,
    color: Color,
    scale: f32,
) {
    let mut cur_x = x;
    let mut paint = Paint::default();
    paint.set_color(color);

    for ch in text.chars() {
        let glyph = get_glyph(ch);
        for row in 0..7 {
            for col in 0..5 {
                if (glyph[row] & (1 << (4 - col))) != 0 {
                    let px = cur_x + (col as f32) * scale;
                    let py = y + (row as f32) * scale;
                    let mut b = PathBuilder::new();
                    b.push_rect(tiny_skia::Rect::from_xywh(px, py, scale.max(1.0), scale.max(1.0)).unwrap());
                    if let Some(p) = b.finish() {
                        pixmap.fill_path(&p, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                    }
                }
            }
        }
        cur_x += 6.0 * scale;
    }
}

fn get_glyph(c: char) -> [u8; 7] {
    match c.to_ascii_uppercase() {
        'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
        'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        'G' => [0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'I' => [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
        'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001],
        'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        '0' => [0b01110, 0b10011, 0b10101, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        '3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
        '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
        '6' => [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
        '[' => [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110],
        ']' => [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110],
        ':' => [0b00000, 0b00100, 0b00000, 0b00000, 0b00100, 0b00000, 0b00000],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '=' => [0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '+' => [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
        '/' => [0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000, 0b00000],
        '.' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00110, 0b00110],
        '|' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        ' ' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        _   => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    }
}
