use common::{struct_fields, tokenize_fields, get_ident};
use quote;
use syn;

pub fn impl_try_from_row(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields = struct_fields(ast).clone();
    let (mapped, non_mapped): (Vec<syn::Field>, Vec<syn::Field>) = fields
        .into_iter()
        .partition(|r| r.attrs.iter().any(|a| match &a.value {
            syn::MetaItem::Word(i) => &i.to_string() == "json_mapped",
            _ => false
        }));
    let mut fields = tokenize_fields(ast, &non_mapped);

    for mapped in mapped {
        let name = mapped.ident.unwrap();
        let ty = get_ident(mapped.ty);

        // Maybe get rid of the clone call
        fields.push(quote! {
            #name: #ty::try_from_row(cdrs.clone())?
        })
    }

    quote! {
      impl TryFromRow for #name {
        fn try_from_row(cdrs: cdrs::types::rows::Row) -> cdrs::Result<Self> {

          Ok(#name {
            #(#fields),*
          })
        }
      }
  }
}