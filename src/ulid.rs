use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

pub fn ulid_impl(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let derives = derives();

    let newtype = quote! {
        #derives
        pub struct #ident(ulid::Ulid);

         impl #ident {
            pub fn new(ulid: ulid::Ulid) -> Self {
                Self(ulid)
            }

            pub fn get(&self) -> &ulid::Ulid {
                &self.0
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<ulid::Ulid> for #ident {
            fn from(ulid: ulid::Ulid) -> Self {
                Self(ulid)
            }
        }

        impl TryFrom<&str> for #ident {
            type Error = ulid::DecodeError;

            fn try_from(string: &str) -> Result<Self, Self::Error> {
                Ok(Self(ulid::Ulid::from_string(string)?))
            }
        }

        impl TryFrom<String> for #ident {
            type Error = ulid::DecodeError;

            fn try_from(string: String) -> Result<Self, Self::Error> {
                Self::try_from(string.as_str())
            }
        }
    };

    newtype.into()
}

fn derives() -> proc_macro2::TokenStream {
    let mut derives = quote! {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    };

    derives.extend(derive_serde());

    derives
}

#[cfg(feature = "serde")]
fn derive_serde() -> proc_macro2::TokenStream {
    quote! {
        #[derive(serde::Deserialize, serde::Serialize)]
    }
}

#[cfg(not(feature = "serde"))]
fn derive_serde() -> proc_macro2::TokenStream {
    quote! {}
}
