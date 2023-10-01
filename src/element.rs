#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Element {
    name: ElementName,
    symbol: ElementSymbol,
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
        name: ElementName,
        symbol: ElementSymbol,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementName(pub [char; 13], pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ElementSymbol {
    OneDigit(char),
    TwoDigit(char, char),
    ThreeDigit(char, char, char),
}

impl ToString for ElementName {
    fn to_string(&self) -> String {
        if self.1 > 13 {
            panic!("Indicated name length too long (len {}) (max 13)", self.1)
        }

        self.0[0..self.1].into_iter().collect()
    }
}

impl ToString for ElementSymbol {
    fn to_string(&self) -> String {
        match self {
            ElementSymbol::OneDigit(c1) => format!("{c1}"),
            ElementSymbol::TwoDigit(c1, c2) => format!("{c1}{c2}"),
            ElementSymbol::ThreeDigit(c1, c2, c3) => format!("{c1}{c2}{c3}"),
        }
    }
}
