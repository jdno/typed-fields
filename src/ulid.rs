use proc_macro::TokenStream;

use quote::quote;
use syn::{Ident, parse_macro_input};

use crate::Input;

pub fn ulid_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();
    let sea_orm_trait_impls = sea_orm_trait_impls(&ident);

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(ulid::Ulid);

         impl #ident {
            /// Create a new `#ident`
            ///
            /// This method creates a new `#ident` from a `Ulid`.
            ///
            /// # Example
            ///
            /// ```
            /// use ulid::Ulid;
            /// use typed_fields::ulid;
            ///
            /// ulid!(MyUlid);
            ///
            /// let ulid = MyUlid::new(Ulid::new());
            /// ```
            pub fn new(ulid: ulid::Ulid) -> Self {
                Self(ulid)
            }

            /// Get the inner value of the `#ident`
            ///
            /// This method returns a reference to the inner value of the
            /// `#ident`.
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
                let uuid = sea_orm::prelude::Uuid::from(source.0);
                uuid.into()
            }
        }

        #[cfg(feature = "sea-orm")]
        impl sea_orm::TryGetable for #ident {
            fn try_get_by<I: sea_orm::ColIdx>(result: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
                let uuid = <sea_orm::prelude::Uuid as sea_orm::TryGetable>::try_get_by(result, index)?;
                let ulid = ulid::Ulid::from(uuid);
                Ok(#ident(ulid))
            }
        }

        #[cfg(feature = "sea-orm")]
        impl sea_orm::sea_query::ValueType for #ident {
            fn try_from(value: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                let uuid = <sea_orm::prelude::Uuid as sea_orm::sea_query::ValueType>::try_from(value)?;
                let ulid = ulid::Ulid::from(uuid);
                Ok(#ident(ulid))
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
                <sea_orm::prelude::Uuid as sea_orm::sea_query::Nullable>::null()
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
