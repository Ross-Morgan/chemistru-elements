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
    #[inline]
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    #[inline]
    pub fn symbol(&self) -> String {
        self.symbol.to_string()
    }

    #[inline]
    pub const fn atomic_mass(&self) -> f64 {
        self.atomic_mass
    }

    #[inline]
    pub const fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    pub const fn data(&self) -> &'static InnerElement {
        self.raw
    }

    #[inline]
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

pub trait Elemental {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn atomic_mass(&self) -> f64;
    fn atomic_number(&self) -> u8;
    fn data(&self) -> &'static InnerElement;
}

impl Elemental for Element {
    #[inline(always)]
    fn name(&self) -> String {
        self.name()
    }

    #[inline(always)]
    fn symbol(&self) -> String {
        self.symbol()
    }
    
    #[inline(always)]
    fn atomic_mass(&self) -> f64 {
        self.atomic_mass()
    }
    
    #[inline(always)]
    fn atomic_number(&self) -> u8 {
        self.atomic_number()
    }
    
    #[inline(always)]
    fn data(&self) -> &'static InnerElement {
        self.data()
    }
}