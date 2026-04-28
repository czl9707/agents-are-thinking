use pyo3::prelude::*;

#[pymodule]
#[allow(non_snake_case)]
mod _impl {
    use agents_are_thinking::effect::Effect as REffect;
    use pyo3::prelude::*;

    #[pyclass(subclass)]
    struct Effect {
        instance: Box<dyn REffect>,
    }

    #[pymethods]
    impl Effect {
        fn step(&mut self) -> String {
            self.instance.step()
        }

        fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<'_, Self>) -> String {
            return slf.instance.step()
        }
    }

    macro_rules! py_effect {
        ($cls:ident) => {
            #[pyclass(extends=Effect, subclass)]
            struct $cls {}

            #[pymethods]
            impl $cls {
                #[new]
                fn new() -> PyClassInitializer<Self> {
                    PyClassInitializer::from(Effect {
                        instance: Box::new(agents_are_thinking::effects::$cls::new()),
                    }).add_subclass($cls {})
                }

                #[classattr]
                fn name() -> String {
                    agents_are_thinking::effects::$cls::name().to_string()
                }

                #[classattr]
                fn description() -> String {
                    agents_are_thinking::effects::$cls::description().to_string()
                }

                #[classattr]
                fn cycle_length() -> PyResult<u8> {
                    Ok(agents_are_thinking::effects::$cls::cycle_length() as u8)
                }
            }
        };
    }

    py_effect!(BrailleSpin);
    py_effect!(BrailleSpin2);
    py_effect!(BrailleWave);
    py_effect!(BrailleRandom);
    py_effect!(BrailleBreathe);
    py_effect!(BrailleRipple);
    py_effect!(BrailleBounce);
    py_effect!(BrailleRain);
    py_effect!(BrailleZigzag);
    py_effect!(BrailleDissolve);
    py_effect!(BrailleFire);
    py_effect!(BrailleNoise);
    py_effect!(BrailleHeartbeat);
    py_effect!(BrailleArrow);
    py_effect!(BrailleScanner);
    py_effect!(BrailleMatrix);
    py_effect!(BrailleCheckerboard);
    py_effect!(BrailleCheckerboard2x2);

    py_effect!(ShadeWave);
    py_effect!(ShadeScanner);
    py_effect!(ShadeFire);
    py_effect!(ShadeRipple);
    py_effect!(ShadeBreathe);
    py_effect!(ShadeSeeSaw);
    py_effect!(ShadeBlink);
    py_effect!(ShadeLayers);
    py_effect!(ShadePinch);

    py_effect!(BarBounce);
    py_effect!(BarWave);
    py_effect!(BarSeeSaw);

    py_effect!(VBlockWave);
    py_effect!(VBlockFill);
    py_effect!(VBlockTide);
    py_effect!(VBlockBreathe);
    py_effect!(VBlockBounce);
    py_effect!(VBlockPulse);
    py_effect!(VBlockRipple);
    py_effect!(VBlockRain);
    py_effect!(VBlockCascade);

    py_effect!(SquarePulse);
    py_effect!(SquareFill);
    py_effect!(SquareBlink);
    py_effect!(SquareArrow);

    py_effect!(DotWave);
    py_effect!(DotPulse);
    py_effect!(DotHeartbeat);
    py_effect!(DotArrow);
    py_effect!(DotBounce);

    macro_rules! add_effects {
        ($m:expr, $py:expr, $out:expr, $cls:ident) => {
            $m.add_class::<$cls>()?;
            $out.push($py.get_type::<$cls>().unbind().into_any());
        };
    }

    #[pymodule_init]
    fn module_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let py = m.py();
        let mut effects: Vec<Py<PyAny>> = Vec::new();

        m.add_class::<Effect>()?;

        add_effects!(m, py, effects, BrailleSpin);
        add_effects!(m, py, effects, BrailleSpin2);
        add_effects!(m, py, effects, BrailleWave);
        add_effects!(m, py, effects, BrailleRandom);
        add_effects!(m, py, effects, BrailleBreathe);
        add_effects!(m, py, effects, BrailleRipple);
        add_effects!(m, py, effects, BrailleBounce);
        add_effects!(m, py, effects, BrailleRain);
        add_effects!(m, py, effects, BrailleZigzag);
        add_effects!(m, py, effects, BrailleDissolve);
        add_effects!(m, py, effects, BrailleFire);
        add_effects!(m, py, effects, BrailleNoise);
        add_effects!(m, py, effects, BrailleHeartbeat);
        add_effects!(m, py, effects, BrailleArrow);
        add_effects!(m, py, effects, BrailleScanner);
        add_effects!(m, py, effects, BrailleMatrix);
        add_effects!(m, py, effects, BrailleCheckerboard);
        add_effects!(m, py, effects, BrailleCheckerboard2x2);
        add_effects!(m, py, effects, ShadeWave);
        add_effects!(m, py, effects, ShadeScanner);
        add_effects!(m, py, effects, ShadeFire);
        add_effects!(m, py, effects, ShadeRipple);
        add_effects!(m, py, effects, ShadeBreathe);
        add_effects!(m, py, effects, ShadeSeeSaw);
        add_effects!(m, py, effects, ShadeBlink);
        add_effects!(m, py, effects, ShadeLayers);
        add_effects!(m, py, effects, ShadePinch);
        add_effects!(m, py, effects, BarBounce);
        add_effects!(m, py, effects, BarWave);
        add_effects!(m, py, effects, BarSeeSaw);
        add_effects!(m, py, effects, VBlockWave);
        add_effects!(m, py, effects, VBlockFill);
        add_effects!(m, py, effects, VBlockTide);
        add_effects!(m, py, effects, VBlockBreathe);
        add_effects!(m, py, effects, VBlockBounce);
        add_effects!(m, py, effects, VBlockPulse);
        add_effects!(m, py, effects, VBlockRipple);
        add_effects!(m, py, effects, VBlockRain);
        add_effects!(m, py, effects, VBlockCascade);
        add_effects!(m, py, effects, SquarePulse);
        add_effects!(m, py, effects, SquareFill);
        add_effects!(m, py, effects, SquareBlink);
        add_effects!(m, py, effects, SquareArrow);
        add_effects!(m, py, effects, DotWave);
        add_effects!(m, py, effects, DotPulse);
        add_effects!(m, py, effects, DotHeartbeat);
        add_effects!(m, py, effects, DotArrow);
        add_effects!(m, py, effects, DotBounce);

        m.add("EFFECTS", effects)?;
        m.add("WIDTH", 9)?;
        Ok(())
    }
}
