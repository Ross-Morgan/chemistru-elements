use proc_macro2::{TokenTree, Group, Delimiter};
use quote::{ToTokens, quote, TokenStreamExt};

/// An electron orbital, containing an S, P, D, and F block
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Orbital(pub SOrbital, pub POrbital, pub DOrbital, pub FOrbital);

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
    /// Orbital number that suborbital is contained in
    fn orbital_number(&self) -> u8;
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

macro_rules! impl_suborbital_block {
    ($($t:ty, $cap:literal, $block_letter:literal),+ $(,)?) => {
        $(
            impl SubOrbital for $t {
                fn orbital_number(&self) -> u8 { self.0 }
                fn electrons(&self) -> u8 { self.1 }
                fn capacity(&self) -> u8 { $cap }
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

impl ToTokens for Orbital {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let orbital_number = self.0.0;

        let s = self.0.1;
        let p = self.1.1;
        let d = self.2.1;
        let f = self.3.1;

        let add_tokens = quote! {
            chemistru_elements::data::electron::orbital::Orbital(
                chemistru_elements::data::electron::orbital::SOrbital(#orbital_number, #s),
                chemistru_elements::data::electron::orbital::POrbital(#orbital_number, #p),
                chemistru_elements::data::electron::orbital::DOrbital(#orbital_number, #d),
                chemistru_elements::data::electron::orbital::DOrbital(#orbital_number, #f),
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