use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;

use crate::Input;

pub fn name_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(String);

        impl #ident {
            /// Create a new `#ident`
            ///
            /// This method creates a new `#ident` from anything that implements
            /// the `Into<String>` trait. This includes `&str`, `String`, and
            /// other types that can be converted into a `String`.
            ///
            /// # Example
            ///
            /// ```
            /// use typed_fields::name;
            ///
            /// name!(Name);
            ///
            /// let name = Name::new("name");
            /// ```
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into())
            }

            /// Get the inner value of the `#ident`
            ///
            /// This method returns a reference to the inner value of the
            /// `#ident`.
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

fn derives() -> proc_macro2::TokenStream {
    let mut derives = quote! {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    };

    derives.extend(derive_sea_orm());
    derives.extend(derive_serde());

    derives
}

#[cfg(feature = "sea-orm")]
fn derive_sea_orm() -> proc_macro2::TokenStream {
    quote! {
        #[derive(sea_orm::DeriveValueType)]
    }
}

#[cfg(not(feature = "sea-orm"))]
fn derive_sea_orm() -> proc_macro2::TokenStream {
    quote! {}
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
