use std::ops::{Range, RangeInclusive};

use proc_macro2::{Delimiter, Group, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

/// An electron orbital, containing an S, P, D, and F block
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnergyLevel {
    pub s: SOrbital,
    pub p: POrbital,
    pub d: DOrbital,
    pub f: FOrbital,
}

/// Suborbital containing up to 2 electrons
///
/// S-block elements are in groups 1 and 2
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SOrbital(u8, u8);

/// Suborbital containing up to 6 electrons
///
/// P-block elements are in groups 13-18
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct POrbital(u8, u8);

/// Suborbital containing up to 10 electrons
///
/// D-block elements are the transition metals (groups 3-12)
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DOrbital(u8, u8);

/// Suborbital containing up to 14 electrons
///
/// F-block elements are the actinides and lathinides
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FOrbital(u8, u8);

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
    /// Mutable reference to electron count
    fn electrons_as_mut(&mut self) -> &mut u8;
}

/// Supertrait of SubOrbital with associated capacity constant
///
/// Supertrait exists so that an associated constant can be defined while
/// allowing the subtrait to be converted into a &dyn Object
pub trait CapSubOrbital: SubOrbital {
    const CAPACITY: u8;
    const ANGULAR_MOMENTUM: u8;
}

impl EnergyLevel {
    pub const fn quantum_number(&self) -> u8 {
        let s = self.s.0;
        let p = self.p.0;
        let d = self.d.0;
        let f = self.f.0;

        assert!(s == p);
        assert!(s == d);
        assert!(s == f);

        s
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

                fn magnetic_spin_number(&self) -> f64 {
                    match self.electrons().cmp(&(self.capacity() / 2)) {
                        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => 0.5,
                        std::cmp::Ordering::Greater => -0.5,
                    }
                }

                fn electrons_as_mut(&mut self) -> &mut u8 { &mut self.1 }
            }

            impl CapSubOrbital for $t {
                const CAPACITY: u8 = $cap;
                const ANGULAR_MOMENTUM: u8 = match $block_letter {
                    's' => 0,
                    'p' => 1,
                    'd' => 2,
                    'f' => 3,
                    _ => panic!("Invalid suborbital letter")
                };
            }

            impl $t {
                pub const fn new(number: u8, fullness: u8) -> Self {
                    match fullness {
                        0..=Self::CAPACITY => Self(number, fullness),
                        _ => Self(number, 0)
                    }
                }

                pub const fn is_empty(&self) -> bool {
                    self.1 == 0
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
        let n = self.quantum_number();
        let s = self.s.1;
        let p = self.p.1;
        let d = self.d.1;
        let f = self.f.1;

        let add_tokens: proc_macro2::TokenStream = quote! {
            chemistru_elements::data::electron::orbital::EnergyLevel {
                s: chemistru_elements::data::electron::orbital::SOrbital::new(#n, #s),
                p: chemistru_elements::data::electron::orbital::POrbital::new(#n, #p),
                d: chemistru_elements::data::electron::orbital::DOrbital::new(#n, #d),
                f: chemistru_elements::data::electron::orbital::FOrbital::new(#n, #f),
            }
        };

        tokens.append(TokenTree::Group(Group::new(Delimiter::None, add_tokens)));
    }
}

#[macro_export]
macro_rules! suborbital {
    (s, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::SOrbital::new($number, $fullness)
    };
    (p, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::POrbital::new($number, $fullness)
    };
    (d, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::DOrbital::new($number, $fullness)
    };
    (f, $number:expr, $fullness:expr) => {
        $crate::data::electron::orbital::FOrbital::new($number, $fullness)
    };
}
