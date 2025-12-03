use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_macro_input;

use crate::Input;

pub fn path_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();
    let sea_orm_trait_impls = sea_orm_trait_impls(&ident);

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(std::path::PathBuf);

        impl #ident {
            /// Create a new `#ident`
            ///
            /// This method creates a new `#ident` from a `PathBuf`.
            ///
            /// # Example
            ///
            /// ```
            /// use std::path::PathBuf;
            /// use typed_fields::path;
            ///
            /// path!(MyPath);
            ///
            /// let path = MyPath::new(PathBuf::from("src"));
            /// ```
            pub fn new(path: std::path::PathBuf) -> Self {
                Self(path.into())
            }

            /// Get the inner value of the `#ident`
            ///
            /// This method returns a reference to the inner value of the
            /// `#ident`.
            pub fn get(&self) -> &std::path::Path {
                &self.0
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.display())
            }
        }

        impl From<&str> for #ident {
            fn from(string: &str) -> #ident {
                #ident::new(std::path::PathBuf::from(string))
            }
        }

        impl From<String> for #ident {
            fn from(string: String) -> #ident {
                #ident::new(std::path::PathBuf::from(string))
            }
        }

        impl From<&std::path::Path> for #ident {
            fn from(path: &std::path::Path) -> #ident {
                #ident::new(path.to_path_buf())
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
        impl From<#ident> for sea_orm::Value {
            fn from(source: #ident) -> Self {
                let string = source.get().display().to_string();
                string.into()
            }
        }

        impl sea_orm::TryGetable for #ident {
            fn try_get_by<I: sea_orm::ColIdx>(result: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
                let string = <String as sea_orm::TryGetable>::try_get_by(result, index)?;
                let path = std::path::PathBuf::from(string);

                Ok(#ident(path))
            }
        }

        impl sea_orm::sea_query::ValueType for #ident {
            fn try_from(value: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                let string = <String as sea_orm::sea_query::ValueType>::try_from(value)?;
                let path = std::path::PathBuf::from(string);

                Ok(#ident(path))
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
