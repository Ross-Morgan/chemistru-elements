use proc_macro2::{TokenStream, TokenTree, Group, Delimiter};
use quote::{ToTokens, TokenStreamExt,  quote};

use crate::suborbital;

use super::orbital::EnergyLevel;

/// Representation of electron configuration using StaticVec
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronConfiguration(pub [EnergyLevel; 8]);

impl ElectronConfiguration {
    pub const fn new(levels: [EnergyLevel; 8]) -> Self {
        Self(levels)
    }

    pub fn new_empty() -> Self {
        let mut n = 0u8..;

        let levels = [(); 8].map(|_| {
            let q = n.next().expect("Infallible");

            EnergyLevel {
                s: suborbital!(s, q, 0),
                p: suborbital!(p, q, 0),
                d: suborbital!(d, q, 0),
                f: suborbital!(f, q, 0),
            }
        });

        Self(levels)
    }
}

impl ToTokens for ElectronConfiguration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let orbitals = self.0;

        let add_tokens = quote! {
            chemistru_elements::data::electron::ElectronConfiguration([#(#orbitals),*])
        };

        tokens.append(TokenTree::Group(Group::new(Delimiter::None, add_tokens)));
    }
}