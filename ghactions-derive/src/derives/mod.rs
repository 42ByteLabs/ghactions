use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Fields};

use crate::attributes::{ActionsAttribute, ActionsAttributeKeys, ActionsAttributeValue};
use ghactions_core::{actions::models::ActionOutput, ActionInput, ActionYML};

pub(crate) fn derive_parser(ast: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let (_, attributes) = ActionsAttribute::parse_all(&ast.attrs)?;

    let mut action = load_actionyaml(&attributes)?;

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            for field in fields.named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                let (name, field_attributes) = ActionsAttribute::parse_all(&field.attrs).unwrap();

                match name.as_str() {
                    "input" => {
                        let mut input = ActionInput::default();

                        input.r#type = field_type.to_token_stream().to_string();

                        field_attributes.iter().for_each(|attr| match attr {
                            ActionsAttribute {
                                key: Some(ActionsAttributeKeys::Description),
                                value: Some(ActionsAttributeValue::String(description)),
                                ..
                            } => {
                                input.description = Some(description.clone());
                            }
                            ActionsAttribute {
                                key: Some(ActionsAttributeKeys::Required),
                                value: Some(ActionsAttributeValue::Bool(required)),
                                ..
                            } => {
                                input.required = Some(*required);
                            }
                            ActionsAttribute {
                                key: Some(ActionsAttributeKeys::Default),
                                value: Some(ActionsAttributeValue::String(default)),
                                ..
                            } => {
                                input.default = Some(default.clone());
                            }
                            _ => {}
                        });

                        action.inputs.insert(field_name.to_string(), input);
                    }
                    "output" => {
                        let mut output = ActionOutput::default();

                        match field_attributes
                            .iter()
                            .find(|attr| attr.key == Some(ActionsAttributeKeys::Description))
                        {
                            Some(ActionsAttribute {
                                value: Some(ActionsAttributeValue::String(description)),
                                ..
                            }) => {
                                output.description = Some(description.clone());
                            }
                            _ => {}
                        }

                        action.outputs.insert(field_name.to_string(), output);
                    }
                    _ => {}
                }
            }

            let tokens = generate_traits(name, &fields, &ast.generics, &action)?;

            // Generate the action.yml file if the feature is enabled
            #[cfg(feature = "generate")]
            {
                if action.path.is_some() {
                    action
                        .write()
                        .map_err(|e| syn::Error::new(ast.span(), e.to_string()))?;
                }
            }

            Ok(tokens)
        }
        _ => Ok(
            syn::Error::new(ast.span(), "Only structs with named fields are supported")
                .to_compile_error(),
        ),
    }
}

pub(crate) fn generate_traits(
    ident: &syn::Ident,
    _fields: &syn::FieldsNamed,
    generics: &syn::Generics,
    action: &ActionYML,
) -> Result<TokenStream, syn::Error> {
    let mut stream = TokenStream::new();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut selfstream = TokenStream::new();

    for (name, input) in action.inputs.iter() {
        let input_name = format!("INPUT_{}", name.to_uppercase());
        let ident_input = syn::Ident::new(name, ident.span());
        let required = if input.required.unwrap_or(false) {
            eprintln!("input: {:?}", input);
            quote! { ? }
        } else {
            quote! { .unwrap_or_default() }
        };

        match input.r#type.as_str() {
            "String" | "&str" => {
                selfstream.extend(quote! {
                    #ident_input: Self::get_input(#input_name)
                        #required,
                });
            }
            "bool" => {
                selfstream.extend(quote! {
                    #ident_input: Self::get_input_bool(#input_name)
                        #required,
                });
            }
            "i32" | "i64" | "u32" | "u64" => {
                selfstream.extend(quote! {
                    #ident_input: Self::get_input_int(#input_name)
                        #required,
                });
            }
            _ => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("Unsupported type for input {} ({})", name, input.r#type),
                ));
            }
        }
    }
    for (name, _output) in action.outputs.iter() {
        let ident_output = syn::Ident::new(name, ident.span());
        selfstream.extend(quote! {
            #ident_output: String::new(),
        });
    }

    let action_name = action.name.clone().unwrap_or_default();
    let action_description = action.description.clone().unwrap_or_default();

    let dotenv = match cfg!(feature = "dotenvy") {
        true => quote! {
            ::dotenvy::dotenv().ok();
        },
        false => quote! {},
    };
    let log = match cfg!(feature = "log") {
        true => quote! {
            ::ghactions::init_logger().init();
        },
        false => quote! {},
    };

    stream.extend(quote! {
        #[automatically_derived]
        impl #impl_generics ::ghactions::ActionTrait for #ident #ty_generics #where_clause {
            fn init() -> Result<Self, ::ghactions::ActionsError> {
                #dotenv
                #log

                Ok(Self {
                    #selfstream
                })
            }

            fn name(&self) -> &str {
                #action_name
            }

            fn description(&self) -> &str {
                #action_description
            }
        }
    });

    Ok(stream)
}

fn load_actionyaml(attributes: &Vec<ActionsAttribute>) -> Result<ActionYML, syn::Error> {
    let mut action = ActionYML::default();

    for attr in attributes.iter() {
        match attr.key {
            Some(ActionsAttributeKeys::Path) => {
                if let Some(ActionsAttributeValue::Path(ref value)) = attr.value {
                    action.path = Some(value.clone());
                }
            }
            Some(ActionsAttributeKeys::Name) => {
                if let Some(ActionsAttributeValue::String(ref value)) = attr.value {
                    action.name = Some(value.clone());
                }
            }
            Some(ActionsAttributeKeys::Description) => {
                if let Some(ActionsAttributeValue::String(ref value)) = attr.value {
                    action.description = Some(value.clone());
                }
            }
            _ => {}
        }
    }
    Ok(action)
}
