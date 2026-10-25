use std::f32::consts::PI;
use tiny_skia::{
    Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform,
};
use windows_sys::Win32::Foundation::SYSTEMTIME;
use windows_sys::Win32::System::SystemInformation::GetLocalTime;

use crate::hud::draw_text;
use crate::theme::RealmTheme;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockMode {
    Full,        // Đầy đủ: 3 Kim Kiếm + Bảng giờ số + 12 Canh Giờ
    Minimal,     // Tối giản: Bảng giờ số tinh tế + Kim giây quét
    HandsOnly,   // Chỉ kim: 3 Kim kiếm tiên đạo xoay theo thời gian thực
    Hidden,      // Ẩn: Trở về pháp trận nguyên bản
}

impl ClockMode {
    pub fn next(&self) -> Self {
        match self {
            ClockMode::Full => ClockMode::Minimal,
            ClockMode::Minimal => ClockMode::HandsOnly,
            ClockMode::HandsOnly => ClockMode::Hidden,
            ClockMode::Hidden => ClockMode::Full,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ClockMode::Full => "DONG HO: TOAN BO (FULL)",
            ClockMode::Minimal => "DONG HO: TOI GIAN (MINIMAL)",
            ClockMode::HandsOnly => "DONG HO: KIM TRAP (HANDS ONLY)",
            ClockMode::Hidden => "DONG HO: TAT (HIDDEN)",
        }
    }
}

pub struct CelestialClock {
    pub mode: ClockMode,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub millis: u16,
    pub day: u16,
    pub month: u16,
    pub year: u16,
}

impl CelestialClock {
    pub fn new() -> Self {
        Self {
            mode: ClockMode::Full, // Default enabled!
            hour: 0,
            minute: 0,
            second: 0,
            millis: 0,
            day: 1,
            month: 1,
            year: 2026,
        }
    }

    pub fn update(&mut self) {
        unsafe {
            let mut st: SYSTEMTIME = std::mem::zeroed();
            GetLocalTime(&mut st);
            self.hour = st.wHour;
            self.minute = st.wMinute;
            self.second = st.wSecond;
            self.millis = st.wMilliseconds;
            self.day = st.wDay;
            self.month = st.wMonth;
            self.year = st.wYear;
        }
    }

    /// Lấy thông tin 12 Canh Giờ truyền thống (Tý, Sửu, Dần, Mão...)
    pub fn get_canh_gio(&self) -> (&'static str, &'static str, usize) {
        // Mỗi canh giờ = 2 tiếng. Giờ Tý bắt đầu từ 23h đến 0h59.
        let h = self.hour;
        match h {
            23 | 0 => ("TY", "DA BAN (23H-01H)", 0),
            1 | 2 => ("SUU", "KE MINH (01H-03H)", 1),
            3 | 4 => ("DAN", "BINH DAN (03H-05H)", 2),
            5 | 6 => ("MAO", "NHAT XUAT (05H-07H)", 3),
            7 | 8 => ("THIN", "THUC THOI (07H-09H)", 4),
            9 | 10 => ("TY", "NGU TRUNG (09H-11H)", 5),
            11 | 12 => ("NGO", "NHAT TRUNG (11H-13H)", 6),
            13 | 14 => ("MUI", "NHAT DIET (13H-15H)", 7),
            15 | 16 => ("THAN", "BO THOI (15H-17H)", 8),
            17 | 18 => ("DAU", "NHAT NHAP (17H-19H)", 9),
            19 | 20 => ("TUAT", "HOANG HON (19H-21H)", 10),
            21 | 22 => ("HOI", "NHAN DINH (21H-23H)", 11),
            _ => ("TY", "DA BAN", 0),
        }
    }

