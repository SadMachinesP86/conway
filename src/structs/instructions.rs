use crate::consts::*;
use crate::{LIGHTGRAY, draw_text, measure_text, screen_height, screen_width};

pub struct Instructions {
    instructions: Vec<String>,
    font_size: f32,
    visibility: bool,
}

impl Instructions {
    pub fn default() -> Instructions {
        Instructions {
            instructions: vec![
                "Instructions: Set the starting population, then run the simulation.".to_owned(),
                "Use arrow keys to move the cursor. Press [Space] to flip the cell at the cursor."
                    .to_owned(),
                "Press [H] to toggle instructions.".to_owned(),
                "Press [Enter] to run the game. Press [Escape] to quit.".to_owned(),
            ],
            font_size: FONT_SIZE as f32,
            visibility: true,
        }
    }

    pub fn draw(&self) {
        if !self.visibility {
            return;
        }

        let mut offset = 0.;

        for text in self.instructions.iter() {
            let text_size = measure_text(text, None, self.font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2. + offset,
                self.font_size,
                LIGHTGRAY,
            );

            offset += self.font_size;
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.visibility = !self.visibility
    }

    pub fn hide(&mut self) {
        self.visibility = false
    }
}
