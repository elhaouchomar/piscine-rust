#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut fields = [self.r, self.g, self.b, self.a];

        for i in 0..4 {
            if fields[i] == first {
                for j in 0..4 {
                    if fields[j] == second {
                        fields.swap(i, j);
                        break;
                    }
                }
                break;
            }
        }

        self.r = fields[0];
        self.g = fields[1];
        self.b = fields[2];
        self.a = fields[3];

        self
    }
}
