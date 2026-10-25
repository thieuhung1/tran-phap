use std::f32::consts::PI;
use tiny_skia::{
    Color, FillRule, GradientStop, Paint, PathBuilder, Pixmap, Point, RadialGradient, SpreadMode, Stroke, Transform,
};
use crate::particles::ParticleSystem;
use crate::theme::RealmTheme;

pub struct MagicFormation {
    pub angle_taiji: f32,
    pub angle_bagua: f32,
    pub angle_star: f32,
    pub angle_runes: f32,
    pub angle_outer: f32,
    pub pulse_phase: f32,
    pub lightning_timer: f32,
    pub lightning_arcs: Vec<Vec<(f32, f32)>>,
    pub particles: ParticleSystem,
}

impl MagicFormation {
    pub fn new() -> Self {
        Self {
            angle_taiji: 0.0,
            angle_bagua: 0.0,
            angle_star: 0.0,
            angle_runes: 0.0,
            angle_outer: 0.0,
            pulse_phase: 0.0,
            lightning_timer: 0.0,
            lightning_arcs: Vec::new(),
            particles: ParticleSystem::new(55, 270.0),
        }
    }

    pub fn update(&mut self, dt: f32, speed_mult: f32, radius: f32) {
        let base_speed = dt * speed_mult;
        self.angle_taiji += base_speed * 0.8;
        self.angle_bagua -= base_speed * 0.35;
        self.angle_star += base_speed * 0.22;
        self.angle_runes -= base_speed * 0.15;
        self.angle_outer += base_speed * 0.30;
        self.pulse_phase += base_speed * 1.5;

        // Update lightning discharge
        self.lightning_timer -= dt;
        if self.lightning_timer <= 0.0 {
            self.lightning_timer = 0.12 + ((self.pulse_phase * 17.0).sin().abs() * 0.3) / speed_mult;
            self.generate_lightning(radius);
        }

        self.particles.update(dt, speed_mult, radius * 0.92);
    }

    fn generate_lightning(&mut self, max_r: f32) {
        self.lightning_arcs.clear();
        let num_bolts = if (self.pulse_phase.sin() * 10.0) as i32 % 3 == 0 { 2 } else { 1 };
        for b in 0..num_bolts {
            let base_angle = (self.angle_runes * 2.0 + (b as f32) * 2.5) % (2.0 * PI);
            let mut pts = Vec::new();
            let start_r = max_r * 0.35;
            let end_r = max_r * 0.88;
            let segments = 7;
            for i in 0..=segments {
                let frac = i as f32 / segments as f32;
                let r = start_r + (end_r - start_r) * frac;
                let jitter = if i == 0 || i == segments { 0.0 } else { ((i as f32 * 3.7 + self.pulse_phase).sin()) * 0.12 };
                let theta = base_angle + jitter;
                pts.push((theta.cos() * r, theta.sin() * r));
            }
            self.lightning_arcs.push(pts);
        }
    }

    pub fn render(
        &self,
        pixmap: &mut Pixmap,
        cx: f32,
        cy: f32,
        max_r: f32,
        theme: &RealmTheme,
        show_particles: bool,
    ) {
        // 1. Draw Expanding Spirit Shockwaves (Linh Lực Ba Động)
        self.draw_shockwaves(pixmap, cx, cy, max_r, theme);

        // 2. Draw Outer Protective Shields & Concentric Rings (Cửu Trọng Khiên)
        self.draw_outer_shields(pixmap, cx, cy, max_r, theme);

        // 3. Draw Celestial Runes & Astrolabe Ticks Ring (Chu Thiên Phù Lục & Tinh Đẩu)
        self.draw_celestial_runes(pixmap, cx, cy, max_r, theme);

        // 4. Draw Sacred Geometry Octagram / Squares (Bát Phương Thần Trận)
        self.draw_sacred_octagram(pixmap, cx, cy, max_r, theme);

        // 5. Draw Bát Quái Đồ (Eight Trigrams Array)
        self.draw_bagua_ring(pixmap, cx, cy, max_r, theme);

        // 6. Draw Inner Star Network (Ngũ Hành / Thần Thập Tinh)
        self.draw_inner_star(pixmap, cx, cy, max_r, theme);

        // 7. Draw Arcing Lightning Discharges
        self.draw_lightning(pixmap, cx, cy, theme);

        // 8. Draw Taiji Core (Thái Cực Đồ)
        self.draw_taiji(pixmap, cx, cy, max_r, theme);

        // 9. Draw Spirit Particles
        if show_particles {
            self.particles.draw(pixmap, cx, cy, theme);
        }
    }

