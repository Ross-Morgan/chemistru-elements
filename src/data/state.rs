use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, TokenStreamExt, ToTokens};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct StateData {
    pub boiling_point: Option<f64>,
    pub melting_point: Option<f64>,
}


impl ToTokens for StateData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let melting_point = self.melting_point.unwrap_or(f64::NAN);
        let boiling_point = self.boiling_point.unwrap_or(f64::NAN);

        let add_tokens = quote! {
            chemistru_elements::raw::StateData {
                melting_point: #melting_point,
                boiling_point: #boiling_point,
            }
        };

        let group = TokenTree::Group(Group::new(
            Delimiter::Brace,
            add_tokens,
        ));

        tokens.append(group);
    }
}
