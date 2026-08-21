use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::Input;

pub fn path_impl(input: TokenStream) -> TokenStream {
    let Input { attrs, ident } = parse_macro_input!(input as Input);
    let derives = derives();

    let new_doc = format!(
        "Creates a new `{ident}`\n\
         \n\
         This method creates a new `{ident}` from a `PathBuf`."
    );
    let get_doc = format!(
        "Gets the inner value of the `{ident}`\n\
         \n\
         This method returns a reference to the inner value of the `{ident}`."
    );

    let newtype = quote! {
        #(#attrs)*
        #derives
        pub struct #ident(std::path::PathBuf);

        impl #ident {
            #[doc = #new_doc]
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

            #[doc = #get_doc]
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
    };

    newtype.into()
}

fn derives() -> proc_macro2::TokenStream {
    quote! {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    }
}
