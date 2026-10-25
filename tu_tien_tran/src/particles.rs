use std::f32::consts::PI;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Point, RadialGradient, SpreadMode, Stroke, Transform};
use crate::theme::RealmTheme;

#[derive(Clone, Debug)]
pub struct SpiritParticle {
    pub angle: f32,
    pub radius: f32,
    pub speed_angle: f32,
    pub radial_speed: f32,
    pub size: f32,
    pub alpha: f32,
    pub phase: f32,
    pub trail: Vec<(f32, f32)>,
}

pub struct ParticleSystem {
    particles: Vec<SpiritParticle>,
    max_radius: f32,
}

impl ParticleSystem {
    pub fn new(count: usize, max_radius: f32) -> Self {
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            let seed = i as f32;
            let angle = (seed * 137.5) % 360.0 * (PI / 180.0);
            let radius = 40.0 + ((seed * 73.0) % (max_radius - 50.0));
            let speed_angle = 0.3 + ((seed * 31.0) % 50.0) / 50.0 * 0.8;
            let dir = if i % 2 == 0 { 1.0 } else { -0.7 };
            let radial_speed = -12.0 - ((seed * 19.0) % 20.0); // spiral inward toward center
            let size = 1.5 + ((seed * 17.0) % 10.0) / 10.0 * 2.5;

            particles.push(SpiritParticle {
                angle,
                radius,
                speed_angle: speed_angle * dir,
                radial_speed,
                size,
                alpha: 0.8,
                phase: (seed * 1.3) % (2.0 * PI),
                trail: Vec::new(),
            });
        }

        Self {
            particles,
            max_radius,
        }
    }

    pub fn update(&mut self, dt: f32, speed_mult: f32, max_radius: f32) {
        self.max_radius = max_radius;
        for p in &mut self.particles {
            p.phase += dt * 3.0 * speed_mult;
            p.angle += p.speed_angle * dt * speed_mult;
            p.radius += p.radial_speed * dt * speed_mult;

            // Reset when reaching core
            if p.radius < 25.0 {
                p.radius = max_radius * (0.8 + 0.2 * (p.phase.sin() * 0.5 + 0.5));
                p.trail.clear();
            }

            // Current Cartesian position
            let x = p.angle.cos() * p.radius;
            let y = p.angle.sin() * p.radius;

            p.trail.push((x, y));
            if p.trail.len() > 6 {
                p.trail.remove(0);
            }
        }
    }

    pub fn draw(
        &self,
        pixmap: &mut Pixmap,
        cx: f32,
        cy: f32,
        theme: &RealmTheme,
    ) {
        let (pr, pg, pb) = theme.primary();
        let (cr, cg, cb) = theme.core();

        for p in &self.particles {
            let flicker = 0.6 + 0.4 * (p.phase.sin() * 0.5 + 0.5);
            let current_alpha = (p.alpha * flicker).clamp(0.0, 1.0);

            // Draw glowing trail
            if p.trail.len() >= 2 {
                let mut path_b = PathBuilder::new();
                let first = p.trail[0];
                path_b.move_to(cx + first.0, cy + first.1);
                for pt in p.trail.iter().skip(1) {
                    path_b.line_to(cx + pt.0, cy + pt.1);
                }
                if let Some(path) = path_b.finish() {
                    let mut trail_paint = Paint::default();
                    trail_paint.anti_alias = true;
                    trail_paint.set_color_rgba8(pr, pg, pb, (current_alpha * 70.0) as u8);
                    let stroke = Stroke {
                        width: p.size * 0.8,
                        ..Default::default()
                    };
                    pixmap.stroke_path(&path, &trail_paint, &stroke, Transform::identity(), None);
                }
            }

            // Draw particle glow orb
            let px = cx + p.angle.cos() * p.radius;
            let py = cy + p.angle.sin() * p.radius;
            let glow_radius = p.size * 3.5;

            let mut glow_paint = Paint::default();
            glow_paint.anti_alias = true;
            if let Some(shader) = RadialGradient::new(
                Point::from_xy(px, py),
                0.0,
                Point::from_xy(px, py),
                glow_radius,
                vec![
                    tiny_skia::GradientStop::new(0.0, Color::from_rgba8(cr, cg, cb, (current_alpha * 240.0) as u8)),
                    tiny_skia::GradientStop::new(0.4, Color::from_rgba8(pr, pg, pb, (current_alpha * 120.0) as u8)),
                    tiny_skia::GradientStop::new(1.0, Color::from_rgba8(pr, pg, pb, 0)),
                ],
                SpreadMode::Pad,
                Transform::identity(),
            ) {
                glow_paint.shader = shader;
                let mut circle_b = PathBuilder::new();
                circle_b.push_circle(px, py, glow_radius);
                if let Some(cpath) = circle_b.finish() {
                    pixmap.fill_path(&cpath, &glow_paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
                }
            }
        }
    }
}
