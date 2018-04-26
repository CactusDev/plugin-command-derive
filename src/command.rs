
use syn;
use quote;

fn find_command_name_from_derive_input(ast: &syn::DeriveInput) -> String {
	let attrs = &ast.attrs;

	for attr in attrs {
		if let syn::MetaItem::NameValue(ref name, ref value) = attr.value {
			if name != "CommandName" {
				continue;
			}
			if let &syn::Lit::Str(ref string, _) = value {
				return string.to_string()
			}
		}
	}
	panic!("Must specify the command's name: #[CommandName = \"name_here\"]")
}

pub fn impl_base_command_on(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;
	if let syn::Body::Struct(_) = ast.body {
		let command_name = find_command_name_from_derive_input(ast);

		quote! {
			impl BaseCommand for #name {
				fn on(arguments: Vec<&str>, handler: Box<Fn(Vec<String>, String, String, bool) -> Option<String>>) {
					handler(vec! [String::from("Test")], "testing".to_string(), "channel".to_string(), true);
				}

				fn command_name() -> &'static str {
					return stringify!(#command_name);
				}
			}
		}
	} else {
		panic!("#[derive(BaseCommand)] is only intended for structs.");
	}
}
