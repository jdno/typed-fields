use proc_macro::TokenStream;

use quote::quote;
use syn::{Ident, parse_macro_input};

use crate::Input;

pub fn uuid_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();
    let sea_orm_trait_impls = sea_orm_trait_impls(&ident);

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

        #sea_orm_trait_impls
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

#[cfg(feature = "sea-orm")]
fn sea_orm_trait_impls(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #[cfg(feature = "sea-orm")]
        impl From<#ident> for sea_orm::Value {
            fn from(source: #ident) -> Self {
                source.0.into()
            }
        }

        #[cfg(feature = "sea-orm")]
        impl sea_orm::TryGetable for #ident {
            fn try_get_by<I: sea_orm::ColIdx>(result: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
                let uuid = <uuid::Uuid as sea_orm::TryGetable>::try_get_by(result, index)?;
                Ok(#ident(uuid))
            }
        }

        #[cfg(feature = "sea-orm")]
        impl sea_orm::sea_query::ValueType for #ident {
            fn try_from(value: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                let uuid = <uuid::Uuid as sea_orm::sea_query::ValueType>::try_from(value)?;
                Ok(#ident(uuid))
            }

            fn type_name() -> String {
                stringify!(#ident).to_owned()
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                sea_orm::sea_query::ArrayType::Uuid
            }

            fn column_type() -> sea_orm::sea_query::ColumnType {
                sea_orm::sea_query::ColumnType::Uuid
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
        #[derive(serde::Deserialize, serde::Serialize)]
    }
}

#[cfg(not(feature = "serde"))]
fn derive_serde() -> proc_macro2::TokenStream {
    quote! {}
}
