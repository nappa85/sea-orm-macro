
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, parse_macro_input, punctuated::Punctuated, token::Comma};

use convert_case::{Case, Casing};

fn ident_to_case(ident: &Ident, case: Case) -> Ident {
    Ident::new(&ident.to_string().to_case(case), Span::call_site())
}

#[proc_macro_derive(AutoColumn, attributes(auto_column))]
pub fn derive_auto_column(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if input.ident != "Model" {
        panic!("Struct name must be Model");
    }

    // if #[auto_column(table_name = "foo")] specified, create Entity struct
    let table_name = input.attrs.iter().filter_map(|attr| {
        if attr.path.get_ident()? != "auto_column" {
            return None;
        }

        let list: Meta = attr.parse_args().ok()?;
        if let Meta::NameValue(nv) = &list {
            if nv.path.get_ident()? == "table_name" {
                let table_name = &nv.lit;
                return Some(quote! {
#[derive(Copy, Clone, Default, Debug, sea_orm::prelude::DeriveEntity)]
pub struct Entity;

impl sea_orm::prelude::EntityName for Entity {
    fn table_name(&self) -> &str {
        #table_name
    }
}
                });
            }
        }

        None
    }).next().unwrap_or_default();

    // generate Column enum and it's ColumnTrait impl
    let mut columns_enum: Punctuated<_, Comma> = Punctuated::new();
    let mut columns_trait: Punctuated<_, Comma> = Punctuated::new();
    let mut primary_keys: Punctuated<_, Comma> = Punctuated::new();
    if let Data::Struct(item_struct) = input.data {
        if let Fields::Named(fields) = item_struct.fields {
            for field in fields.named {
                if let Some(ident) = &field.ident {
                    let field_type = &field.ty;
                    let field_name = ident_to_case(ident, Case::Pascal);
                    columns_enum.push(quote! { #field_name });

                    let mut nullable = false;
                    let mut sql_type = None;
                    // search for #[auto_column(type = "bar")]
                    field.attrs.iter().for_each(|attr| {
                        if let Some(ident) = attr.path.get_ident() {
                            if ident != "auto_column" {
                                return;
                            }
                        }
                        else {
                            return;
                        }

                        if let Ok(list) = attr.parse_args() {
                            match &list {
                                Meta::NameValue(nv) => {
                                    if let Some(name) = nv.path.get_ident() {
                                        if name == "type" {
                                            let ty = &nv.lit;
                                            sql_type = Some(quote! { #ty });
                                        }
                                    }
                                },
                                Meta::Path(p) => {
                                    if let Some(name) = p.get_ident() {
                                        if name == "primary_key" {
                                            primary_keys.push(quote! { #field_name });
                                        }
                                    }
                                },
                                _ => {},
                            }
                        }
                    });
                    let field_type = sql_type.unwrap_or_else(|| {
                        let temp = quote! { #field_type }
                            .to_string()//Example: "Option < String >"
                            .replace(" ", "");
                        let temp = if temp.starts_with("Option<") {
                            nullable = true;
                            &temp[7..(temp.len() - 1)]
                        }
                        else {
                            temp.as_str()
                        };
                        match temp {//TODO: expand match
                            "char" => quote! { Char(None) },
                            "String" | "&str" => quote! { String(None) },
                            "u8" | "i8" => quote! { TinyInteger },
                            "u16" | "i16" => quote! { SmallInteger },
                            "u32" | "u64" | "i32" | "i64" => quote! { Integer },
                            "u128" | "i128" => quote! { BigInteger },
                            "f32" => quote! { Float },
                            "f64" => quote! { Double },
                            "bool" => quote! { Boolean },
                            "NaiveDate" => quote! { Date },
                            "NaiveTime" => quote! { Time },
                            "NaiveDateTime" => quote! { DateTime },
                            "Uuid" => quote! { Uuid },
                            _ => panic!("unrecognized type {}", temp),//TODO: better error handling
                        }
                    });

                    if nullable {
                        columns_trait.push(quote! { Self::#field_name => sea_orm::prelude::ColumnType::#field_type.def().null() });
                    }
                    else {
                        columns_trait.push(quote! { Self::#field_name => sea_orm::prelude::ColumnType::#field_type.def() });
                    }
                }
            }
        }
    }

    let primary_key = (!primary_keys.is_empty()).then(|| {
        let auto_increment = primary_keys.len() == 1;
        quote! {
#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    #primary_keys
}

impl PrimaryKeyTrait for PrimaryKey {
    fn auto_increment() -> bool {
        #auto_increment
    }
}
        }
    }).unwrap_or_default();

    return quote! {
#[derive(Copy, Clone, Debug, sea_orm::prelude::EnumIter, sea_orm::prelude::DeriveColumn)]
pub enum Column {
    #columns_enum
}

impl sea_orm::prelude::ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> sea_orm::prelude::ColumnDef {
        match self {
            #columns_trait
        }
    }
}

#table_name

#primary_key
    }.into();
}
