use proc_macro::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(StructBuilder)]
pub fn derive_struct_builder(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let builder = format_ident!("{}Builder", ident);

    match input.data {
        Data::Struct(_struct) => {
            let fields = _struct.fields;

            match fields {
                Fields::Named(_) => {
                    // 获取各字段名称及类型
                    let mut _fields = vec![];
                    for field in fields.iter() {
                        _fields.push((field.ident.as_ref().unwrap(), &field.ty));
                    }

                    let mut builder_fields = quote!();
                    let mut builder_set_fields = quote!();
                    let mut builder_lets = quote!();
                    let mut builder_values = quote!();

                    for _field in _fields {
                        let ident = _field.0;
                        let ty = _field.1;

                        builder_fields.append_all(quote!(#ident: Option<#ty>,));
                        builder_set_fields.append_all(quote!(
                            pub fn #ident(mut self, value: #ty) -> Self {
                                self.#ident = Some(value);
                                self
                            }
                        ));
                        builder_lets.append_all(quote!(
                            let #ident = self.#ident.ok_or(format!("Field \"{}\" not set yet!", stringify!(#ident)))?;
                        ));
                        builder_values.append_all(quote!(#ident,));
                    }

                    // 最终代码
                    let res = quote!(
                        impl #ident {
                            pub fn builder() -> #builder {
                                #builder::default()
                            }
                        }
                        #[derive(Default)]
                        struct #builder {
                            #builder_fields
                        }

                        impl #builder {
                            #builder_set_fields

                            pub fn build(self) -> Result<#ident, String> {
                                #builder_lets

                                Ok(#ident { #builder_values })
                            }
                        }
                    );

                    return res.into();
                }
                _ => {}
            }
        }
        Data::Enum(_) => {}
        Data::Union(_) => {}
    }
    quote!().into()
}