    pub fn draw(
        &self,
        pixmap: &mut Pixmap,
        cx: f32,
        cy: f32,
        max_r: f32,
        theme: &RealmTheme,
    ) {
        if self.mode == ClockMode::Hidden {
            return;
        }

        // Continuous smooth time fractions for butter-smooth animation
        let sec_smooth = (self.second as f32) + (self.millis as f32) / 1000.0;
        let min_smooth = (self.minute as f32) + sec_smooth / 60.0;
        let hr_smooth = ((self.hour % 12) as f32) + min_smooth / 60.0;


        let angle_sec = sec_smooth * (2.0 * PI / 60.0) - PI * 0.5;
        let angle_min = min_smooth * (2.0 * PI / 60.0) - PI * 0.5;
        let angle_hr = hr_smooth * (2.0 * PI / 12.0) - PI * 0.5;

        // 1. Draw 12 Canh Giờ & 60 Minute Ticks around the clock ring (Full mode)
        if self.mode == ClockMode::Full {
            self.draw_dial(pixmap, cx, cy, max_r, theme);
        }

        // 2. Draw Analog Clock Hands (Kim Pháp Trận)
        if self.mode == ClockMode::Full || self.mode == ClockMode::HandsOnly {
            // Kim Giờ: Hoàng Kim Cự Kiếm (Thần Kiếm)
            self.draw_hour_sword(pixmap, cx, cy, angle_hr, max_r * 0.44, theme);

            // Kim Phút: Thanh Phong Linh Kiếm
            self.draw_minute_sword(pixmap, cx, cy, angle_min, max_r * 0.65, theme);
        }

        // Kim Giây: Tia Linh Lực Quét Mượt Mà (Full, HandsOnly, Minimal)
        self.draw_second_beam(pixmap, cx, cy, angle_sec, max_r * 0.82, theme);

        // 3. Digital Jade Time Tablet (Bảng Giờ Số Tiên Đạo)
        if self.mode == ClockMode::Full || self.mode == ClockMode::Minimal {
            self.draw_digital_tablet(pixmap, cx, cy, max_r, theme);
        }
    }

    /// Vẽ 12 Vị Trí Canh Giờ và 60 Vạch Phút
    fn draw_dial(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();
        let (_, _, current_canh_idx) = self.get_canh_gio();

        let r_dial = max_r * 0.86;
        let canh_gio_names = [
            "TY", "SUU", "DAN", "MAO", "THIN", "TY.",
            "NGO", "MUI", "THAN", "DAU", "TUAT", "HOI"
        ];

        // 12 Canh Giờ Positions
        for i in 0..12 {
            let a = (i as f32) * (2.0 * PI / 12.0) - PI * 0.5;
            let nx = cx + a.cos() * r_dial;
            let ny = cy + a.sin() * r_dial;

            let is_current = i == current_canh_idx;

            if is_current {
                // Quầng sáng nổi bật cho Canh Giờ hiện tại
                let mut glow_b = PathBuilder::new();
                glow_b.push_circle(nx, ny, 10.0);
                if let Some(gp) = glow_b.finish() {
                    let mut gpaint = Paint::default();
                    gpaint.anti_alias = true;
                    gpaint.set_color_rgba8(cr, cg, cb, 100);
                    pixmap.fill_path(&gp, &gpaint, FillRule::Winding, Transform::identity(), None);
                }

                // Vòng kim hoàn
                let mut ring_b = PathBuilder::new();
                ring_b.push_circle(nx, ny, 7.5);
                if let Some(rp) = ring_b.finish() {
                    let mut rpaint = Paint::default();
                    rpaint.anti_alias = true;
                    rpaint.set_color_rgba8(pr, pg, pb, 230);
                    pixmap.stroke_path(&rp, &rpaint, &Stroke { width: 1.5, ..Default::default() }, Transform::identity(), None);
                }
            } else {
                // Chấm bảo thạch bình thường
                let mut dot_b = PathBuilder::new();
                dot_b.push_circle(nx, ny, 2.5);
                if let Some(dp) = dot_b.finish() {
                    let mut dpaint = Paint::default();
                    dpaint.anti_alias = true;
                    dpaint.set_color_rgba8(pr, pg, pb, 140);
                    pixmap.fill_path(&dp, &dpaint, FillRule::Winding, Transform::identity(), None);
                }
            }

            // Tên Canh Giờ viết bằng chữ vector
            let label = canh_gio_names[i];
            let label_r = r_dial + if is_current { 14.0 } else { 12.0 };
            let lx = cx + a.cos() * label_r - 7.0;
            let ly = cy + a.sin() * label_r - 4.0;
            let text_color = if is_current {
                Color::from_rgba8(cr, cg, cb, 255)
            } else {
                Color::from_rgba8(pr, pg, pb, 150)
            };
            draw_text(pixmap, lx, ly, label, text_color, 1.0);
        }
    }

