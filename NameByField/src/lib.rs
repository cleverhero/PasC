#![crate_type = "proc-macro"]

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(name_by_field)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();

    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");

    let gen = impl_name_by_field(ast);

    gen.to_string().parse().expect("couldn't parse string to tokens")
}

fn impl_name_by_field(ast: syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
	match ast.body {
	    syn::Body::Enum(fields) => {
	    	let conditions = fields.iter().map(|f| {
                let f_name = &f.ident;
                let s = f_name.to_string();

                quote!(
                	#name::#f_name => { return #s.to_string(); }
                )
            });

	        let q = quote! {
	            impl #impl_generics #name #ty_generics #where_clause {
	                pub fn fields_name(&self) -> String {
	                	match *self {
	                		#(#conditions), *
	                	}
	                    "none".to_string()
	                }
	            }
	        };

	    	q
	    },
	    _ => panic!("#[derive(name_by_field)] can only be used with enums"),
	}
}