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
        text_params.clone(),
    );
    let text = "Press LeftShift to continue ...";
    draw_text_ex(
        text,
        screen_width() / 2.0 - text_dimensions.width,
        screen_height() / 2.0 + text_dimensions.height*2.0,
        text_params.clone(),
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
