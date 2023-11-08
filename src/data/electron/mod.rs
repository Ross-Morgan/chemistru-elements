pub mod configuration;
pub mod orbital;

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

use configuration::ElectronConfiguration;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronData {
    pub shells: [u8; 8],
    pub ionisation_energies: [f64; 30],
    pub electron_configuration: ElectronConfiguration,
    pub electron_affinity: Option<f64>,
    pub electronegativity: Option<f64>,
}

impl ToTokens for ElectronData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let electron_configuration = &self.electron_configuration;
        let ionisation_energies = self.ionisation_energies;

        let ionisation_energies = slice_to_tokens(ionisation_energies);
        let shells = slice_to_tokens(self.shells);
        let electron_affinity = match self.electron_affinity {
            Some(v) => quote!(Some(#v)),
            None => quote!(None),
        };

        let electronegativity = match self.electronegativity {
            Some(v) => quote!(Some(#v)),
            None => quote!(None),
        };

        let add_tokens = quote! {
            chemistru_elements::data::electron::ElectronData {
                electron_configuration: #electron_configuration,
                ionisation_energies: #ionisation_energies,
                shells: #shells,
                electron_affinity: #electron_affinity,
                electronegativity: #electronegativity,
            }
        };

        let group = TokenTree::Group(Group::new(Delimiter::None, add_tokens));

        tokens.append(group);
    }
}



fn slice_to_tokens<T: ToTokens, const N: usize>(s: [T; N]) -> TokenStream {
    let item = s.iter();

    quote!([#(#item),*])
}
