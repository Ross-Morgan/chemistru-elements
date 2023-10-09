#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Element {
    name: &'static str,
    symbol: &'static str,
    atomic_mass: f64,
    atomic_number: u8,
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
    ) -> Self {
        Self {
            name,
            symbol,
            atomic_mass,
            atomic_number,
        }
    }
}