    /// 1. Linh Lực Ba Động (Pulsing expanding energy waves)
    fn draw_shockwaves(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        for i in 0..3 {
            let offset = i as f32 * 0.333;
            let progress = ((self.pulse_phase * 0.4 + offset) % 1.0 + 1.0) % 1.0;
            let r = max_r * (0.15 + 0.80 * progress);
            let alpha = ((1.0 - progress) * 110.0).clamp(0.0, 110.0) as u8;

            let mut b = PathBuilder::new();
            b.push_circle(cx, cy, r);
            if let Some(path) = b.finish() {
                let mut paint = Paint::default();
                paint.anti_alias = true;
                paint.set_color_rgba8(pr, pg, pb, alpha);
                let stroke = Stroke {
                    width: 1.0 + (1.0 - progress) * 2.5,
                    ..Default::default()
                };
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }

            // Subtle inner glow wave
            if progress < 0.5 {
                let mut glow_b = PathBuilder::new();
                glow_b.push_circle(cx, cy, r);
                if let Some(gp) = glow_b.finish() {
                    let mut gpaint = Paint::default();
                    gpaint.anti_alias = true;
                    gpaint.set_color_rgba8(cr, cg, cb, (alpha / 2).min(50));
                    let stroke = Stroke { width: 0.8, ..Default::default() };
                    pixmap.stroke_path(&gp, &gpaint, &stroke, Transform::identity(), None);
                }
            }
        }
    }

