use crate::*;
use macroquad::prelude::*;

pub struct AppState {
    pub world: World,
    pub world_canvas: Image,
    pub world_texture: Texture2D,

    pub ticks_per_update: usize,

    pub is_paused: bool,
}

impl AppState {
    pub fn new(world_size: AreaSize) -> Self {
        let world = World::new(world_size);

        let image_size = world.get_image_size();
        let mut world_canvas =
            Image::gen_image_color(image_size.width as u16, image_size.height as u16, BLACK);

        world.draw_to_image(&mut world_canvas);

        let world_texture = Texture2D::from_image(&world_canvas);
        world_texture.set_filter(FilterMode::Nearest);

        Self {
            world,
            world_canvas,
            world_texture,
            ticks_per_update: 1,
            is_paused: true,
        }
    }

    pub fn reset(&mut self) {
        self.world = World::new(self.world.size);
    }

    pub fn on_frame(&mut self) {
        self.draw_world();
        self.draw_debug_text();

        self.handle_reset();
        self.handle_pause_switch();
        self.handle_tick_speed_selection();
        self.handle_ticks();
    }

    pub fn handle_reset(&mut self) {
        if is_key_pressed(KeyCode::R) {
            self.reset();
            self.update_texture();
        }
    }

    pub fn handle_ticks(&mut self) {
        if !self.is_paused || is_key_pressed(KeyCode::Space) {
            for _ in 0..self.ticks_per_update {
                self.world.tick();
            }
        }
    }

    pub fn handle_pause_switch(&mut self) {
        if is_key_pressed(KeyCode::P) {
            self.is_paused = !self.is_paused;
        }
    }

    pub fn update_texture(&mut self) {
        self.world.draw_to_image(&mut self.world_canvas);
        self.world_texture.update(&self.world_canvas);
    }

    pub fn draw_world(&mut self) {
        if !self.is_paused {
            self.update_texture();
        }

        let width = screen_width();
        let height = screen_height();

        let mut cell_in_pixels = height / self.world_canvas.height as f32;

        let mut scaled_width = self.world_canvas.width as f32 * cell_in_pixels;
        let mut scaled_height = height;
        if scaled_width > width {
            cell_in_pixels = width / self.world_canvas.width as f32;
            scaled_width = width;
            scaled_height = self.world_canvas.height as f32 * cell_in_pixels;
        }

        draw_texture_ex(
            &self.world_texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: vec2(scaled_width, scaled_height).into(),
                ..Default::default()
            },
        );
    }

    pub fn handle_tick_speed_selection(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.ticks_per_update += 1;
        }

        if is_key_pressed(KeyCode::Down) {
            self.ticks_per_update = self.ticks_per_update.saturating_sub(1).max(1);
        }
    }

    pub fn draw_debug_text(&self) {
        let x = 10.0;
        let text_size = 16.0;
        let text_color = WHITE;
        let shadow_color = BLACK;

        let mut y = 00.0;
        macro_rules! draw_text {
            ($($arg:tt)*) => {{
                y += 20.0;
                let shadow_offset = 1.0;

                let text = format!($($arg)*);
                draw_text(&text, x + shadow_offset, y + shadow_offset, text_size, shadow_color);
                draw_text(&text, x - shadow_offset, y + shadow_offset, text_size, shadow_color);
                draw_text(&text, x + shadow_offset, y - shadow_offset, text_size, shadow_color);
                draw_text(&text, x - shadow_offset, y - shadow_offset, text_size, shadow_color);
                draw_text(&text, x, y, text_size, text_color);
            }};
        }

        let fps = get_fps();
        draw_text!("FPS: {fps}");

        draw_text!(
            "Ticks per update (up/down to change): {}",
            self.ticks_per_update
        );
    }
}
