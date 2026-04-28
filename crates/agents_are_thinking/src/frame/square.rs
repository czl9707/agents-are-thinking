const SQUARES: &[char] = &['·', '□', '■'];

pub struct SquareFrame {
    width: usize,
    cells: Vec<f64>,
}

impl SquareFrame {
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
                let idx = (self.cells[c] * (SQUARES.len() - 1) as f64).round() as usize;
                let idx = idx.min(SQUARES.len() - 1);
                SQUARES[idx]
            })
            .collect();
        vec![row]
    }
}
