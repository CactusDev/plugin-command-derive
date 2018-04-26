
use syn;
use quote;

use std::collections::HashMap;
use std::vec::Vec;

fn get_command_attributes(ast: &syn::DeriveInput) -> HashMap<String, String> {
	let mut attributes: HashMap<String, String> = HashMap::new();
	let our_attributes: Vec<&str> = vec! ["CommandName", "SubCommands"];

	let attrs = &ast.attrs;
	for attr in attrs {
		if let syn::MetaItem::NameValue(ref name, ref value) = attr.value {
			if let &syn::Lit::Str(ref string, _) = value {

				if !our_attributes.contains(&name.as_ref()) {
					// We don't care about this attribute
					continue
				}
				attributes.insert(name.to_string(), string.to_string());
			}
		}
	}
	attributes
}

pub fn impl_base_command_on(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;
	if let syn::Body::Struct(_) = ast.body {
		let (command_name, subcommands) = {
			let result = get_command_attributes(ast);

			let name = result.get("CommandName").unwrap_or(&"".to_string()).to_string();
			let sub = result.get("SubCommands").unwrap_or(&"".to_string()).to_string();
			(name, sub)
		};

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
