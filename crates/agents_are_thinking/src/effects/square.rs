use crate::effect::{Effect, EffectState, WIDTH, cycle_length, pause};
use crate::frame::SquareFrame;

pub struct SquarePulse {
    state: EffectState,
}

impl SquarePulse {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for SquarePulse {
    fn name() -> &'static str {
        "square-pulse"
    }
    fn description() -> &'static str {
        "Expanding ring in square characters"
    }
    fn cycle_length() -> usize {
        18
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl SquarePulse {
    fn render(&mut self) -> String {
        let full_cycle = WIDTH * 2;
        let mut f = SquareFrame::new(WIDTH);
        let cx = WIDTH as f64 / 2.0;
        let frame_mod = self.state.frame % full_cycle;
        let ring = if frame_mod < full_cycle / 2 {
            frame_mod
        } else {
            full_cycle - frame_mod - 1
        };
        for i in 0..WIDTH {
            let dist = (i as f64 - cx + 0.5).abs();
            let v = ((ring as f64 - dist + 1.0) / 2.0).clamp(0.0, 1.0);
            f.set(i, v);
        }
        f.render().join("\n")
    }
}

pub struct SquareFill {
    state: EffectState,
}

impl SquareFill {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for SquareFill {
    fn name() -> &'static str {
        "square-fill"
    }
    fn description() -> &'static str {
        "Progressive fill using square characters"
    }
    fn cycle_length() -> usize {
        35
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl SquareFill {
    fn render(&mut self) -> String {
        let speed: isize = 3;
        let cycle = WIDTH as isize * speed + 8;
        let mut f = SquareFrame::new(WIDTH);
        let pos = (self.state.frame % cycle as usize) as isize;
        for i in 0..WIDTH {
            let target = i as isize * speed;
            let elapsed = pos - target;
            if elapsed < 0 {
                f.set(i, 0.0);
            } else if elapsed < speed {
                f.set(i, 0.5);
            } else {
                f.set(i, 1.0);
            }
        }
        f.render().join("\n")
    }
}

pub struct SquareBlink {
    state: EffectState,
}

impl SquareBlink {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for SquareBlink {
    fn name() -> &'static str {
        "square-blink"
    }
    fn description() -> &'static str {
        "Blinking square levels"
    }
    fn cycle_length() -> usize {
        27
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl SquareBlink {
    fn render(&mut self) -> String {
        let period = cycle_length::SHORT;
        let mut f = SquareFrame::new(WIDTH);
        let offset = (self.state.frame / period) % 3;
        for i in 0..WIDTH {
            let level = (i + offset) % 3;
            f.set(i, level as f64 / 2.0);
        }
        f.render().join("\n")
    }
}

pub struct SquareArrow {
    state: EffectState,
}

impl SquareArrow {
    pub fn new() -> Self {
        Self {
            state: EffectState::new(42, Self::cycle_length()),
        }
    }
}

impl Effect for SquareArrow {
    fn name() -> &'static str {
        "square-arrow"
    }
    fn description() -> &'static str {
        "Arrow bouncing in square characters"
    }
    fn cycle_length() -> usize {
        34
    }

    fn step(&mut self) -> String {
        let result = self.render();
        self.state.advance();
        result
    }
}

impl SquareArrow {
    fn render(&mut self) -> String {
        let travel: isize = WIDTH as isize + 2;
        let paws: usize = pause::MEDIUM;
        let cycle = (travel * 2 + paws as isize) as usize;
        let mut f = SquareFrame::new(WIDTH);
        let frame = self.state.frame % cycle;
        let (head, direction): (isize, isize) = if frame < travel as usize {
            (-2 + frame as isize, 1)
        } else if frame < travel as usize * 2 {
            let f2 = frame as isize - travel;
            (WIDTH as isize - 1 - f2, -1)
        } else {
            return f.render().join("\n");
        };
        let empty = head;
        let solid1 = head + direction;
        let solid2 = head + direction * 2;
        for x in [solid1, solid2] {
            if x >= 0 && (x as usize) < WIDTH {
                f.set(x as usize, 1.0);
            }
        }
        if empty >= 0 && (empty as usize) < WIDTH {
            f.set(empty as usize, 0.5);
        }
        f.render().join("\n")
    }
}
