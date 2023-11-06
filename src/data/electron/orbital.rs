use std::ops::{Range, RangeInclusive};

use proc_macro2::{Delimiter, Group, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

/// An electron orbital, containing an S, P, D, and F block
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnergyLevel(pub SOrbital, pub POrbital, pub DOrbital, pub FOrbital);

/// Suborbital containing up to 2 electrons
///
/// S-block elements are in groups 1 and 2
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SOrbital(pub u8, pub u8);

/// Suborbital containing up to 6 electrons
///
/// P-block elements are in groups 13-18
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct POrbital(pub u8, pub u8);

/// Suborbital containing up to 10 electrons
///
/// D-block elements are the transition metals (groups 3-12)
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DOrbital(pub u8, pub u8);

/// Suborbital containing up to 14 electrons
///
/// F-block elements are the actinides and lathinides
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FOrbital(pub u8, pub u8);

/// Trait containing methods relating to fullness, capacity and orbital number
pub trait SubOrbital {
    /// Number of energy level
    fn quantum_number(&self) -> u8;
    /// Indicates which sublevel the suborbital is
    fn angular_momentum(&self) -> u8;
    /// The orbital the last electron is contained in
    fn magnetic_quantum_number(&self) -> i8;
    /// Spin of last electron in suborbital
    fn magnetic_spin_number(&self) -> f64;
    /// Number of electrons in suborbital
    fn electrons(&self) -> u8;
    /// Maximum number of electrons in suborbital
    fn capacity(&self) -> u8;
}

/// Supertrait of SubOrbital with associated capacity constant
///
/// Supertrait exists so that an associated constant can be defined while
/// allowing the subtrait to be converted into a &dyn Object
pub trait CapSubOrbital: SubOrbital {
    const CAPACITY: u8;
}

impl EnergyLevel {
    pub const fn quantum_number(&self) -> u8 {
        self.0 .1
    }

    pub const fn possible_angular_momenta(&self) -> Range<u8> {
        0..self.quantum_number()
    }

    pub const fn possible_magnetic_quantum_numbers(&self) -> RangeInclusive<i8> {
        (-(self.quantum_number() as i8))..=(self.quantum_number() as i8)
    }
}

macro_rules! impl_suborbital_block {
    ($($t:ty, $cap:literal, $block_letter:literal),+ $(,)?) => {
        $(
            impl SubOrbital for $t {
                fn quantum_number(&self) -> u8 { self.0 }

                fn electrons(&self) -> u8 { self.1 }

                fn capacity(&self) -> u8 { $cap }

                fn angular_momentum(&self) -> u8 { match $block_letter {
                    's' => 0,
                    'p' => 1,
                    'd' => 2,
                    'f' => 3,
                    _ => panic!("Invalid suborbital letter")
                }}

                fn magnetic_quantum_number(&self) -> i8 {
                    (self.electrons() % (self.capacity() / 2)) as i8 - self.angular_momentum() as i8
                }

                fn magnetic_spin_number(&self) -> f64 { match self.electrons().cmp(&(self.capacity() / 2)) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => 0.5,
                    std::cmp::Ordering::Greater => -0.5,
                }}
            }

            impl CapSubOrbital for $t {
                const CAPACITY: u8 = $cap;
            }

            impl $t {
                pub const fn new(number: u8, fullness: u8) -> Self {
                    match fullness {
                        0..=Self::CAPACITY => Self(number, fullness),
                        _ => Self(number, 0)
                    }
                }
            }
        )*
    };
}

impl_suborbital_block! {
    SOrbital, 2,  's',
    POrbital, 6,  'p',
    DOrbital, 10, 'd',
    FOrbital, 14, 'f',
}

impl ToTokens for EnergyLevel {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let orbital_number = self.0 .1;
        let s = self.0 .1;
        let p = self.1 .1;
        let d = self.2 .1;
        let f = self.3 .1;

        let add_tokens = quote! {
            chemistru_elements::data::electron::orbital::EnergyLevel(
                chemistru_elements::data::electron::orbital::SOrbital(#orbital_number, #s),
                chemistru_elements::data::electron::orbital::POrbital(#orbital_number, #p),
                chemistru_elements::data::electron::orbital::DOrbital(#orbital_number, #d),
                chemistru_elements::data::electron::orbital::FOrbital(#orbital_number, #f),
            )
        };

        tokens.append(TokenTree::Group(Group::new(Delimiter::None, add_tokens)));
    }
}

#[macro_export]
macro_rules! suborbital {
    (s, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::SOrbital($number, $fullness)
    };
    (p, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::POrbital($number, $fullness)
    };
    (d, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::DOrbital($number, $fullness)
    };
    (f, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::FOrbital($number, $fullness)
    };
}
