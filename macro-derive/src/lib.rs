use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(HasBar)]
pub fn macro_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    impl_macro_derive(ast)
}

fn impl_macro_derive(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;

    quote::quote! {
        impl WithBar for #ident {
            fn bar() -> &'static str {
                "bar"
            }
        }
    }
    .into()
}
