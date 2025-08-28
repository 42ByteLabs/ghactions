use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Attribute, Ident, LitBool, LitInt, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

const ALLOWED_COLOURS: [&str; 9] = [
    "white",
    "black",
    "yellow",
    "blue",
    "green",
    "orange",
    "red",
    "purple",
    "gray-dark",
];

#[derive(Debug, Clone)]
pub(crate) struct ActionsAttribute {
    #[allow(dead_code)]
    pub(crate) span: Ident,
    pub(crate) key: Option<ActionsAttributeKeys>,
    pub(crate) value: Option<ActionsAttributeValue>,
    pub(crate) value_span: Option<Span>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ActionsAttributeKeys {
    /// Main Actions attribute
    Actions,
    /// Action Input attribute
    Input,
    /// Action Output attribute
    Output,
    /// Output Step name
    OutputStepName,

    // Sub-attributes
    /// Path attribute
    Path,
    /// Name / Rename
    Name,
    Description,
    /// Author / Maintainer name
    Author,
    /// Branding Icon
    BrandingIcon,
    /// Branding Color
    BrandingColor,
    /// Default
    Default,
    /// https://docs.github.com/en/actions/learn-github-actions/expressions
    Expression,
    /// https://docs.github.com/en/actions/learn-github-actions/contexts#github-context
    GitHub,
    /// Required attribute
    Required,
    /// Docker Image
    Image,
    /// Separator
    Separator,
    /// Entrypoint
    Entrypoint,
    /// Composite Action
    Composite,
    /// Installer
    Installer,
}

#[derive(Debug, Clone)]
pub(crate) enum ActionsAttributeValue {
    /// String value
    #[allow(dead_code)]
    String(String),
    /// Integer value
    #[allow(dead_code)]
    Int(i64),
    /// Boolean value
    #[allow(dead_code)]
    Bool(bool),
    /// Path value
    #[allow(dead_code)]
    Path(std::path::PathBuf),
}

impl Parse for ActionsAttribute {
    #[allow(irrefutable_let_patterns)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        let key: Option<ActionsAttributeKeys> = match name_str.as_str() {
            // Main attributes
            "actions" => Some(ActionsAttributeKeys::Actions),
            "input" => Some(ActionsAttributeKeys::Input),
            "output" => Some(ActionsAttributeKeys::Output),
            "output-step" | "output_step" => Some(ActionsAttributeKeys::OutputStepName),
            // Sub-attributes
            "path" => Some(ActionsAttributeKeys::Path),
            "name" | "rename" => Some(ActionsAttributeKeys::Name),
            "description" => Some(ActionsAttributeKeys::Description),
            "author" => Some(ActionsAttributeKeys::Author),
            "branding_icon" | "icon" => Some(ActionsAttributeKeys::BrandingIcon),
            "branding_color" | "color" => Some(ActionsAttributeKeys::BrandingColor),
            "default" => Some(ActionsAttributeKeys::Default),
            "expression" => Some(ActionsAttributeKeys::Expression),
            "required" => Some(ActionsAttributeKeys::Required),
            "image" => Some(ActionsAttributeKeys::Image),
            "entrypoint" => Some(ActionsAttributeKeys::Entrypoint),
            "composite" => Some(ActionsAttributeKeys::Composite),
            "installer" => Some(ActionsAttributeKeys::Installer),
            "separator" | "split" => Some(ActionsAttributeKeys::Separator),
            _ => {
                return Err(syn::Error::new(
                    name.span(),
                    format!("Unknown attribute: {}", name),
                ));
            }
        };

        let mut value_span: Option<Span> = None;

        let value = if input.peek(Token![=]) {
            // `name = value` attributes.
            let _assign_token = input.parse::<Token![=]>()?; // skip '='

            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;

                // TODO: Is this correct?
                if lit.value().starts_with("..")
                    || lit.value().starts_with("./")
                    || lit.value().starts_with("/")
                {
                    value_span = Some(lit.span());
                    Some(ActionsAttributeValue::Path(lit.value().into()))
                } else {
                    value_span = Some(lit.span());
                    Some(ActionsAttributeValue::String(lit.value()))
                }
            } else if input.peek(LitInt) {
                let lit: LitInt = input.parse()?;
                value_span = Some(lit.span());

                Some(ActionsAttributeValue::Int(lit.base10_parse().unwrap()))
            } else if input.peek(LitBool) {
                let lit: LitBool = input.parse()?;

                Some(ActionsAttributeValue::Bool(lit.value))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self {
            span: name,
            key,
            value,
            value_span,
        })
    }
}

