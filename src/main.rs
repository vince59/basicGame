use macroquad::prelude::*;
#[macroquad::main("My game")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    const MOVEMENT_SPEED: f32 = 200.0;
    
    loop {
        let delta_time = get_frame_time(); // temps passé depuis la dernière frame
        clear_background(BLUE);
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        // on s'assure qu'on ne déborde pas de l'écran
        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());
        draw_circle(x, y, 16.0, YELLOW);
        next_frame().await
    }
}
