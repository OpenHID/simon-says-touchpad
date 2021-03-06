use afi;

pub struct Game {
    input: afi::Input,
}

impl Game {
    pub fn new() -> Game {
        Game { input: afi::Input::new() }
    }
    pub fn update(&mut self) -> [f32; 4] {
        let input = self.input.update();

        // Do some stuff...
        let d = [if input[0] > 120. { 1. } else { 0. },
                 if input[1] > 120. { 1. } else { 0. },
                 if input[2] > 120. { 1. } else { 0. },
                 if input[3] > 120. { 1. } else { 0. }];
        d
    }
}

