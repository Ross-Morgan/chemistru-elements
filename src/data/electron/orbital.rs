use bounded_integer::bounded_integer;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Orbital(SOrbital, POrbital, DOrbital, FOrbital);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SOrbital(u8, u8);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct POrbital(u8, u8);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DOrbital(u8, u8);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FOrbital(u8, u8);

pub trait SubOrbital {
    pub const CAPACITY: u8;

    pub fn orbital_number(&self) -> u8;
    pub fn electrons(&self) -> u8;
}

macro_rules! impl_suborbital_block {
    ($($t:ty, $cap:literal, $block_letter:literal),+ $(,)?) => {
        $(
            impl SubOrbital for $ty {
                const CAPACITY: u8 = $cap;

                fn orbital_number(&self) -> u8 { self.0 }
                fn electrons(&self) -> u8 { self.1 }
            }

            impl $ty {
                pub const fn new(number: u8, fullness: u8) -> Self {
                    match fullness {
                        0..Self::CAPACITY => Self(number, fullness),
                        _ => panic!("Specified more than {} electrons in a {} suborbital", Self::CAPACITY, $block_letter)
                    }
                }
            }
        )*
    };
}


impl_suborbital_block! {
    SBlock, 2,  's',
    PBlock, 6,  'p',
    DBlock, 10, 'd',
    FBlock, 14, 'f',
}