    /// Kim Giờ: Thần Kiếm Tiên Đạo
    fn draw_hour_sword(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, angle: f32, length: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        let transform = Transform::from_translate(cx, cy).pre_rotate(angle.to_degrees() + 90.0);

        // Lưỡi kiếm giờ
        let mut b = PathBuilder::new();
        b.move_to(-3.5, 0.0);
        b.line_to(-4.5, -length * 0.7);
        b.line_to(0.0, -length);
        b.line_to(4.5, -length * 0.7);
        b.line_to(3.5, 0.0);
        b.line_to(0.0, 10.0); // Chuôi kiếm đối trọng
        b.close();

        if let Some(path) = b.finish() {
            // Hào quang tỏa sáng của Thần Kiếm
            let mut glow_p = Paint::default();
            glow_p.anti_alias = true;
            glow_p.set_color_rgba8(pr, pg, pb, 90);
            pixmap.stroke_path(&path, &glow_p, &Stroke { width: 3.5, ..Default::default() }, transform, None);

            // Thân kiếm phát quang
            let mut body_p = Paint::default();
            body_p.anti_alias = true;
            body_p.set_color_rgba8(cr, cg, cb, 220);
            pixmap.fill_path(&path, &body_p, FillRule::Winding, transform, None);

            // Sống kiếm trung tâm
            let mut spine_b = PathBuilder::new();
            spine_b.move_to(0.0, 8.0);
            spine_b.line_to(0.0, -length * 0.85);
            if let Some(sp) = spine_b.finish() {
                let mut spine_p = Paint::default();
                spine_p.anti_alias = true;
                spine_p.set_color_rgba8(pr, pg, pb, 255);
                pixmap.stroke_path(&sp, &spine_p, &Stroke { width: 1.2, ..Default::default() }, transform, None);
            }
        }
    }

    /// Kim Phút: Thanh Phong Linh Kiếm
    fn draw_minute_sword(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, angle: f32, length: f32, theme: &RealmTheme) {
        let (sr, sg, sb) = theme.secondary();
        let (cr, cg, cb) = theme.core();

        let transform = Transform::from_translate(cx, cy).pre_rotate(angle.to_degrees() + 90.0);

        let mut b = PathBuilder::new();
        b.move_to(-2.2, 0.0);
        b.line_to(-2.5, -length * 0.8);
        b.line_to(0.0, -length);
        b.line_to(2.5, -length * 0.8);
        b.line_to(2.2, 0.0);
        b.line_to(0.0, 12.0);
        b.close();

        if let Some(path) = b.finish() {
            let mut glow_p = Paint::default();
            glow_p.anti_alias = true;
            glow_p.set_color_rgba8(sr, sg, sb, 110);
            pixmap.stroke_path(&path, &glow_p, &Stroke { width: 2.8, ..Default::default() }, transform, None);

            let mut body_p = Paint::default();
            body_p.anti_alias = true;
            body_p.set_color_rgba8(cr, cg, cb, 210);
            pixmap.fill_path(&path, &body_p, FillRule::Winding, transform, None);
        }
    }

    /// Kim Giây: Quét Mượt Mà (Smooth Sweeping Qi Beam)
    fn draw_second_beam(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, angle: f32, length: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        let tip_x = cx + angle.cos() * length;
        let tip_y = cy + angle.sin() * length;

        let tail_x = cx - angle.cos() * (length * 0.20);
        let tail_y = cy - angle.sin() * (length * 0.20);

        // Tia năng lượng kim giây
        let mut b = PathBuilder::new();
        b.move_to(tail_x, tail_y);
        b.line_to(tip_x, tip_y);
        if let Some(path) = b.finish() {
            let mut glow = Paint::default();
            glow.anti_alias = true;
            glow.set_color_rgba8(pr, pg, pb, 90);
            pixmap.stroke_path(&path, &glow, &Stroke { width: 2.5, ..Default::default() }, Transform::identity(), None);

            let mut core = Paint::default();
            core.anti_alias = true;
            core.set_color_rgba8(cr, cg, cb, 240);
            pixmap.stroke_path(&path, &core, &Stroke { width: 1.0, ..Default::default() }, Transform::identity(), None);
        }

        // Tinh Cầu phát sáng ở đầu kim giây (Quang Cầu Dẫn Đường)
        let mut orb_b = PathBuilder::new();
        orb_b.push_circle(tip_x, tip_y, 4.0);
        if let Some(path) = orb_b.finish() {
            let mut p = Paint::default();
            p.anti_alias = true;
            p.set_color_rgba8(cr, cg, cb, 255);
            pixmap.fill_path(&path, &p, FillRule::Winding, Transform::identity(), None);
        }

        // Hạt đối trọng ở đuôi
        let mut tail_b = PathBuilder::new();
        tail_b.push_circle(tail_x, tail_y, 2.5);
        if let Some(path) = tail_b.finish() {
            let mut p = Paint::default();
            p.anti_alias = true;
            p.set_color_rgba8(pr, pg, pb, 200);
            pixmap.fill_path(&path, &p, FillRule::Winding, Transform::identity(), None);
        }
    }

