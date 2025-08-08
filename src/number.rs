use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;

use crate::Input;

pub fn number_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(i64);

        impl #ident {
            /// Create a new `#ident`
            ///
            /// This method creates a new `#ident` from an `i64`.
            ///
            /// # Example
            ///
            /// ```
            /// use typed_fields::number;
            ///
            /// number!(Number);
            ///
            /// let number = Number::new(42);
            /// ```
            pub fn new(id: i64) -> Self {
                Self(id)
            }

            /// Get the inner value of the `#ident`
            ///
            /// This method returns a copy of the inner value of the `#ident`.
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

fn derives() -> proc_macro2::TokenStream {
    let mut derives = quote! {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
