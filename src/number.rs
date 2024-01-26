use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

pub fn number_impl(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);

    let newtype = quote! {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct #ident(i64);

        impl #ident {
            pub fn new(id: i64) -> Self {
                Self(id)
            }

            pub fn get(&self) -> i64 {
                self.0
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<i64> for #ident {
            fn from(id: i64) -> #ident {
                #ident(id)
            }
        }
    };

    newtype.into()
}
