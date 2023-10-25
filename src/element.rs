use crate::inner::InnerElement;

/// Small-ish Representation of an Element
#[derive(Copy, Debug, Clone)]
pub struct Element {
    name: &'static str,
    symbol: &'static str,
    atomic_mass: f64,
    atomic_number: u8,
    raw: &'static InnerElement,
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

    pub const fn data(&self) -> &'static InnerElement {
        self.raw
    }

    pub const fn new(
        name: &'static str,
        symbol: &'static str,
        atomic_mass: f64,
        atomic_number: u8,
        raw: &'static InnerElement,
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
