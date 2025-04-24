use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Lit, Meta, NestedMeta, MetaNameValue};

/// Derive macro for implementing the Validate trait
/// 
/// # Example
/// 
/// ```rust
/// #[derive(Validate)]
/// struct User {
///     #[validate(required, length(min = 3, max = 20))]
///     username: String,
///     
///     #[validate(required, email)]
///     email: String,
///     
///     #[validate(min = 18)]
///     age: i32,
/// }
/// ```
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the name of the struct
    let name = &input.ident;
    
    // Get the fields of the struct
    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("Validate derive only supports structs with named fields"),
            }
        },
        _ => panic!("Validate derive only supports structs"),
    };
    
    // Generate validation code for each field
    let field_validations = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        
        // Get validation attributes
        let validations = field.attrs.iter()
            .filter(|attr| attr.path.is_ident("validate"))
            .flat_map(|attr| {
                match attr.parse_meta() {
                    Ok(Meta::List(meta_list)) => meta_list.nested,
                    _ => panic!("Invalid validate attribute"),
                }
            });
        
        // Generate validation code for each attribute
        let validation_code = validations.map(|validation| {
            match validation {
                NestedMeta::Meta(Meta::Path(path)) => {
                    let rule_name = path.get_ident().unwrap().to_string();
                    let rule_ident = format_ident!("{}", rule_name);
                    
                    match rule_name.as_str() {
                        "required" => quote! {
                            if let Err(err) = validator.get_rule("required").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "email" => quote! {
                            if let Err(err) = validator.get_rule("email").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "url" => quote! {
                            if let Err(err) = validator.get_rule("url").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "uuid" => quote! {
                            if let Err(err) = validator.get_rule("uuid").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "json" => quote! {
                            if let Err(err) = validator.get_rule("json").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "positive" => quote! {
                            if let Err(err) = validator.get_rule("positive").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "negative" => quote! {
                            if let Err(err) = validator.get_rule("negative").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "unique" => quote! {
                            if let Err(err) = validator.get_rule("unique").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        "phone" => quote! {
                            if let Err(err) = validator.get_rule("phone").unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                        _ => quote! {
                            // Custom rule
                            if let Err(err) = validator.get_rule(#rule_name).unwrap().validate(&self.#field_name as &dyn Any) {
                                errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                            }
                        },
                    }
                },
                NestedMeta::Meta(Meta::List(meta_list)) => {
                    let rule_name = meta_list.path.get_ident().unwrap().to_string();
                    let rule_ident = format_ident!("{}", rule_name);
                    
                    match rule_name.as_str() {
                        "length" => {
                            let mut min = 0;
                            let mut max = None;
                            
                            for nested in meta_list.nested.iter() {
                                if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                                    let name = name_value.path.get_ident().unwrap().to_string();
                                    if let Lit::Int(lit_int) = &name_value.lit {
                                        let value = lit_int.base10_parse::<usize>().unwrap();
                                        if name == "min" {
                                            min = value;
                                        } else if name == "max" {
                                            max = Some(value);
                                        }
                                    }
                                }
                            }
                            
                            let rule_name = format!("{}_length", field_name_str);
                            
                            quote! {
                                validator.add_rule(#rule_name, common::Length { min: #min, max: #max });
                                if let Err(err) = validator.get_rule(#rule_name).unwrap().validate(&self.#field_name as &dyn Any) {
                                    errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                                }
                            }
                        },
                        "min" => {
                            let mut value = 0;
                            
                            for nested in meta_list.nested.iter() {
                                if let NestedMeta::Lit(Lit::Int(lit_int)) = nested {
                                    value = lit_int.base10_parse::<i32>().unwrap();
                                }
                            }
                            
                            let rule_name = format!("{}_min", field_name_str);
                            
                            quote! {
                                validator.add_rule(#rule_name, numeric::Min { value: #value });
                                if let Err(err) = validator.get_rule(#rule_name).unwrap().validate(&self.#field_name as &dyn Any) {
                                    errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                                }
                            }
                        },
                        "max" => {
                            let mut value = 0;
                            
                            for nested in meta_list.nested.iter() {
                                if let NestedMeta::Lit(Lit::Int(lit_int)) = nested {
                                    value = lit_int.base10_parse::<i32>().unwrap();
                                }
                            }
                            
                            let rule_name = format!("{}_max", field_name_str);
                            
                            quote! {
                                validator.add_rule(#rule_name, numeric::Max { value: #value });
                                if let Err(err) = validator.get_rule(#rule_name).unwrap().validate(&self.#field_name as &dyn Any) {
                                    errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                                }
                            }
                        },
                        "range" => {
                            let mut min = 0;
                            let mut max = 0;
                            
                            for nested in meta_list.nested.iter() {
                                if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                                    let name = name_value.path.get_ident().unwrap().to_string();
                                    if let Lit::Int(lit_int) = &name_value.lit {
                                        let value = lit_int.base10_parse::<i32>().unwrap();
                                        if name == "min" {
                                            min = value;
                                        } else if name == "max" {
                                            max = value;
                                        }
                                    }
                                }
                            }
                            
                            let rule_name = format!("{}_range", field_name_str);
                            
                            quote! {
                                validator.add_rule(#rule_name, numeric::Range { min: #min, max: #max });
                                if let Err(err) = validator.get_rule(#rule_name).unwrap().validate(&self.#field_name as &dyn Any) {
                                    errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                                }
                            }
                        },
                        _ => quote! {},
                    }
                },
                NestedMeta::Meta(Meta::NameValue(name_value)) => {
                    let rule_name = name_value.path.get_ident().unwrap().to_string();
                    
                    match rule_name.as_str() {
                        "min" => {
                            if let Lit::Int(lit_int) = &name_value.lit {
                                let value = lit_int.base10_parse::<i32>().unwrap();
                                let rule_name = format!("{}_min", field_name_str);
                                
                                quote! {
                                    validator.add_rule(#rule_name, numeric::Min { value: #value });
                                    if let Err(err) = validator.get_rule(#rule_name).unwrap().validate(&self.#field_name as &dyn Any) {
                                        errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                                    }
                                }
                            } else {
                                quote! {}
                            }
                        },
                        "max" => {
                            if let Lit::Int(lit_int) = &name_value.lit {
                                let value = lit_int.base10_parse::<i32>().unwrap();
                                let rule_name = format!("{}_max", field_name_str);
                                
                                quote! {
                                    validator.add_rule(#rule_name, numeric::Max { value: #value });
                                    if let Err(err) = validator.get_rule(#rule_name).unwrap().validate(&self.#field_name as &dyn Any) {
                                        errors.entry(#field_name_str.to_string()).or_insert_with(Vec::new).push(format!("{}", err));
                                    }
                                }
                            } else {
                                quote! {}
                            }
                        },
                        _ => quote! {},
                    }
                },
                _ => quote! {},
            }
        }).collect::<Vec<_>>();
        
        quote! {
            #(#validation_code)*
        }
    }).collect::<Vec<_>>();
    
    // Generate the implementation of the Validate trait
    let expanded = quote! {
        impl Validate for #name {
            fn validate(&self) -> Result<(), ValidationError> {
                use std::any::Any;
                use std::collections::HashMap;
                
                // Create a new validator instance
                let mut validator = Validator::new();
                
                // Add common validation rules
                validator.add_rule("required", common::Required);
                validator.add_rule("email", common::Email { check_dns: false });
                validator.add_rule("url", common::Url { allowed_schemes: None });
                validator.add_rule("uuid", common::Uuid);
                validator.add_rule("json", common::Json);
                validator.add_rule("positive", numeric::Positive);
                validator.add_rule("negative", numeric::Negative);
                validator.add_rule("unique", collection::Unique);
                validator.add_rule("phone", common::Phone { allow_empty: false });
                
                // Validate fields
                let mut errors = HashMap::new();
                
                #(#field_validations)*
                
                // Check if there are any validation errors
                if !errors.is_empty() {
                    return Err(ValidationError::Multiple(errors));
                }
                
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded)
}
