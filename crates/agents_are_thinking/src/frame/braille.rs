const DOT_MAP: [[u8; 2]; 4] = [
    [0x01, 0x08],
    [0x02, 0x10],
    [0x04, 0x20],
    [0x40, 0x80],
];

pub struct BrailleFrame {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl BrailleFrame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let col = x / 2;
        let dot_col = x % 2;
        let row = y / 4;
        let dot_row = y % 4;
        let idx = row * self.width + col;
        if idx < self.cells.len() {
            self.cells[idx] |= DOT_MAP[dot_row][dot_col];
        }
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        let col = x / 2;
        let dot_col = x % 2;
        let row = y / 4;
        let dot_row = y % 4;
        let idx = row * self.width + col;
        if idx < self.cells.len() {
            self.cells[idx] &= !DOT_MAP[dot_row][dot_col];
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(0);
    }

    pub fn render(&self) -> Vec<String> {
        let mut rows = Vec::with_capacity(self.height);
        for r in 0..self.height {
            let start = r * self.width;
            let row: String = (0..self.width)
                .map(|c| {
                    let val = self.cells[start + c] & 0xFF;
                    char::from_u32(0x2800 + val as u32).unwrap()
                })
                .collect();
            rows.push(row);
        }
        rows
    }
}
