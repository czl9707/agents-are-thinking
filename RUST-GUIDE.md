## Install

```bash
cargo add agents-are-thinking
```

## Basic usage

```rust
use std::thread;
use std::time::Duration;
use agents_are_thinking::effects::ShadeFire;
use agents_are_thinking::effect::Effect;

fn main() {
    let mut ef = ShadeFire::new();
    for _ in 0..50 {
        print!("\r{}", ef.step());
        thread::sleep(Duration::from_millis(100));
    }
}
```

## List all effects

```rust
use agents_are_thinking::effects::all_effects;
use agents_are_thinking::effect::Effect;

for ef in all_effects() {
    println!("{} - {} (cycle: {})", ef.name(), ef.description(), ef.cycle_length());
}
```

## API

Every effect implements the `Effect` trait:

```rust
pub trait Effect {
    fn step(&mut self) -> String;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn cycle_length(&self) -> usize;
}
```

Effects are in the `agents_are_thinking::effects` module. Use `all_effects()` to get a `Vec<Box<dyn Effect>>` of all 48.
