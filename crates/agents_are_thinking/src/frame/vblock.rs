const VBLOCKS: &[char] = &[' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

pub struct VBlockFrame {
    width: usize,
    cells: Vec<f64>,
}

impl VBlockFrame {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            cells: vec![0.0; width],
        }
    }

    pub fn set(&mut self, x: usize, density: f64) {
        if x < self.width {
            self.cells[x] = density.clamp(0.0, 1.0);
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(0.0);
    }

    pub fn render(&self) -> Vec<String> {
        let row: String = (0..self.width)
            .map(|c| {
                let idx = (self.cells[c] * (VBLOCKS.len() - 1) as f64).round() as usize;
                let idx = idx.min(VBLOCKS.len() - 1);
                VBLOCKS[idx]
            })
            .collect();
        vec![row]
    }
}
