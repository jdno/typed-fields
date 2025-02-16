use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;

use crate::Input;

pub fn secret_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();

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