    /// Bảng Giờ Số Tiên Đạo (Digital Jade Chronometer)
    fn draw_digital_tablet(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        let (canh_name, canh_desc, _) = self.get_canh_gio();

        // Chuỗi giờ điện tử: HH:MM:SS
        let colon = if self.millis < 500 { ":" } else { " " };
        let time_str = format!("{:02}{}{:02}{}{:02}", self.hour, colon, self.minute, colon, self.second);
        let date_str = format!("{:02}/{:02}/{}", self.day, self.month, self.year);
        let canh_str = format!("[ CANH {} - {} ]", canh_name, canh_desc);

        // Kích thước phiến ngọc hiển thị giờ
        let tablet_w = 210.0;
        let tablet_h = 58.0;
        let tablet_x = cx - tablet_w / 2.0;
        let tablet_y = cy + max_r * 0.22; // Nằm thanh thoát ngay phía dưới tâm Thái Cực

        // Nền phiến ngọc bán trong suốt
        let mut bg_b = PathBuilder::new();
        bg_b.push_rect(tiny_skia::Rect::from_xywh(tablet_x, tablet_y, tablet_w, tablet_h).unwrap());
        if let Some(bg_path) = bg_b.finish() {
            let mut bg_paint = Paint::default();
            bg_paint.anti_alias = true;
            bg_paint.set_color_rgba8(6, 8, 14, 215);
            pixmap.fill_path(&bg_path, &bg_paint, FillRule::Winding, Transform::identity(), None);

            // Viền phiến ngọc
            let mut border_paint = Paint::default();
            border_paint.anti_alias = true;
            border_paint.set_color_rgba8(pr, pg, pb, 160);
            pixmap.stroke_path(&bg_path, &border_paint, &Stroke { width: 1.2, ..Default::default() }, Transform::identity(), None);

            // 4 Góc trang trí hoa văn tiên gia
            let corner_len = 6.0;
            let mut corner_b = PathBuilder::new();
            // Góc trên trái
            corner_b.move_to(tablet_x - 2.0, tablet_y + corner_len);
            corner_b.line_to(tablet_x - 2.0, tablet_y - 2.0);
            corner_b.line_to(tablet_x + corner_len, tablet_y - 2.0);
            // Góc trên phải
            corner_b.move_to(tablet_x + tablet_w - corner_len, tablet_y - 2.0);
            corner_b.line_to(tablet_x + tablet_w + 2.0, tablet_y - 2.0);
            corner_b.line_to(tablet_x + tablet_w + 2.0, tablet_y + corner_len);
            // Góc dưới trái
            corner_b.move_to(tablet_x - 2.0, tablet_y + tablet_h - corner_len);
            corner_b.line_to(tablet_x - 2.0, tablet_y + tablet_h + 2.0);
            corner_b.line_to(tablet_x + corner_len, tablet_y + tablet_h + 2.0);
            // Góc dưới phải
            corner_b.move_to(tablet_x + tablet_w - corner_len, tablet_y + tablet_h + 2.0);
            corner_b.line_to(tablet_x + tablet_w + 2.0, tablet_y + tablet_h + 2.0);
            corner_b.line_to(tablet_x + tablet_w + 2.0, tablet_y + tablet_h - corner_len);

            if let Some(cp) = corner_b.finish() {
                let mut cp_paint = Paint::default();
                cp_paint.anti_alias = true;
                cp_paint.set_color_rgba8(cr, cg, cb, 220);
                pixmap.stroke_path(&cp, &cp_paint, &Stroke { width: 1.5, ..Default::default() }, Transform::identity(), None);
            }
        }

        // Vẽ chữ giờ to nổi bật
        let time_w = (time_str.len() as f32) * 6.0 * 2.1;
        let time_x = cx - time_w / 2.0;
        draw_text(pixmap, time_x, tablet_y + 8.0, &time_str, Color::from_rgba8(cr, cg, cb, 255), 2.1);

        // Vẽ thông tin Canh Giờ
        let canh_w = (canh_str.len() as f32) * 6.0 * 1.0;
        let canh_x = cx - canh_w / 2.0;
        draw_text(pixmap, canh_x, tablet_y + 29.0, &canh_str, Color::from_rgba8(pr, pg, pb, 230), 1.0);

        // Vẽ Ngày tháng năm
        let date_w = (date_str.len() as f32) * 6.0 * 0.9;
        let date_x = cx - date_w / 2.0;
        draw_text(pixmap, date_x, tablet_y + 42.0, &date_str, Color::from_rgba8(170, 185, 210, 190), 0.9);
    }
}
