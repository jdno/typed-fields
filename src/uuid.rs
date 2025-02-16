use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;

use crate::Input;

pub fn uuid_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(uuid::Uuid);

         impl #ident {
            /// Create a new `#ident`
            ///
            /// This method creates a new `#ident` from a `Uuid`.
            ///
            /// # Example
            ///
            /// ```
            /// use typed_fields::uuid;
            ///
            /// uuid!(MyUuid);
            ///
            /// let uuid = MyUuid::new(Uuid::new_v4());
            /// ```
            pub fn new(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }

            /// Get the inner value of the `#ident`
            ///
            /// This method returns a reference to the inner value of the
            /// `#ident`.
            pub fn get(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<uuid::Uuid> for #ident {
            fn from(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl TryFrom<&str> for #ident {
            type Error = uuid::Error;

            fn try_from(string: &str) -> Result<Self, Self::Error> {
                Ok(Self(uuid::Uuid::try_from(string)?))
            }
        }

        impl TryFrom<String> for #ident {
            type Error = uuid::Error;

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
