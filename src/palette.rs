pub struct Palette {
    data: Vec<[f32; 4]>
}

impl Palette {
    pub fn new(palette: Vec<[f32; 4]>) -> Palette {
        Palette {
            data: palette
        }
    }

    pub fn get_color(&self, idx: u8) -> [f32; 4] {
        self.data[(idx - 1) as usize]
    }
}
