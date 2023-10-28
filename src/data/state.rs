use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt, ToTokens};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct StateData {
    /// Boiling point in kelvin
    pub boiling_point: f64,
    /// Melting point in kelvin
    pub melting_point: f64,
}

impl ToTokens for StateData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let melting_point = self.melting_point;
        let boiling_point = self.boiling_point;

        let add_tokens = quote! {
            chemistru_elements::data::state::StateData {
                melting_point: #melting_point,
                boiling_point: #boiling_point,
            }
        };

        let group = TokenTree::Group(Group::new(
            Delimiter::None,
            add_tokens,
        ));

        tokens.append(group);
    }
}
