use eframe::egui::{self, Color32, Pos2, Rect, Vec2};
use rand::Rng;

struct Particle {
    pos: Pos2,
    vel: Vec2,
    life: f32,     // 0.0 to 1.0
    max_life: f32, // Seconds
    size: f32,
    color: Color32,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn update(&mut self, dt: f32, screen_rect: Rect) {
        let mut rng = rand::rng();

        // Spawn new particles
        if self.particles.len() < self.max_particles {
            // Spawn rate
            if rng.random_bool(0.1) {
                let x = rng.random_range(screen_rect.min.x..screen_rect.max.x);
                let y = rng.random_range(screen_rect.min.y..screen_rect.max.y);
                
                let speed = rng.random_range(5.0..20.0);
                let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
                
                self.particles.push(Particle {
                    pos: Pos2::new(x, y),
                    vel: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                    life: 1.0,
                    max_life: rng.random_range(2.0..5.0),
                    size: rng.random_range(1.0..3.0),
                    color: if rng.random_bool(0.5) {
                        Color32::from_rgb(0, 255, 255) // Cyan
                    } else {
                        Color32::from_rgb(255, 0, 128) // Magenta
                    },
                });
            }
        }

        // Update existing
        for p in &mut self.particles {
            p.pos += p.vel * dt;
            p.life -= dt / p.max_life;
        }

        // Remove dead
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn draw(&self, painter: &eframe::egui::Painter) {
        for p in &self.particles {
            let opacity = (p.life * 255.0) as u8;
            let color = Color32::from_rgba_premultiplied(
                p.color.r(), p.color.g(), p.color.b(), opacity
            );
            
            painter.circle_filled(p.pos, p.size, color);
        }
    }
}

// Click Effect
pub struct ClickRipple {
    pub pos: Pos2,
    pub time: f32, // Seconds alive
    pub max_time: f32,
}

impl ClickRipple {
    pub fn new(pos: Pos2) -> Self {
        Self { pos, time: 0.0, max_time: 0.5 }
    }
}

pub fn draw_click_effects(painter: &eframe::egui::Painter, ripples: &mut Vec<ClickRipple>, dt: f32) {
    for r in ripples.iter_mut() {
        r.time += dt;
        let progress = r.time / r.max_time; // 0.0 to 1.0
        
        if progress < 1.0 {
            let size = progress * 50.0;
            let alpha = ((1.0 - progress) * 255.0) as u8;
            let color = Color32::from_rgba_premultiplied(0, 255, 255, alpha); // Cyan ripple
            
            // Pixelated square ripple
            let rect = Rect::from_center_size(r.pos, Vec2::splat(size));
            painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, color), eframe::egui::StrokeKind::Middle);
        }
    }
    
    ripples.retain(|r| r.time < r.max_time);
}
