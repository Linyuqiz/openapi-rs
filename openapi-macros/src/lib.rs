use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed};

#[proc_macro_derive(With)]
pub fn derive_with(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(DataStruct {
            fields: syn::Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("With宏仅支持具名字段的结构体"),
    };

    let methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let method_name = format_ident!("with_{}", field_name);

        // 提取Option内部类型
        let inner_type = match &field.ty {
            syn::Type::Path(type_path) => {
                let segments = &type_path.path.segments;
                if let Some(segment) = segments.last() {
                    if segment.ident == "Option" {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                                ty
                            } else {
                                panic!("Option类型参数不合法");
                            }
                        } else {
                            panic!("Option类型参数不合法");
                        }
                    } else {
                        panic!("字段必须为Option类型");
                    }
                } else {
                    panic!("无效的类型路径");
                }
            }
            _ => panic!("不支持的非Option字段类型"),
        };

        quote! {
            #[doc = "设置字段 `"#field_name "` 的值"]
            #[inline]
            pub fn #method_name(mut self, value: #inner_type) -> Self {
                self.#field_name = Some(value);
                self
            }
        }
    });

    let expanded = quote! {
        impl #struct_name {
            #(#methods)*
        }
    };

    expanded.into()
}