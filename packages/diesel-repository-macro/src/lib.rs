use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Ident, LitStr, Token};

#[proc_macro_derive(RepositoryAsync, attributes(
    repo_table_name,    // e.g. #[repo_table_name = "users"]
    id_type,            // e.g. #[id_type = "i32"]
    create_type,        // Optional override for save_one input type.
    repo_features,      // e.g. #[repo_features(get_one, save_one, delete_one, find_page, fts_search)]
    fts_fields          // e.g. #[fts_fields(name, description)]
))]
pub fn derive_repository_async(input: TokenStream) -> TokenStream {
    // Parse the derive input.
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    // Initialize holders for our attributes.
    let mut repo_table_name_str: Option<String> = None;
    let mut id_type_str: Option<String> = None;
    let mut create_type_str: Option<String> = None;
    let mut features: Vec<String> = Vec::new();
    let mut fts_field_list: Vec<String> = Vec::new();

    // Iterate over all attributes on the struct.
    for attr in &ast.attrs {
        if attr.path().is_ident("repo_table_name") {
            // Expect an attribute like: #[repo_table_name = "users"]
            let lit: LitStr = attr
                .parse_args()
                .expect("Expected a string literal for repo_table_name");
            repo_table_name_str = Some(lit.value());
        } else if attr.path().is_ident("id_type") {
            // Expect an attribute like: #[id_type = "i32"]
            let lit: LitStr = attr
                .parse_args()
                .expect("Expected a string literal for id_type");
            id_type_str = Some(lit.value());
        } else if attr.path().is_ident("create_type") {
            // Optional: #[create_type = "NewUser"]
            let lit: LitStr = attr
                .parse_args()
                .expect("Expected a string literal for create_type");
            create_type_str = Some(lit.value());
        } else if attr.path().is_ident("repo_features") {
            // Expect a comma‐separated list, e.g. #[repo_features(get_one, save_one, delete_one, find_page, fts_search)]
            let features_punct: Punctuated<Ident, Token![,]> = attr
                .parse_args_with(Punctuated::parse_terminated)
                .expect("Failed to parse repo_features attribute");
            for ident in features_punct.iter() {
                features.push(ident.to_string());
            }
        } else if attr.path().is_ident("fts_fields") {
            // Expect a comma‐separated list, e.g. #[fts_fields(name, description)]
            let fields_punct: Punctuated<Ident, Token![,]> = attr
                .parse_args_with(Punctuated::parse_terminated)
                .expect("Failed to parse fts_fields attribute");
            for ident in fields_punct.iter() {
                fts_field_list.push(ident.to_string());
            }
        }
    }

    // Ensure mandatory attributes are provided.
    let repo_table_name_str =
        repo_table_name_str.expect("Missing #[repo_table_name = \"...\"] attribute on struct");
    let id_type_str = id_type_str.expect("Missing #[id_type = \"...\"] attribute on struct");

    // Parse the id_type and create_type into Rust types.
    let id_type: syn::Type =
        syn::parse_str(&id_type_str).expect("Failed to parse id_type as a Rust type");
    // If create_type is not provided, default to the struct itself.
    let default_create = struct_name.to_string();
    let create_type_str = create_type_str.unwrap_or(default_create);
    let create_type: syn::Type =
        syn::parse_str(&create_type_str).expect("Failed to parse create_type as a Rust type");

    // Create an identifier for the Diesel table (we expect a schema module with that name).
    let repo_table_name_ident = syn::Ident::new(&repo_table_name_str, struct_name.span());
    // Generate a repository struct name, e.g. UserRepo.
    let repo_struct_ident = syn::Ident::new(&format!("{}Repo", struct_name), struct_name.span());

    // Prepare a vector to hold the generated impl blocks.
    let mut impls = Vec::new();

    // --- Generate the GetOneByRepoAsync impl if requested ---
    if features.contains(&"get_one".to_string()) {
        let code = quote! {
            use diesel_repository::GetOneByRepoAsync;

            #[allow(non_snake_case)]
            impl GetOneByRepoAsync<#struct_name, #id_type> for #repo_struct_ident {
                async fn get_one_by_id(
                    conn: &mut diesel_async::AsyncPgConnection,
                    id_val: #id_type
                ) -> diesel::result::QueryResult<#struct_name> {
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;
                    use crate::schema::#repo_table_name_ident::dsl::*;
                    #repo_table_name_ident.find(id_val)
                        .first::<#struct_name>(conn)
                        .await
                }
            }
        };
        impls.push(code);
    }

    // --- Generate the SaveOneRepoAsync impl if requested ---
    if features.contains(&"save_one".to_string()) {
        let code = quote! {
            use diesel_repository::SaveOneRepoAsync;

            #[allow(non_snake_case)]
            impl SaveOneRepoAsync<#struct_name> for #repo_struct_ident {
                async fn save_one(
                    conn: &mut diesel_async::AsyncPgConnection,
                    data: impl Into<#struct_name> + Send
                ) -> diesel::result::QueryResult<#struct_name> {
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;
                    use crate::schema::#repo_table_name_ident::dsl::*;
                    let entity = data.into();
                    diesel::insert_into(#repo_table_name_ident)
                        .values(&entity)
                        .on_conflict_do_nothing()
                        .get_result(conn)
                        .await
                }
            }
        };
        impls.push(code);
    }

    // --- Generate the DeleteOneRepoAsync impl if requested ---
    if features.contains(&"delete_one".to_string()) {
        let code = quote! {
            use diesel_repository::DeleteOneRepoAsync;

            #[allow(non_snake_case)]
            impl DeleteOneRepoAsync<#id_type> for #repo_struct_ident {
                async fn delete_one(
                    conn: &mut diesel_async::AsyncPgConnection,
                    id_val: #id_type
                ) -> diesel::result::QueryResult<usize> {
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;
                    use crate::schema::#repo_table_name_ident::dsl::*;
                    diesel::delete(#repo_table_name_ident.find(id_val))
                        .execute(conn)
                        .await
                }
            }
        };
        impls.push(code);
    }

    // --- Generate the FindPageRepoAsync impl if requested ---
    if features.contains(&"find_page".to_string()) {
        let code = quote! {
            use diesel_repository::FindPageRepoAsync;

            #[allow(non_snake_case)]
            impl FindPageRepoAsync<#struct_name> for #repo_struct_ident {
                async fn find_page(
                    conn: &mut diesel_async::AsyncPgConnection,
                    page: i64,
                    size: i64
                ) -> diesel::result::QueryResult<diesel_repository::Paged<#struct_name>> {
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;
                    use crate::schema::#repo_table_name_ident::dsl::*;
                    let total = #repo_table_name_ident
                        .count()
                        .get_result::<i64>(conn)
                        .await?;
                    let offset_val = (page - 1) * size;
                    let items = #repo_table_name_ident
                        .offset(offset_val)
                        .limit(size)
                        .load::<#struct_name>(conn)
                        .await?;
                    Ok(diesel_repository::Paged {
                        items,
                        total,
                        page,
                        size,
                    })
                }
            }
        };
        impls.push(code);
    }

    // --- Generate the FullTextSearchRepoAsync impl if requested ---
    if features.contains(&"fts_search".to_string()) {
        // Build the tsvector expression from the provided fts_fields.
        let fts_concat = if !fts_field_list.is_empty() {
            let parts: Vec<String> = fts_field_list
                .iter()
                .map(|field| format!("COALESCE({field}, '')"))
                .collect();
            let joined = parts.join(" || ' ' || ");
            format!("to_tsvector('simple', {})", joined)
        } else {
            "to_tsvector('simple', '')".to_string()
        };

        let code = quote! {
            use diesel_repository::FullTextSearchRepoAsync;
            use diesel_repository::Count;

            #[allow(non_snake_case)]
            impl FullTextSearchRepoAsync<#struct_name> for #repo_struct_ident {
                async fn full_text_search(
                    conn: &mut diesel_async::AsyncPgConnection,
                    query: &str,
                    page: i64,
                    size: i64
                ) -> diesel::result::QueryResult<diesel_repository::Paged<#struct_name>> {
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;
                    use diesel::sql_types::Text;
                    use crate::schema::#repo_table_name_ident::dsl::*;
                    let tsvector_expr = #fts_concat;
                    let count_sql = format!(
                        "SELECT COUNT(*) FROM {} WHERE {} @@ plainto_tsquery('simple', $1)",
                        #repo_table_name_str, tsvector_expr
                    );
                    let Count{count: total}: Count = diesel::sql_query(count_sql)
                        .get_result(conn)
                        .await?;
                    //let total = result.count;
                    let offset_val = (page - 1) * size;
                    let items_sql = format!(
                        "SELECT * FROM {} WHERE {} @@ plainto_tsquery('simple', $1) OFFSET {} LIMIT {}",
                        #repo_table_name_str,
                        tsvector_expr,
                        offset_val,
                        size
                    );
                    let items: Vec<#struct_name> = diesel::sql_query(items_sql)
                        .bind::<Text, _>(query)
                        .load(conn)
                        .await?;
                    Ok(diesel_repository::Paged {
                        items,
                        total,
                        page,
                        size,
                    })
                }
            }
        };
        impls.push(code);
    }

    // --- Combine the repository struct definition with all the impl blocks ---
    let expanded = quote! {
        /// Auto-generated async repository for the model.
        pub struct #repo_struct_ident;
        #(#impls)*
    };

    expanded.into()
}