    /// 2. Outer Protective Shields & Orbiting Beacons
    fn draw_outer_shields(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (sr, sg, sb) = theme.secondary();
        let (cr, cg, cb) = theme.core();

        let r_outer = max_r * 0.94;
        let r_shield = max_r * 0.89;
        let r_inner = max_r * 0.84;

        // Outermost boundary thin ring
        let mut b = PathBuilder::new();
        b.push_circle(cx, cy, r_outer);
        if let Some(path) = b.finish() {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color_rgba8(pr, pg, pb, 90);
            pixmap.stroke_path(&path, &paint, &Stroke { width: 1.2, ..Default::default() }, Transform::identity(), None);
        }

        // Segmented thick shield arcs (4 primary quadrants rotating clockwise)
        let num_shields = 4;
        let arc_len = PI * 0.36; // arc span
        for i in 0..num_shields {
            let center_angle = self.angle_outer + (i as f32) * (2.0 * PI / num_shields as f32);
            let start_a = center_angle - arc_len / 2.0;
            let end_a = center_angle + arc_len / 2.0;

            let mut arc_b = PathBuilder::new();
            let steps = 24;
            for s in 0..=steps {
                let a = start_a + (end_a - start_a) * (s as f32 / steps as f32);
                let x = cx + a.cos() * r_shield;
                let y = cy + a.sin() * r_shield;
                if s == 0 { arc_b.move_to(x, y); } else { arc_b.line_to(x, y); }
            }

            if let Some(path) = arc_b.finish() {
                // Outer glow stroke
                let mut glow_p = Paint::default();
                glow_p.anti_alias = true;
                glow_p.set_color_rgba8(pr, pg, pb, 70);
                pixmap.stroke_path(&path, &glow_p, &Stroke { width: 4.5, ..Default::default() }, Transform::identity(), None);

                // Core stroke
                let mut core_p = Paint::default();
                core_p.anti_alias = true;
                core_p.set_color_rgba8(cr, cg, cb, 210);
                pixmap.stroke_path(&path, &core_p, &Stroke { width: 1.8, ..Default::default() }, Transform::identity(), None);
            }

            // Terminal glowing beads on shield ends
            for &terminal_angle in &[start_a, end_a] {
                let tx = cx + terminal_angle.cos() * r_shield;
                let ty = cy + terminal_angle.sin() * r_shield;
                let mut dot_b = PathBuilder::new();
                dot_b.push_circle(tx, ty, 3.2);
                if let Some(dp) = dot_b.finish() {
                    let mut dpaint = Paint::default();
                    dpaint.anti_alias = true;
                    dpaint.set_color_rgba8(cr, cg, cb, 240);
                    pixmap.fill_path(&dp, &dpaint, FillRule::Winding, Transform::identity(), None);
                }
            }
        }

        // 8 counter-rotating smaller accent arcs on inner radius
        let num_inner_arcs = 8;
        let inner_len = PI * 0.12;
        for i in 0..num_inner_arcs {
            let center_angle = -self.angle_outer * 1.3 + (i as f32) * (2.0 * PI / num_inner_arcs as f32);
            let start_a = center_angle - inner_len / 2.0;
            let end_a = center_angle + inner_len / 2.0;

            let mut arc_b = PathBuilder::new();
            let steps = 14;
            for s in 0..=steps {
                let a = start_a + (end_a - start_a) * (s as f32 / steps as f32);
                let x = cx + a.cos() * r_inner;
                let y = cy + a.sin() * r_inner;
                if s == 0 { arc_b.move_to(x, y); } else { arc_b.line_to(x, y); }
            }
            if let Some(path) = arc_b.finish() {
                let mut p = Paint::default();
                p.anti_alias = true;
                p.set_color_rgba8(sr, sg, sb, 150);
                pixmap.stroke_path(&path, &p, &Stroke { width: 1.5, ..Default::default() }, Transform::identity(), None);
            }
        }

        // Orbiting Satellite Beacons (4 Celestial Pearls / Ngũ Hành Tinh Cầu)
        for i in 0..4 {
            let a = self.angle_outer * 1.8 + (i as f32) * (PI * 0.5);
            let bx = cx + a.cos() * r_outer;
            let by = cy + a.sin() * r_outer;

            let mut orb_b = PathBuilder::new();
            orb_b.push_circle(bx, by, 4.0);
            if let Some(path) = orb_b.finish() {
                let mut p = Paint::default();
                p.anti_alias = true;
                p.set_color_rgba8(cr, cg, cb, 255);
                pixmap.fill_path(&path, &p, FillRule::Winding, Transform::identity(), None);
            }
        }
    }

