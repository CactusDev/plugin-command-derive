
use syn;
use quote;

pub fn impl_base_command_execute(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;
	if let syn::Body::Struct(_) = ast.body {
		quote! {
			impl BaseCommand for #name {
				fn execute() {
					println!("Woah we're a thing! {}", stringify!(#name));
				}
			}
		}
	} else {
		panic!("#[derive(BaseCommand)] is only intended for structs.");
	}
}
