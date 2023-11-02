use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct StateData {
    /// Boiling point in kelvin
    pub boiling_point: Option<f64>,
    /// Melting point in kelvin
    pub melting_point: Option<f64>,
}

impl ToTokens for StateData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let melting_point = match self.melting_point {
            Some(v) => quote!(Some(#v)),
            None => quote!(None),
        };

        let boiling_point = match self.boiling_point {
            Some(v) => quote!(Some(#v)),
            None => quote!(None),
        };

        let add_tokens = quote! {
            chemistru_elements::data::state::StateData {
                melting_point: #melting_point,
                boiling_point: #boiling_point,
            }
        };

        let group = TokenTree::Group(Group::new(Delimiter::None, add_tokens));

        tokens.append(group);
    }
}
