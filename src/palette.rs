pub struct Palette {
    data: Vec<[f32; 4]>
}

impl Palette {
    pub fn empty() -> Palette {
        Palette { data: Vec::new() }
    }

    pub fn get_color(&self, idx: u8) -> [f32; 4] {
        self.data[(idx - 1) as usize]
    }

    pub fn add_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> u8 {
        for (i, rgba) in self.data.iter().enumerate() {
            if rgba[0] == r && rgba[1] == g && rgba[2] == b && rgba[3] == a { // testing for equality on floats… why am i doing that to myself!?
                return (i + 1) as u8;
            }
        }
        self.data.push([r, g, b, a]);
        return self.data.len() as u8;
    }
}
