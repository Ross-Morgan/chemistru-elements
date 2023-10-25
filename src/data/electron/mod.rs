pub mod orbital;

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt, ToTokens};

use orbital::Orbital;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronData {
    pub shells: [u8; 8],
    pub ionisation_energies: [f64; 30],
    pub electron_configuration: ElectronConfiguration,
    pub electron_affinity: Option<f64>,
    pub electronegativity: Option<f64>,
}

/// Representation of electron configuration using StaticVec
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronConfiguration(pub(crate) [Orbital; 8]);

impl ToTokens for ElectronData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let electron_configuration = &self.electron_configuration;
        let ionisation_energies = self.ionisation_energies;

        let ionisation_energies = slice_to_tokens(ionisation_energies);
        let shells = slice_to_tokens(self.shells);
        let electron_affinity = self.electron_affinity;
        let electronegativity = self.electronegativity;

        let add_tokens = quote! {
            chemistru_elements::raw::ElectronData {
                electron_configuration: #electron_configuration,
                ionisation_energies: #ionisation_energies,
                shells: #shells,
                electron_affinity: #electron_affinity,
                electronegativity: #electronegativity,
            }
        };

        let group = TokenTree::Group(Group::new(
            Delimiter::Brace,
            add_tokens,
        ));

        tokens.append(group);
    }
}


impl ToTokens for ElectronConfiguration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let orbitals = self.0;

        let add_tokens = quote! {
            chemistru_elements::data::ElectronConfiguration([#(#orbitals),*])
        };

        tokens.append(TokenTree::Group(Group::new(Delimiter::Brace, add_tokens)));
    }
}


fn slice_to_tokens<T: ToTokens, const N: usize> (s: [T; N]) -> TokenStream {
    let item = s.iter();

    quote!([#(#item),*])
}