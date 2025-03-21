use ghactions_core::ActionYML;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub(crate) fn generate_helpers(
    ident: &syn::Ident,
    fields: &syn::FieldsNamed,
    _generics: &syn::Generics,
    action: &ActionYML,
) -> Result<TokenStream, syn::Error> {
    let mut tokens = TokenStream::new();

    // Generate the `set_{}` functions
    let mut set_functions = TokenStream::new();
    let outputs: Vec<String> = action.outputs.keys().cloned().collect();

    for field in fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap();

        if outputs.contains(&field_name.to_string()) {
            let func_name = format!("set_{}", field_name);
            let func = syn::Ident::new(&func_name, Span::call_site());

            set_functions.extend(quote! {
                pub fn #func(&self, value: impl Into<String>) {
                    <#ident as ghactions::ActionTrait>::set_output(stringify!(#field_name), value);
                }
            });
        }
    }

    tokens.extend(quote! {
        impl #ident {
            #set_functions
        }
    });

    Ok(tokens)
}
