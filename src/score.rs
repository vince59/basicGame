use macroquad::prelude::*;

pub struct Score {
    score: u32,
    high_score: u32,
}

impl Score {
    pub fn new() -> Score {
        let score: u32 = 0;
        let storage = &mut quad_storage::STORAGE.lock().unwrap(); // Récupération du mutex
        let high_score: u32 = storage
            .get("highscore")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        Score { score, high_score }
    }

    pub fn display(&mut self) {
        draw_text(
            format!("Score: {}", self.score).as_str(),
            10.0,
            35.0,
            25.0,
            WHITE,
        );
        let highscore_text = format!("High score: {}", self.high_score);
        let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
        draw_text(
            highscore_text.as_str(),
            screen_width() - text_dimensions.width - 10.0,
            35.0,
            25.0,
            WHITE,
        );
    }

    pub fn increase(&mut self, value: u32) {
        self.score += value;
        self.high_score = self.high_score.max(self.score);
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }

    pub fn display_high_score(&mut self, font: &Font) {
        if self.score == self.high_score {
            let s = self.high_score.to_string();
            let storage = &mut quad_storage::STORAGE.lock().unwrap();
            storage.set("highscore", &s);
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
    }
}
