use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

pub fn name_impl(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);

    let newtype = quote! {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct #ident(String);

        impl #ident {
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into())
            }

            pub fn get(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for #ident {
            fn from(string: &str) -> #ident {
                #ident::new(string)
            }
        }

        impl From<String> for #ident {
            fn from(string: String) -> #ident {
                #ident::new(string)
            }
        }
    };

    newtype.into()
}
