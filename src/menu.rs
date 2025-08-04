/* Structure Menu (gestion du menu) */

use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui};

pub struct Menu {
    pub window_size: Vec2,
}

impl Menu {
    pub async fn new() -> Menu {
        let window_background = load_image("window_background.png").await.unwrap();
        let button_background = load_image("button_background.png").await.unwrap();
        let button_clicked_background = load_image("button_clicked_background.png").await.unwrap();
        let font = load_file("atari_games.ttf").await.unwrap();
        let window_style = root_ui()
            .style_builder()
            .background(window_background)
            .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
            .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
            .build();
        let button_style = root_ui()
            .style_builder()
            .background(button_background)
            .background_clicked(button_clicked_background)
            .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
            .font(&font)
            .unwrap()
            .text_color(WHITE)
            .font_size(64)
            .build();
        let label_style = root_ui()
            .style_builder()
            .font(&font)
            .unwrap()
            .text_color(WHITE)
            .font_size(28)
            .build();
        let ui_skin = Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&ui_skin);
        let window_size: Vec2 = vec2(370.0, 320.0);

        Menu { window_size }
    }

    pub fn display<F>(&mut self, play: &mut F)
    where
        F: FnMut(),
    {
        root_ui().window(
            hash!(),
            vec2(
                screen_width() / 2.0 - self.window_size.x / 2.0,
                screen_height() / 2.0 - self.window_size.y / 2.0,
            ),
            self.window_size,
            |ui| {
                ui.label(vec2(80.0, -34.0), "Main Menu");
                if ui.button(vec2(65.0, 25.0), "Play") {
                    play();
                }
                if ui.button(vec2(65.0, 125.0), "Quit") {
                    std::process::exit(0);
                }
            },
        );
    }
}
