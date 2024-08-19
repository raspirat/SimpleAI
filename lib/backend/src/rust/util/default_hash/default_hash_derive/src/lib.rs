use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(DefaultHash)]
pub fn default_hash_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_default_hash(&ast)
}

fn impl_default_hash(ast: &syn::DeriveInput) -> TokenStream
{
    let name = &ast.ident;
    let gen = quote! {
        impl DefaultHash for #name {
             fn default_hash(&self) -> u64 {
                let mut s = DefaultHasher::new();
                self.hash(&mut s);
                s.finish()
            }
        }
    };
    gen.into()
}
