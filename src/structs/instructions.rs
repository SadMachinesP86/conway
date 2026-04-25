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
                "Instructions: Click to add/remove 'cells' in order to set the starting population.".to_owned(),
                "[H] - Toggle the visibility of these instructions.".to_owned(),
                "[C] - Clear the population.".to_owned(),
                "[Enter] - Run the simulation.".to_owned(),
                "[Escape] - Quit.".to_owned(),
                "During the simulation:".to_owned(),
                "[Space] - Pause the simulation.".to_owned(),
                "[Escape] - End the simulation and return to the start.".to_owned(),
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
}
