use code_selection::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Code selection".to_owned(),
        high_dpi: true,

        window_height: 800,
        window_width: 800,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::new(AreaSize::splat(128));

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        state.on_frame();

        next_frame().await;
    }
}
