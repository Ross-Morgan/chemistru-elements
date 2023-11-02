use proc_macro2::{Delimiter, Group, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct AtomicData {
    /// Number of protons
    pub atomic_number: u8,
    /// Number of nucleons (protons and neutrons)
    pub nucleon_number: u16,
    /// Atomic mass
    pub atomic_mass: f64,
}

impl ToTokens for AtomicData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let atomic_number = self.atomic_number;
        let nucleon_number = self.nucleon_number;
        let atomic_mass = self.atomic_mass;

        let add_tokens = quote! {
            chemistru_elements::data::atomic::AtomicData {
                atomic_number: #atomic_number,
                nucleon_number: #nucleon_number,
                atomic_mass: #atomic_mass,
            }
        };

        let group = proc_macro2::TokenTree::Group(Group::new(Delimiter::None, add_tokens));

        tokens.append(group);
    }
}
