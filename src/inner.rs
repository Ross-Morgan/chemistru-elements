use super::data::prelude::*;

/// Detailed information about an element
/// 
/// Should be held behind avstatic reference where possible
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InnerElement {
    pub name: &'static str,
    pub symbol: &'static str,
    pub description: &'static str,
    pub atomic_data: AtomicData,
    pub state_data: StateData,
    pub electron_data: ElectronData,
}


impl ToTokens for InnerElement {
    fn to_tokens(&self, stream: &mut TokenStream) {
        let name = self.name;
        let symbol = self.symbol;
        let desc = self.description;
        let atomic_data = self.atomic_data;
        let state_data = self.state_data;
        let electron_data = self.electron_data;

        let tokens = quote! {
            InnerElement {
                name: #name,
                symbol: #symbol,
                description: #desc,
                atomic_data: #atomic_data,
                state_data: #state_data,
                electron_data: #electron_data,
            }
        };
    }
}
