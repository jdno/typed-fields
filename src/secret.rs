use proc_macro::TokenStream;

use quote::quote;
use syn::{Ident, parse_macro_input};

use crate::Input;

pub fn secret_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();
    let sea_orm_trait_impls = sea_orm_trait_impls(&ident);

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(secrecy::SecretString);

        impl #ident {
            /// Create a new `#ident`
            ///
            /// This method creates a new `#ident` from a `&str`.
            ///
            /// # Example
            ///
            /// ```
            /// use typed_fields::secret;
            ///
            /// secret!(Secret);
            ///
            /// let secret = Secret::new("secret");
            /// ```
            pub fn new(secret: &str) -> Self {
                Self(String::from(secret).into())
            }

            /// Expose the secret's inner value
            ///
            /// This method returns a reference to the exposed value of the
            /// `#ident`.
            pub fn expose(&self) -> &str {
                use secrecy::ExposeSecret;
                self.0.expose_secret()
            }
        }

        impl PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                self.expose() == other.expose()
            }
        }
        impl Eq for #ident {}

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[REDACTED]")
            }
        }

        impl From<&str> for #ident {
            fn from(secret: &str) -> #ident {
                #ident(String::from(secret).into())
            }
        }

        impl From<String> for #ident {
            fn from(secret: String) -> #ident {
                #ident(secret.into())
            }
        }

        #sea_orm_trait_impls
    };

    newtype.into()
}

fn derives() -> proc_macro2::TokenStream {
    let mut derives = quote! {
        #[derive(Clone, Debug)]
    };

    derives.extend(derive_serde());

    derives
}

#[cfg(feature = "sea-orm")]
fn sea_orm_trait_impls(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #[cfg(feature = "sea-orm")]
        impl From<#ident> for sea_orm::Value {
            fn from(source: #ident) -> Self {
                let string = source.expose().to_string();
                string.into()
            }
        }

        #[cfg(feature = "sea-orm")]
        impl sea_orm::TryGetable for #ident {
            fn try_get_by<I: sea_orm::ColIdx>(result: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
                let string = <String as sea_orm::TryGetable>::try_get_by(result, index)?;
                Ok(#ident(string.into()))
            }
        }

        #[cfg(feature = "sea-orm")]
        impl sea_orm::sea_query::ValueType for #ident {
            fn try_from(value: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                let string = <String as sea_orm::sea_query::ValueType>::try_from(value)?;
                Ok(#ident(string.into()))
            }

            fn type_name() -> String {
                stringify!(#ident).to_owned()
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                sea_orm::sea_query::ArrayType::String
            }

            fn column_type() -> sea_orm::sea_query::ColumnType {
                sea_orm::sea_query::ColumnType::String(sea_orm::sea_query::StringLen::None)
            }
        }

        #[cfg(feature = "sea-orm")]
        impl sea_orm::sea_query::Nullable for #ident {
            fn null() -> sea_orm::Value {
                <String as sea_orm::sea_query::Nullable>::null()
            }
        }
    }
}

#[cfg(not(feature = "sea-orm"))]
fn sea_orm_trait_impls(_ident: &Ident) -> proc_macro2::TokenStream {
    quote! {}
}

#[cfg(feature = "serde")]
fn derive_serde() -> proc_macro2::TokenStream {
    quote! {
        #[derive(serde::Deserialize)]
    }
}

#[cfg(not(feature = "serde"))]
fn derive_serde() -> proc_macro2::TokenStream {
    quote! {}
}
