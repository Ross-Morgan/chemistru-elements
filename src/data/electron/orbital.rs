use bounded_integer::bounded_integer;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Orbital(SOrbital, POrbital, DOrbital, FOrbital);


pub trait SubOrbital {
    pub const CAPACITY: u8;

    pub fn electrons(&self) -> u8;

    pub fn electron_capcacity_pair(&self) -> (u8, u8) {
        (Self::electrons(&self), Self::CAPACITY)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SOrbital(u8, u8);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct POrbital(u8, u8);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DOrbital(u8, u8);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FOrbital(u8, u8);

impl SOrbital {
    pub const fn new(capcaity: u8, fullness: u8) -> Self {
        assert!(capcaity <= Self::CAPACITY);
        Self(capacity, fullness)
    }

    pub const fn into_inner(self) -> u8 { self.0 }
}

impl POrbital {
    pub const fn new(capcaity: u8, fullness: u8) -> Self {
        assert!(capcaity <= Self::CAPACITY);
        Self(capacity, fullness)
    }

    pub const fn into_inner(self) -> u8 { self.0 }
}

impl DOrbital {
    pub const fn new(capcaity: u8, fullness: u8) -> Self {
        assert!(capcaity <= Self::CAPACITY);
        Self(capacity, fullness)
    }

    pub const fn into_inner(self) -> u8 { self.0 }
}

impl FOrbital {
    pub const fn new(capcaity: u8, fullness: u8) -> Self {
        assert!(capcaity <= Self::CAPACITY);
        Self(capacity, fullness)
    }

    pub const fn into_inner(self) -> u8 { self.0 }
}

impl SubOrbital for SOrbital {
    const CAPACITY: u8 = 2;

    fn electrons(&self) -> u8 { self.0 }
}

impl SubOrbital for POrbital {
    const CAPACITY: u8 = 6;

    fn electrons(&self) -> u8 { self.0 }
}

impl SubOrbital for DOrbital {
    const CAPACITY: u8 = 10;

    fn electrons(&self) -> u8 { self.0 }
}

impl SubOrbital for FOrbital {
    const CAPACITY: u8 = 14;

    fn electrons(&self) -> u8 { self.0 }
}
