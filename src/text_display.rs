use macroquad::prelude::*;

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
