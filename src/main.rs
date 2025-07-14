use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.circle().overlaps_rect(&other.rect())
    }
    fn circle(&self) -> Circle {
        Circle {
            x: (self.x),
            y: (self.y),
            r: (self.size),
        }
    }
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("My game")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 500.0;
    rand::srand(miniquad::date::now() as u64);

    let mut gameover = false;

    let mut circle = Shape {
        size: 16.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
    };
    let mut squares = vec![];
    let squares_colors: Vec<Color> = vec![
        LIGHTGRAY, GRAY, DARKGRAY, GOLD, ORANGE, PINK, RED, MAROON, GREEN, LIME, DARKGREEN,
        SKYBLUE, DARKBLUE, PURPLE, VIOLET, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, BLACK,
        BLANK, MAGENTA,
    ];

    let font = load_ttf_font("./assets/test.ttf").await.unwrap();

    loop {
        let delta_time = get_frame_time(); // temps passé depuis la dernière frame
        clear_background(BLUE);
        // dessin du cercle
        if !gameover {
            if is_key_down(KeyCode::Right) {
                circle.x += circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += circle.speed * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= circle.speed * delta_time;
            }
            // on s'assure qu'on ne déborde pas de l'écran
            circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
            circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);
        }
        draw_circle(circle.x, circle.y, circle.size, YELLOW);

        if !gameover {
            // dessin des carrés : 5% de chance d'avoir un nouveau carré
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: squares_colors.choose().copied().unwrap(),
                });
            }
            // on les fait tomber
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            squares.retain(|square| square.y < screen_height() + square.size); // on vire les carrés hors écran
        }
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }
        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
            let text = "GAME OVER!";
            let text_params = TextParams {
                font_size: 50,
                font: Some(&font),
                color: RED,
                font_scale: 1.0,
                ..Default::default()
            };
            let text_dimensions = measure_text(
                text,
                text_params.font,
                text_params.font_size,
                text_params.font_scale,
            );
            draw_text_ex(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0 + text_dimensions.height / 2.0,
                text_params,
            );
        }
        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }
        next_frame().await
    }
}
