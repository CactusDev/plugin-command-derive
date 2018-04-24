
use syn;
use quote;

fn find_command_name_from_derive_input(ast: &syn::DeriveInput) -> String {
	let attrs = &ast.attrs;

	for attr in attrs {
		// Make sure this attribute is the kind we're looking for
		if let syn::MetaItem::NameValue(ref name, ref value) = attr.value {
			if name != "CommandName" {
				continue;
			}
			// And then we only want string values here.
			if let &syn::Lit::Str(ref name, _) = value {
				return name.to_string();
			}
		}
	}
	panic!("Must specify the command's name: #[CommandName = \"name_here\"]")
}

pub fn impl_base_command_on(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;
	let command_name = find_command_name_from_derive_input(ast);

	if let syn::Body::Struct(_) = ast.body {
		quote! {
			impl BaseCommand for #name {
				fn on(arguments: Vec<&str>, handler: Box<Fn(Vec<String>, String, String, bool) -> Option<String>>) -> String {
					println!("Woah we're a thing! {:?} {}", arguments, stringify!(#name));
					handler(vec! [String::from("Test")], "testing".to_string(), "channel".to_string(), true);
					command_name
				}
			}
		}
	} else {
		panic!("#[derive(BaseCommand)] is only intended for structs.");
	}
}
