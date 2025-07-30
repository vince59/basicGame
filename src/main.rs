use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad_particles::{self as particles, ColorCurve, Emitter, EmitterConfig};

//https://vince59.github.io/basicGame/

pub struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
    collided: bool,
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

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

pub fn particle_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: RED,
            mid: ORANGE,
            end: RED,
        },
        ..Default::default()
    }
}

pub fn display_game_over(font: &Font) {
    let text = "GAME OVER!";
    let text_params = TextParams {
        font_size: 50,
        font: Some(font),
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

pub fn display_congratulations(font: &Font) {
    let text = "CONGRATULATIONS ! you reached a high score";
    let text_params = TextParams {
        font_size: 25,
        font: Some(font),
        color: YELLOW,
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
        screen_height() / 2.0 + text_dimensions.height / 2.0 + 60.0,
        text_params,
    );
}

pub fn display_score(score: &u32, high_score: &u32) {
    draw_text(
        format!("Score: {}", score).as_str(),
        10.0,
        35.0,
        25.0,
        WHITE,
    );
    let highscore_text = format!("High score: {}", high_score);
    let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
    draw_text(
        highscore_text.as_str(),
        screen_width() - text_dimensions.width - 10.0,
        35.0,
        25.0,
        WHITE,
    );
}

pub fn display_squares(squares: &Vec<Shape>) {
    for square in squares {
        draw_rectangle(
            square.x - square.size / 2.0,
            square.y - square.size / 2.0,
            square.size,
            square.size,
            square.color,
        );
    }
}

pub fn display_bullets(bullets: &Vec<Shape>) {
    for bullet in bullets {
        draw_circle(bullet.x, bullet.y, bullet.size, bullet.color);
    }
}

pub fn display_paused() {
    let text = "Paused";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0,
        50.0,
        WHITE,
    );
}

pub fn display_game_name() {
    let text = "Asteroïd";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        text_dimensions.height + 10.0,
        50.0,
        YELLOW,
    );
}

pub fn display_press_space() {
    let text = "Press space";
    let text_dimensions = measure_text(text, None, 50, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_dimensions.width / 2.0,
        screen_height() / 2.0,
        50.0,
        WHITE,
    );
}
#[macroquad::main("Astéroïd")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 500.0;
    rand::srand(miniquad::date::now() as u64);

    let mut game_state = GameState::MainMenu;

    let mut circle = Shape {
        size: 16.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
        collided: false,
    };
    let mut squares: Vec<Shape> = vec![];
    let mut bullets: Vec<Shape> = vec![];

    let squares_colors: Vec<Color> = vec![
        LIGHTGRAY, GRAY, DARKGRAY, GOLD, ORANGE, PINK, RED, MAROON, GREEN, LIME, DARKGREEN,
        SKYBLUE, DARKBLUE, PURPLE, VIOLET, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, BLACK,
        BLANK, MAGENTA,
    ];

    let font = load_ttf_font("./assets/test.ttf").await.unwrap();

    let mut score: u32 = 0;
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let mut high_score: u32 = storage
        .get("highscore")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    let texture: Texture2D = load_texture("assets/chess.png").await.unwrap();

    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);

    let material = load_material(
        ShaderSource::Glsl {
            vertex: LENS_VERTEX_SHADER,
            fragment: LENS_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Center", UniformType::Float2),
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(BLACK);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                    explosions.clear();
                }
                display_press_space();
            }
            GameState::Playing => {
                let delta_time = get_frame_time(); // temps passé depuis la dernière frame
                display_score(&score, &high_score);
                // dessin du cercle
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
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        x: circle.x,
                        y: circle.y,
                        speed: circle.speed * 2.0,
                        size: 5.0,
                        collided: false,
                        color: RED,
                    });
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // on dessine les balles
                display_bullets(&bullets);

                // on dessine le cercle
                draw_circle(circle.x, circle.y, circle.size, YELLOW);

                // ajout des carrés : 5% de chance d'avoir un nouveau carré
                if rand::gen_range(0, 99) >= 95 {
                    let size = rand::gen_range(16.0, 64.0);
                    squares.push(Shape {
                        size,
                        speed: rand::gen_range(50.0, 150.0),
                        x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                        y: -size,
                        color: squares_colors.choose().copied().unwrap(),
                        collided: false,
                    });
                }
                // on les fait tomber
                for square in &mut squares {
                    square.y += square.speed * delta_time;
                }
                squares.retain(|square| square.y < screen_height() + square.size); // on vire les carrés hors écran

                // on déplace les balles
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                // pour tous les carrés pour toutes les balles on regarde s'il y a une collision
                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: square.size.round() as u32 * 2,
                                    ..particle_explosion()
                                }),
                                vec2(square.x, square.y),
                            ));
                        }
                    }
                }

                bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0); // on vire les balles hors écran
                squares.retain(|square| !square.collided); // on vire les carrés touché
                bullets.retain(|bullet| !bullet.collided); // on vire les balles touchées
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // on dessine les carrés
                display_squares(&squares);
                for (explosion, coords) in explosions.iter_mut() {
                    explosion.draw(*coords);
                }

                // test de collison entre les carrés et le cercle
                // affichage de game over si collison
                if squares.iter().any(|square| circle.collides_with(square)) {
                    game_state = GameState::GameOver;
                }
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                display_squares(&squares);
                draw_circle(circle.x, circle.y, circle.size, YELLOW);
                display_score(&score, &high_score);
                display_bullets(&bullets);
                display_paused();
                display_game_name();
            }
            GameState::GameOver => {
                // Redémarrage du jeu si on presse espace
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                display_game_over(&font);
                if score == high_score {
                    let s = high_score.to_string();
                    storage.set("highscore", &s);
                    display_congratulations(&font);
                }
            }
        }
        next_frame().await
    }
}

const LENS_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;
varying vec2 center;

uniform sampler2D _ScreenTexture;

void main() {
    float gradient = length(uv);
    vec2 uv_zoom = (uv_screen - center) * gradient + center;

    gl_FragColor = texture2D(_ScreenTexture, uv_zoom);
}
"#;

const LENS_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 center;
varying lowp vec2 uv;
varying lowp vec2 uv_screen;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec2 Center;

void main() {
    vec4 res = Projection * Model * vec4(position, 1);
    vec4 c = Projection * Model * vec4(Center, 0, 1);

    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    center = c.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;

    gl_Position = res;
}
";
