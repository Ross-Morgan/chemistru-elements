pub mod orbital;

use quote::quote;
use staticvec::StaticVec;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronData {
    pub electron_configuration: ElectronConfiguration,
    pub ionisation_energies: StaticVec<f64, 30>,
}

/// Representation of electron configuration using StaticVec
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronConfiguration(StaticVec<orbital::Orbital, 6>);

impl ToTokens for ElectronData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let electron_configuration = self.electron_configuration;
        let ionisation_energies = self.ionisation_energies.iter();

        let add_tokens = quote! {
            chemistru_elements::raw::ElectronData {
                electron_configuration: #electron_configuration,
                ionisation_energies: staticvec::StaticVec::new_from_slice(&[#(#ionisation_energies),*]),
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
        let slice = slice_to_string().parse::<proc_macro2::TokenStream>().unwrap();

        let add_tokens = quote! {
            chemistru_elements::data::ElectronConfiguration(staticvec::StaticVec::new_from_slice(&[#(#orbital),*]))
        };
    }
}
