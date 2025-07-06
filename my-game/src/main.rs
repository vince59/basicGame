use macroquad::prelude::*;
#[macroquad::main("My game")]
async fn main() {
    loop {
        clear_background(BLUE);
        next_frame().await
    }
}
