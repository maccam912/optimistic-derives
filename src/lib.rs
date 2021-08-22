extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn optimistic_std(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream)
                 -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        #input
    };
    output.into()
}

#[proc_macro_attribute]
pub fn optimistic_serde(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream)
                 -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
        #input
    };
    output.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
