use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

use crate::suborbital;

use super::orbital::{EnergyLevel, SubOrbital};

/// Representation of electron configuration using StaticVec
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronConfiguration([EnergyLevel; 8]);

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

    pub fn shells(&self) -> Vec<EnergyLevel> {
        let n = self
            .0
            .iter()
            .find(|&e| e.s.electrons() + e.p.electrons() + e.d.electrons() + e.f.electrons() == 0)
            .map(|e| e.quantum_number() - 1)
            .unwrap_or(8);

        self.0[0..(n as usize)].to_vec()
    }
}

impl ToTokens for ElectronConfiguration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let orbitals = self.0;

        let add_tokens = quote! {
            chemistru_elements::data::electron::configuration::ElectronConfiguration::new([#(#orbitals),*])
        };

        tokens.append(TokenTree::Group(Group::new(Delimiter::None, add_tokens)));
    }
}