    /// 3. Chu Thiên Phù Lục & Tinh Đẩu (Celestial Runes & Astrolabe Ticks)
    fn draw_celestial_runes(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        let r_mid = max_r * 0.77;
        let r_in = max_r * 0.71;
        let r_out = max_r * 0.83;

        // Border rings for rune canal
        for &r in &[r_in, r_out] {
            let mut b = PathBuilder::new();
            b.push_circle(cx, cy, r);
            if let Some(path) = b.finish() {
                let mut paint = Paint::default();
                paint.anti_alias = true;
                paint.set_color_rgba8(pr, pg, pb, 110);
                pixmap.stroke_path(&path, &paint, &Stroke { width: 1.0, ..Default::default() }, Transform::identity(), None);
            }
        }

        // 72 Astrolabe Ticks (72 Địa Sát Tinh Vĩ)
        let total_ticks = 72;
        for i in 0..total_ticks {
            let a = self.angle_runes + (i as f32) * (2.0 * PI / total_ticks as f32);
            let is_major = i % 6 == 0;
            let tick_len = if is_major { 8.0 } else { 4.0 };
            let r1 = r_out;
            let r2 = r_out - tick_len;

            let mut b = PathBuilder::new();
            b.move_to(cx + a.cos() * r1, cy + a.sin() * r1);
            b.line_to(cx + a.cos() * r2, cy + a.sin() * r2);
            if let Some(path) = b.finish() {
                let mut paint = Paint::default();
                paint.anti_alias = true;
                let alpha = if is_major { 220 } else { 90 };
                paint.set_color_rgba8(pr, pg, pb, alpha);
                let stroke = Stroke { width: if is_major { 1.5 } else { 0.8 }, ..Default::default() };
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }

        // 16 Procedural Daoist Esoteric Runes (Cổ Triện Tiên Phù)
        let num_runes = 16;
        for i in 0..num_runes {
            let a = self.angle_runes + (i as f32) * (2.0 * PI / num_runes as f32) + 0.1;
            let rx = cx + a.cos() * r_mid;
            let ry = cy + a.sin() * r_mid;

            // Draw procedural talisman glyph at (rx, ry) rotated tangentially
            self.draw_daoist_glyph(pixmap, rx, ry, a + PI * 0.5, i, theme);
        }

        // 12 Major Celestial Diamond Nodes
        for i in 0..12 {
            let a = self.angle_runes + (i as f32) * (2.0 * PI / 12.0);
            let dx = cx + a.cos() * r_in;
            let dy = cy + a.sin() * r_in;
            self.draw_diamond(pixmap, dx, dy, 3.5, Color::from_rgba8(cr, cg, cb, 230));
        }
    }

    /// Procedural Daoist Rune / Talisman Glyph (Phù Lục Cổ Tự)
    fn draw_daoist_glyph(&self, pixmap: &mut Pixmap, x: f32, y: f32, rotation: f32, seed: usize, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        let transform = Transform::from_translate(x, y).pre_rotate(rotation.to_degrees());

        let mut b = PathBuilder::new();
        let s = seed % 4;

        // Distinct ancient seal strokes based on seed
        match s {
            0 => {
                // Heavenly Canopy & Lightning Hook
                b.move_to(-6.0, -8.0);
                b.line_to(6.0, -8.0);
                b.move_to(0.0, -8.0);
                b.line_to(0.0, 2.0);
                b.line_to(4.0, 5.0);
                b.line_to(-2.0, 8.0);
            }
            1 => {
                // Sacred Spiral Dragon Seal
                b.move_to(0.0, -8.0);
                b.line_to(0.0, 8.0);
                b.move_to(-5.0, -3.0);
                b.line_to(5.0, -3.0);
                b.move_to(-4.0, 4.0);
                b.line_to(4.0, 4.0);
                b.move_to(-5.0, -6.0);
                b.line_to(-2.0, -3.0);
            }
            2 => {
                // Tri-fold Cloud Talisman
                b.move_to(-5.0, -7.0);
                b.line_to(0.0, -4.0);
                b.line_to(5.0, -7.0);
                b.move_to(0.0, -4.0);
                b.line_to(0.0, 8.0);
                b.move_to(-4.0, 1.0);
                b.line_to(4.0, 1.0);
                b.move_to(-5.0, 8.0);
                b.line_to(5.0, 8.0);
            }
            _ => {
                // Star Seal Core
                b.move_to(0.0, -8.0);
                b.line_to(0.0, 8.0);
                b.move_to(-5.0, 0.0);
                b.line_to(5.0, 0.0);
                b.move_to(-3.0, -5.0);
                b.line_to(3.0, 5.0);
                b.move_to(3.0, -5.0);
                b.line_to(-3.0, 5.0);
            }
        }

        if let Some(path) = b.finish() {
            // Glow
            let mut glow = Paint::default();
            glow.anti_alias = true;
            glow.set_color_rgba8(pr, pg, pb, 90);
            pixmap.stroke_path(&path, &glow, &Stroke { width: 2.8, ..Default::default() }, transform, None);

            // Core stroke
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color_rgba8(cr, cg, cb, 210);
            pixmap.stroke_path(&path, &paint, &Stroke { width: 1.2, ..Default::default() }, transform, None);
        }
    }

    /// 4. Sacred Geometry Octagram / Interlocking Squares (Bát Phương Thần Trận)
    fn draw_sacred_octagram(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (sr, sg, sb) = theme.secondary();
        let (cr, cg, cb) = theme.core();

        let r_star = max_r * 0.58;

        // Two interlocking squares rotated 45 degrees relative to each other
        for sq in 0..2 {
            let offset = self.angle_star + (sq as f32) * (PI * 0.25);
            let mut b = PathBuilder::new();
            for i in 0..4 {
                let a = offset + (i as f32) * (PI * 0.5);
                let x = cx + a.cos() * r_star;
                let y = cy + a.sin() * r_star;
                if i == 0 { b.move_to(x, y); } else { b.line_to(x, y); }
            }
            b.close();

            if let Some(path) = b.finish() {
                // Secondary glow
                let mut gp = Paint::default();
                gp.anti_alias = true;
                gp.set_color_rgba8(sr, sg, sb, 80);
                pixmap.stroke_path(&path, &gp, &Stroke { width: 3.0, ..Default::default() }, Transform::identity(), None);

                // Main stroke
                let mut mp = Paint::default();
                mp.anti_alias = true;
                mp.set_color_rgba8(pr, pg, pb, 190);
                pixmap.stroke_path(&path, &mp, &Stroke { width: 1.4, ..Default::default() }, Transform::identity(), None);
            }
        }

        // Draw diamond runes and node circles at all 8 tips of the octagram
        for i in 0..8 {
            let a = self.angle_star + (i as f32) * (PI * 0.25);
            let nx = cx + a.cos() * r_star;
            let ny = cy + a.sin() * r_star;

            // Small outer ring
            let mut ring_b = PathBuilder::new();
            ring_b.push_circle(nx, ny, 5.5);
            if let Some(rp) = ring_b.finish() {
                let mut p = Paint::default();
                p.anti_alias = true;
                p.set_color_rgba8(pr, pg, pb, 170);
                pixmap.stroke_path(&rp, &p, &Stroke { width: 1.2, ..Default::default() }, Transform::identity(), None);
            }

            // Glowing center diamond
            self.draw_diamond(pixmap, nx, ny, 3.0, Color::from_rgba8(cr, cg, cb, 240));
        }

        // Inner circular guideline connecting intersections
        let r_mid = r_star * 0.707;
        let mut b = PathBuilder::new();
        b.push_circle(cx, cy, r_mid);
        if let Some(path) = b.finish() {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color_rgba8(pr, pg, pb, 70);
            pixmap.stroke_path(&path, &paint, &Stroke { width: 1.0, ..Default::default() }, Transform::identity(), None);
        }
    }

    /// 5. Bát Quái Đồ (Eight Trigrams Ring)
    fn draw_bagua_ring(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        let r_bagua = max_r * 0.44;
        let r_border_in = max_r * 0.38;
        let r_border_out = max_r * 0.50;

        // Concentric Bagua boundaries
        for &r in &[r_border_in, r_border_out] {
            let mut b = PathBuilder::new();
            b.push_circle(cx, cy, r);
            if let Some(path) = b.finish() {
                let mut paint = Paint::default();
                paint.anti_alias = true;
                paint.set_color_rgba8(pr, pg, pb, 90);
                pixmap.stroke_path(&path, &paint, &Stroke { width: 1.0, ..Default::default() }, Transform::identity(), None);
            }
        }

        // Early Heaven Bagua Trigrams (Tiên Thiên Bát Quái)
        // 0: Càn (☰), 1: Đoài (☱), 2: Ly (☲), 3: Chấn (☳), 4: Tốn (☴), 5: Khảm (☵), 6: Cấn (☶), 7: Khôn (☷)
        // True = Yang (Liền ───), False = Yin (Đứt ─ ─)
        let trigrams: [[bool; 3]; 8] = [
            [true, true, true],     // 0: Càn ☰ (Nam)
            [false, true, true],    // 1: Đoài ☱ (Đông Nam)
            [true, false, true],    // 2: Ly ☲ (Đông)
            [false, false, true],   // 3: Chấn ☳ (Đông Bắc)
            [true, true, false],    // 4: Tốn ☴ (Tây Nam)
            [false, true, false],   // 5: Khảm ☵ (Tây)
            [true, false, false],   // 6: Cấn ☶ (Tây Bắc)
            [false, false, false],  // 7: Khôn ☷ (Bắc)
        ];

        let bar_width = max_r * 0.085;
        let bar_height = 2.4;
        let bar_spacing = 4.2;
        let gap = 3.5;

        for i in 0..8 {
            let a = self.angle_bagua + (i as f32) * (PI * 0.25);
            let bx = cx + a.cos() * r_bagua;
            let by = cy + a.sin() * r_bagua;

            // Transform rotated tangentially
            let transform = Transform::from_translate(bx, by).pre_rotate((a + PI * 0.5).to_degrees());

            // Draw the 3 lines of the trigram
            for line_idx in 0..3 {
                let y_pos = ((line_idx as f32) - 1.0) * bar_spacing;
                let is_yang = trigrams[i][line_idx];

                let mut b = PathBuilder::new();
                if is_yang {
                    // Continuous Yang bar
                    b.push_rect(tiny_skia::Rect::from_xywh(-bar_width / 2.0, y_pos - bar_height / 2.0, bar_width, bar_height).unwrap());
                } else {
                    // Broken Yin bar (two halves with center gap)
                    let half_w = (bar_width - gap) / 2.0;
                    b.push_rect(tiny_skia::Rect::from_xywh(-bar_width / 2.0, y_pos - bar_height / 2.0, half_w, bar_height).unwrap());
                    b.push_rect(tiny_skia::Rect::from_xywh(gap / 2.0, y_pos - bar_height / 2.0, half_w, bar_height).unwrap());
                }

                if let Some(path) = b.finish() {
                    let mut p = Paint::default();
                    p.anti_alias = true;
                    p.set_color_rgba8(cr, cg, cb, 230);
                    pixmap.fill_path(&path, &p, FillRule::Winding, transform, None);
                }
            }

            // Decorative arch bracket framing each trigram
            let mut bracket = PathBuilder::new();
            let bw = bar_width * 0.7;
            bracket.move_to(-bw, -9.0);
            bracket.line_to(-bw - 3.0, -9.0);
            bracket.line_to(-bw - 3.0, 9.0);
            bracket.line_to(-bw, 9.0);

            bracket.move_to(bw, -9.0);
            bracket.line_to(bw + 3.0, -9.0);
            bracket.line_to(bw + 3.0, 9.0);
            bracket.line_to(bw, 9.0);

            if let Some(bp) = bracket.finish() {
                let mut bp_paint = Paint::default();
                bp_paint.anti_alias = true;
                bp_paint.set_color_rgba8(pr, pg, pb, 140);
                pixmap.stroke_path(&bp, &bp_paint, &Stroke { width: 1.0, ..Default::default() }, transform, None);
            }
        }
    }

    /// 6. Inner Star Network (Ngũ Hành / Thần Thập Tinh)
    fn draw_inner_star(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (sr, sg, sb) = theme.secondary();
        let (cr, cg, cb) = theme.core();

        let r_inner = max_r * 0.28;
        let count = 6; // Hexagram / Lục Hợp Thần Trận

        // Draw star lines
        for i in 0..count {
            let a1 = -self.angle_star * 1.5 + (i as f32) * (2.0 * PI / count as f32);
            let a2 = -self.angle_star * 1.5 + ((i + 2) as f32) * (2.0 * PI / count as f32);

            let x1 = cx + a1.cos() * r_inner;
            let y1 = cy + a1.sin() * r_inner;
            let x2 = cx + a2.cos() * r_inner;
            let y2 = cy + a2.sin() * r_inner;

            let mut b = PathBuilder::new();
            b.move_to(x1, y1);
            b.line_to(x2, y2);
            if let Some(path) = b.finish() {
                let mut paint = Paint::default();
                paint.anti_alias = true;
                paint.set_color_rgba8(sr, sg, sb, 120);
                pixmap.stroke_path(&path, &paint, &Stroke { width: 1.2, ..Default::default() }, Transform::identity(), None);
            }
        }

        // Tips beads
        for i in 0..count {
            let a = -self.angle_star * 1.5 + (i as f32) * (2.0 * PI / count as f32);
            let x = cx + a.cos() * r_inner;
            let y = cy + a.sin() * r_inner;

            let mut b = PathBuilder::new();
            b.push_circle(x, y, 3.2);
            if let Some(path) = b.finish() {
                let mut paint = Paint::default();
                paint.anti_alias = true;
                paint.set_color_rgba8(cr, cg, cb, 230);
                pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
            }
        }

        // Inner containment ring
        let mut b = PathBuilder::new();
        b.push_circle(cx, cy, r_inner);
        if let Some(path) = b.finish() {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color_rgba8(pr, pg, pb, 130);
            pixmap.stroke_path(&path, &paint, &Stroke { width: 1.0, ..Default::default() }, Transform::identity(), None);
        }
    }

    /// 7. Arcing Lightning Discharges
    fn draw_lightning(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        for bolt in &self.lightning_arcs {
            if bolt.len() < 2 { continue; }
            let mut b = PathBuilder::new();
            b.move_to(cx + bolt[0].0, cy + bolt[0].1);
            for pt in bolt.iter().skip(1) {
                b.line_to(cx + pt.0, cy + pt.1);
            }

            if let Some(path) = b.finish() {
                // Wide glow stroke
                let mut glow_p = Paint::default();
                glow_p.anti_alias = true;
                glow_p.set_color_rgba8(pr, pg, pb, 90);
                pixmap.stroke_path(&path, &glow_p, &Stroke { width: 3.5, ..Default::default() }, Transform::identity(), None);

                // Thin bright core stroke
                let mut core_p = Paint::default();
                core_p.anti_alias = true;
                core_p.set_color_rgba8(cr, cg, cb, 240);
                pixmap.stroke_path(&path, &core_p, &Stroke { width: 1.2, ..Default::default() }, Transform::identity(), None);
            }
        }
    }

    /// 8. Taiji Core (Thái Cực Đồ - Yin Yang Core)
    fn draw_taiji(&self, pixmap: &mut Pixmap, cx: f32, cy: f32, max_r: f32, theme: &RealmTheme) {
        let (pr, pg, pb) = theme.primary();
        let (sr, sg, sb) = theme.secondary();
        let (cr, cg, cb) = theme.core();

        let r_taiji = max_r * 0.16;

        // Breathing aura pulse (Linh tức hô hấp)
        let breath = (self.pulse_phase * 2.5).sin() * 0.5 + 0.5;
        let glow_r = r_taiji * (1.6 + 0.35 * breath);

        // Core Radial Gradient Halo
        let mut halo_paint = Paint::default();
        halo_paint.anti_alias = true;
        if let Some(shader) = RadialGradient::new(
            Point::from_xy(cx, cy),
            0.0,
            Point::from_xy(cx, cy),
            glow_r,
            vec![
                GradientStop::new(0.0, Color::from_rgba8(cr, cg, cb, 200)),
                GradientStop::new(0.4, Color::from_rgba8(pr, pg, pb, 110)),
                GradientStop::new(1.0, Color::from_rgba8(pr, pg, pb, 0)),
            ],
            SpreadMode::Pad,
            Transform::identity(),
        ) {
            halo_paint.shader = shader;
            let mut hb = PathBuilder::new();
            hb.push_circle(cx, cy, glow_r);
            if let Some(path) = hb.finish() {
                pixmap.fill_path(&path, &halo_paint, FillRule::Winding, Transform::identity(), None);
            }
        }

        let transform = Transform::from_translate(cx, cy).pre_rotate(self.angle_taiji.to_degrees());

        // Full circle base (Yang side - primary radiant color)
        let mut base_b = PathBuilder::new();
        base_b.push_circle(0.0, 0.0, r_taiji);
        if let Some(path) = base_b.finish() {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color_rgba8(pr, pg, pb, 220);
            pixmap.fill_path(&path, &paint, FillRule::Winding, transform, None);
        }

        // Yin side S-curve:
        // Top half-circle (radius R/2 centered at (0, -R/2)),
        // Bottom half-circle subtracted (radius R/2 centered at (0, R/2)),
        // Outer arc (radius R from (0, R) to (0, -R)).
        let mut yin_b = PathBuilder::new();
        yin_b.move_to(0.0, -r_taiji);

        // Inner top arc down to center
        let steps = 18;
        let half_r = r_taiji * 0.5;
        for s in 0..=steps {
            let a = -PI * 0.5 + PI * (s as f32 / steps as f32);
            let x = a.cos() * half_r;
            let y = -half_r + a.sin() * half_r;
            yin_b.line_to(x, y);
        }

        // Inner bottom arc down to bottom
        for s in 0..=steps {
            let a = -PI * 0.5 - PI * (s as f32 / steps as f32);
            let x = a.cos() * half_r;
            let y = half_r + a.sin() * half_r;
            yin_b.line_to(x, y);
        }

        // Outer circular return arc from bottom (0, R) back to top (0, -R)
        for s in 0..=steps {
            let a = PI * 0.5 + PI * (s as f32 / steps as f32);
            let x = a.cos() * r_taiji;
            let y = a.sin() * r_taiji;
            yin_b.line_to(x, y);
        }
        yin_b.close();

        if let Some(path) = yin_b.finish() {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color_rgba8(sr / 3 + 10, sg / 3 + 12, sb / 3 + 20, 235); // Dark contrast Yin
            pixmap.fill_path(&path, &paint, FillRule::Winding, transform, None);
        }

        // Fish Eyes (Âm Dương Nhãn)
        let eye_r = r_taiji * 0.17;
        // Top Eye (in Yang area, colored Dark Yin)
        let mut eye_top = PathBuilder::new();
        eye_top.push_circle(0.0, -half_r, eye_r);
        if let Some(path) = eye_top.finish() {
            let mut p = Paint::default();
            p.anti_alias = true;
            p.set_color_rgba8(sr / 3 + 10, sg / 3 + 12, sb / 3 + 20, 245);
            pixmap.fill_path(&path, &p, FillRule::Winding, transform, None);
        }

        // Bottom Eye (in Yin area, colored Bright Core)
        let mut eye_bot = PathBuilder::new();
        eye_bot.push_circle(0.0, half_r, eye_r);
        if let Some(path) = eye_bot.finish() {
            let mut p = Paint::default();
            p.anti_alias = true;
            p.set_color_rgba8(cr, cg, cb, 255);
            pixmap.fill_path(&path, &p, FillRule::Winding, transform, None);
        }

        // Outer ring perimeter of Taiji
        let mut ring_b = PathBuilder::new();
        ring_b.push_circle(0.0, 0.0, r_taiji);
        if let Some(path) = ring_b.finish() {
            let mut paint = Paint::default();
            paint.anti_alias = true;
            paint.set_color_rgba8(cr, cg, cb, 210);
            pixmap.stroke_path(&path, &paint, &Stroke { width: 1.5, ..Default::default() }, transform, None);
        }
    }

    /// Helper to draw a glowing diamond node
    fn draw_diamond(&self, pixmap: &mut Pixmap, x: f32, y: f32, radius: f32, color: Color) {
        let mut b = PathBuilder::new();
        b.move_to(x, y - radius);
        b.line_to(x + radius, y);
        b.line_to(x, y + radius);
        b.line_to(x - radius, y);
        b.close();

        if let Some(path) = b.finish() {
            let mut p = Paint::default();
            p.anti_alias = true;
            p.set_color(color);
            pixmap.fill_path(&path, &p, FillRule::Winding, Transform::identity(), None);
        }
    }
}
