use crate::raw::RawElement;

#[derive(Copy, Clone, Debug)]
pub struct Element {
    name: &'static str,
    symbol: &'static str,
    atomic_mass: f64,
    atomic_number: u8,
    raw: &'static RawElement,
}

impl Element {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn symbol(&self) -> String {
        self.symbol.to_string()
    }

    pub const fn atomic_mass(&self) -> f64 {
        self.atomic_mass
    }

    pub const fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    pub const fn new(
        name: &'static str,
        symbol: &'static str,
        atomic_mass: f64,
        atomic_number: u8,
        raw: &'static RawElement
    ) -> Self {
        Self {
            name,
            symbol,
            atomic_mass,
            atomic_number,
            raw,
        }
    }
}
