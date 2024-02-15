use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed, Lit, Meta, MetaNameValue,
};

#[proc_macro_derive(PSQLInsertable, attributes(psql_type))]
pub fn derive_psql_insertable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };
    let unnest_string = make_unnest_string(fields);
    let field_vecs = make_field_vectors(fields);
    let unnest_query = make_unnest_query(fields);

    // Build the trait implementation

    let expanded = quote! {
        #[::async_trait::async_trait]
        impl PSQLInsertable for #name {
            async fn bulk_insert(req: Vec<#name>, table_name: &str, client: &::sqlx::PgPool) -> Result<()> {
                const UNNEST_FMT_STR:&str = #unnest_string;
                // println!("made unnest format: {}", unnest_fmt_str);
                #field_vecs
                #unnest_query
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn parse_type_name(input: &[Attribute]) -> String {
    for attr in input.iter().filter(|x| x.path.is_ident("psql_type")) {
        let meta = attr.parse_meta().unwrap();
        match meta {
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(v), ..
            }) => return v.value(),
            _ => panic!("incorrect psql type formatting! format should be #[psql_type = `type`]"),
        }
    }
    panic!("no types provided for psql");
}

fn make_unnest_string(fields: &FieldsNamed) -> String {
    fields
        .named
        .iter()
        .enumerate()
        .filter_map(|(i, f)| Some(format!("${}::{}[]", i + 1, parse_type_name(&f.attrs))))
        .collect::<Vec<String>>()
        .join(",")
}

fn make_field_vectors(fields: &FieldsNamed) -> TokenStream {
    let row_name = Ident::new("row", Span::call_site());

    let init_vecs: TokenStream = fields
        .named
        .iter()
        .map(|f| {
            if let Some(name) = &f.ident {
                let vecname = format_ident!("{}_vec", name);
                let ty = &f.ty;
                quote! {
                    let mut #vecname: Vec<#ty> = vec![];
                }
            } else {
                panic!("field in named struct has no name")
            }
        })
        .collect();
    let insert_into_vec: TokenStream = fields
        .named
        .iter()
        .map(|f| {
            // row is the name of the value
            if let Some(name) = &f.ident {
                let vecname = format_ident!("{}_vec", name);
                quote! {
                    #vecname.push(#row_name.#name.clone());
                }
            } else {
                panic!("field in named struct has no name")
            }
        })
        .collect();
    let for_loop = quote! {
        for #row_name in req.iter() {
            #insert_into_vec
        }
    };
    quote! {
        #init_vecs
        #for_loop
    }
}

fn make_unnest_query(fields: &FieldsNamed) -> TokenStream {
    let field_names = fields.named.iter().filter_map(|f| (&f.ident).as_ref());
    let table_str = format!(
        "({})",
        quote! {
            #(#field_names), *
        }
    );
    let field_names = fields.named.iter().filter_map(|f| (&f.ident).as_ref());
    let vec_names = field_names.map(|id| {
        let vecname = format_ident!("{}_vec", id);
        quote! {
            #vecname
        }
    });
    quote! {
        let table_str = &#table_str;
        ::sqlx::query(&format!("
        INSERT INTO {table_name}{table_str}
        SELECT * FROM UNNEST({UNNEST_FMT_STR})"))
        #(.bind(&#vec_names[..]))*
    .execute(client).await?;
    Ok(())
    }
}
