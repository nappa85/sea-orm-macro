
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{ItemStruct, Lit, Meta, NestedMeta, Token, parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, token::Comma};

use convert_case::{Case, Casing};

struct TableMacroInput {
    table_name: Option<Lit>,
    primary_key: Option<Lit>,
    relations_enum: Option<Lit>
}

impl Parse for TableMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<NestedMeta, Token![,]>::parse_separated_nonempty(input)?;
        let table_name = args.iter().filter_map(|nm| {
                if let NestedMeta::Meta(Meta::NameValue(v)) = nm {
                    if v.path.get_ident() == Some(&Ident::new("table_name", Span::call_site())) {
                        return Some(&v.lit);
                    }
                }
                None
            }).next().cloned();
        let primary_key = args.iter().filter_map(|nm| {
                if let NestedMeta::Meta(Meta::NameValue(v)) = nm {
                    if v.path.get_ident() == Some(&Ident::new("primary_key", Span::call_site())) {
                        return Some(&v.lit);
                    }
                }
                None
            }).next().cloned();
        let relations_enum = args.iter().filter_map(|nm| {
                if let NestedMeta::Meta(Meta::NameValue(v)) = nm {
                    if v.path.get_ident() == Some(&Ident::new("relations_enum", Span::call_site())) {
                        return Some(&v.lit);
                    }
                }
                None
            }).next().cloned();

        Ok(TableMacroInput {
            table_name,
            primary_key,
            relations_enum,
        })
    }
}

fn ident_to_case(ident: &Ident, case: Case) -> Ident {
    Ident::new(&ident.to_string().to_case(case), Span::call_site())
}

#[proc_macro_attribute]
pub fn table(args: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let input = parse_macro_input!(args as TableMacroInput);

    let mod_name = ident_to_case(&item_struct.ident, Case::Snake);
    let table_name = input.table_name.unwrap();//TODO: better error handling
    let primary_key = input.primary_key.unwrap();//TODO: better error handling
    let mut model_struct: Punctuated<_, Comma> = Punctuated::new();
    let mut columns_enum: Punctuated<_, Comma> = Punctuated::new();
    let mut columns_trait: Punctuated<_, Comma> = Punctuated::new();
    for field in item_struct.fields {
        if let Some(ident) = &field.ident {
            let field_type = &field.ty;
            model_struct.push(quote! { pub #ident: #field_type, });
            let field_name = ident_to_case(ident, Case::Pascal);
            columns_enum.push(quote! { #field_name, });
            let mut nullable = false;
            let field_type = if let Some(attr) = field.attrs.iter().find(|a| a.path.get_ident() == Some(&Ident::new("table", Span::call_site())) && !a.tokens.is_empty()) {
                let field_type = &attr.tokens;//TODO: parse tokens, manage attrbutes, nullability
                quote! { #field_type }
            }
            else {
                let temp = format!("{:?}", field.ty);
                let temp = if temp.starts_with("Option<") {
                    nullable = true;
                    &temp[7..(temp.len() - 8)]
                }
                else {
                    temp.as_str()
                };
                match temp {//TODO: expand match
                    "String" | "&str" => quote! { String(None) },
                    "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" => quote! { Integer },
                    "NaiveDateTime" => quote! { DateTime },
                    _ => unreachable!(),//TODO: better error handling
                }
            };
            if nullable {
                columns_trait.push(quote! { Self::#field_name => ColumnType::#field_type.def().null() });
            }
            else {
                columns_trait.push(quote! { Self::#field_name => ColumnType::#field_type.def() });
            }
        }
    }

    return quote! {
mod #mod_name {
    use sea_orm::entity::prelude::*;

    #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
    pub struct Entity;

    impl EntityName for Entity {
        fn table_name(&self) -> &str {
            #table_name
        }
    }

    #[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
    pub struct Model {
        #model_struct
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
    pub enum Column {
        #columns_enum
    }

    #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
    pub enum PrimaryKey {
        #primary_key
    }

    impl PrimaryKeyTrait for PrimaryKey {
        fn auto_increment() -> bool {
            true
        }
    }

    impl ColumnTrait for Column {
        type EntityName = Entity;

        fn def(&self) -> ColumnDef {
            match self {
                #columns_trait
            }
        }
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}
    
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            unreachable!()
        }
    }
    
    impl ActiveModelBehavior for ActiveModel {}    
}
    }.into();
}
