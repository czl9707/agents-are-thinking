pub mod effect;
pub mod frame;
pub mod effects;

#[cfg(test)]
mod tests {
    use crate::effects::all_effects;

    #[test]
    fn test_registry_count() {
        let effects = all_effects();
        assert_eq!(effects.len(), 48, "Expected 48 effects in registry");
    }

    #[test]
    fn test_all_effects_step() {
        let mut effects = all_effects();
        for eff in &mut effects {
            let frame = eff.step();
            assert!(!frame.is_empty(), "Effect {} produced empty frame", eff.name());
        }
    }

    #[test]
    fn test_effect_names_unique() {
        let effects = all_effects();
        let mut names: Vec<&str> = effects.iter().map(|e| e.name()).collect();
        names.sort();
        let len_before = names.len();
        names.dedup();
        assert_eq!(len_before, names.len(), "Duplicate effect names found");
    }

    const RNG_EFFECTS: &[&str] = &[
        "braille-random",
        "braille-bounce",
        "braille-dissolve",
        "braille-fire",
        "braille-scanner",
        "braille-matrix",
        "vblock-rain",
    ];

    #[test]
    fn test_deterministic_effects_cycle_loops() {
        let mut effects = all_effects();
        for eff in &mut effects {
            if RNG_EFFECTS.contains(&eff.name()) {
                continue;
            }
            let first = eff.step();
            let cycle = eff.cycle_length();
            for _ in 1..cycle {
                eff.step();
            }
            let looped = eff.step();
            assert_eq!(
                first, looped,
                "Effect {} doesn't loop cleanly after {} frames",
                eff.name(), cycle
            );
        }
    }

    #[test]
    fn test_stateful_effects_run_full_cycle() {
        let mut effects = all_effects();
        for eff in &mut effects {
            if !RNG_EFFECTS.contains(&eff.name()) {
                continue;
            }
            let cycle = eff.cycle_length();
            for _ in 0..cycle {
                let frame = eff.step();
                assert!(!frame.is_empty(), "Stateful effect {} produced empty frame", eff.name());
            }
        }
    }
}
