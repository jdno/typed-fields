use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Attribute, Ident, Token, Type};

/// Input for the `number!` macro
///
/// Supports:
/// - `number!(Ident)` → defaults to `i64`
/// - `number!(Ident, BackingType)`
struct NumberInput {
    attrs: Vec<Attribute>,
    ident: Ident,
    ty: Option<Type>,
}

impl Parse for NumberInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let ident: Ident = input.parse()?;

        let ty = if input.peek(Token![,]) {
            let _ = input.parse::<Token![,]>()?;
            Some(input.parse::<Type>()?)
        } else {
            None
        };

        Ok(Self { attrs, ident, ty })
    }
}

pub fn number_impl(input: TokenStream) -> TokenStream {
    let NumberInput { attrs, ident, ty } = parse_macro_input!(input as NumberInput);
    let derives = derives();
    let backing_ty: Type = ty.unwrap_or_else(|| syn::parse_quote! { i64 });

    let backing_ty_name = quote! { #backing_ty }.to_string();
    let new_doc = format!(
        "Creates a new `{ident}`\n\
         \n\
         This method creates a new `{ident}` from a `{backing_ty_name}`."
    );
    let get_doc = format!(
        "Gets the inner value of the `{ident}`\n\
         \n\
         This method returns a copy of the inner value of the `{ident}`."
    );

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(#backing_ty);

        impl #ident {
            #[doc = #new_doc]
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
            pub fn new(id: #backing_ty) -> Self {
                Self(id)
            }

            #[doc = #get_doc]
            pub fn get(&self) -> #backing_ty {
                self.0
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<#backing_ty> for #ident {
            fn from(id: #backing_ty) -> #ident {
                #ident(id)
            }
        }
    };

    newtype.into()
}

fn derives() -> proc_macro2::TokenStream {
    quote! {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    }
}