impl ActionsAttribute {
    pub(crate) fn parse_all(all_attrs: &[Attribute]) -> Result<(String, Vec<Self>), syn::Error> {
        let mut name = String::new();
        let mut parsed = Vec::new();

        for attribute in all_attrs {
            // TODO: This could be nicer
            if attribute.path().is_ident("action") {
                name = String::from("action");
                for attr in attribute
                    .parse_args_with(Punctuated::<ActionsAttribute, Token![,]>::parse_terminated)?
                {
                    // Validate the attribute before adding it to the parsed list
                    attr.validate()?;
                    parsed.push(attr);
                }
            } else if attribute.path().is_ident("input") {
                name = String::from("input");
                for attr in attribute
                    .parse_args_with(Punctuated::<ActionsAttribute, Token![,]>::parse_terminated)?
                {
                    // Validate the attribute before adding it to the parsed list
                    attr.validate()?;
                    parsed.push(attr);
                }
            } else if attribute.path().is_ident("output") {
                name = String::from("output");
                for attr in attribute
                    .parse_args_with(Punctuated::<ActionsAttribute, Token![,]>::parse_terminated)?
                {
                    // Validate the attribute before adding it to the parsed list
                    attr.validate()?;
                    parsed.push(attr);
                }
            } else {
                continue;
            };
        }
        Ok((name, parsed))
    }

    #[allow(irrefutable_let_patterns)]
    pub(crate) fn validate(&self) -> Result<(), syn::Error> {
        match self.key {
            Some(ActionsAttributeKeys::Path) => {
                if let Some(ActionsAttributeValue::Path(_)) = &self.value {
                    // TODO: Validate path
                    Ok(())
                } else if let Some(ActionsAttributeValue::String(_)) = &self.value {
                    return Err(syn::Error::new(
                        self.value_span.unwrap(),
                        "Path attribute must start with `.` or `/` (e.g. `./action.yml`)",
                    ));
                } else {
                    Err(syn::Error::new(
                        self.span.span(),
                        "Path attribute must have a string value",
                    ))
                }
            }
            Some(ActionsAttributeKeys::Name) => {
                if let Some(ActionsAttributeValue::String(_)) = &self.value {
                    Ok(())
                } else {
                    Err(syn::Error::new(
                        self.span.span(),
                        "Name attribute must have a string value",
                    ))
                }
            }
            Some(ActionsAttributeKeys::BrandingColor) => {
                if let Some(ActionsAttributeValue::String(data)) = &self.value {
                    if ALLOWED_COLOURS.contains(&data.as_str()) {
                        Ok(())
                    } else {
                        Err(syn::Error::new(
                            self.value_span.unwrap(),
                            "Invalid color value, please check the documentation for allowed values",
                        ))
                    }
                } else {
                    Err(syn::Error::new(
                        self.span.span(),
                        "Color attribute must have a string value",
                    ))
                }
            }
            Some(ActionsAttributeKeys::Required) => {
                if let Some(ActionsAttributeValue::Bool(_)) = &self.value {
                    Ok(())
                } else {
                    Err(syn::Error::new(
                        self.value_span.unwrap(),
                        "Required attribute must have a boolean value",
                    ))
                }
            }
            Some(ActionsAttributeKeys::Image) => {
                if let Some(ActionsAttributeValue::Path(path)) = &self.value {
                    if path.exists() {
                        Ok(())
                    } else {
                        Err(syn::Error::new(
                            self.value_span.unwrap(),
                            "Image attribute must have a valid path value (file not found)",
                        ))
                    }
                } else {
                    Err(syn::Error::new(
                        self.span.span(),
                        "Image attribute must have a string value",
                    ))
                }
            }
            Some(ActionsAttributeKeys::Entrypoint) => {
                if let Some(ActionsAttributeValue::Path(path)) = &self.value {
                    if path.exists() {
                        Ok(())
                    } else {
                        Err(syn::Error::new(
                            self.value_span.unwrap(),
                            "Entrypoint attribute must have a valid path value (file not found)",
                        ))
                    }
                } else if let Some(ActionsAttributeValue::String(_)) = &self.value {
                    Ok(())
                } else {
                    Err(syn::Error::new(
                        self.span.span(),
                        "Entrypoint attribute must have a string value",
                    ))
                }
            }
            Some(ActionsAttributeKeys::Composite) => {
                if let Some(ActionsAttributeValue::Bool(_)) = &self.value {
                    Ok(())
                } else {
                    Err(syn::Error::new(
                        self.value_span.unwrap(),
                        "Composite attribute must have a boolean value",
                    ))
                }
            }
            Some(ActionsAttributeKeys::Installer) => {
                if let Some(ActionsAttributeValue::Path(path)) = &self.value {
                    if path.exists() {
                        Ok(())
                    } else {
                        Err(syn::Error::new(
                            self.value_span.unwrap(),
                            "Installer attribute must have a valid path value (file not found)",
                        ))
                    }
                } else if let Some(ActionsAttributeValue::Int(_)) = &self.value {
                    Err(syn::Error::new(
                        self.value_span.unwrap(),
                        "Installer attribute must have a string value",
                    ))
                } else {
                    Ok(())
                }
            }
            Some(ActionsAttributeKeys::Separator) => {
                if let Some(ActionsAttributeValue::String(_)) = &self.value {
                    Ok(())
                } else {
                    Err(syn::Error::new(
                        self.span.span(),
                        "Separator attribute must have a string value",
                    ))
                }
            }
            _ => Ok(()),
        }
    }
}
