
use syn;
use quote;

use std::vec::Vec;

use util::get_command_attributes;

pub fn impl_base_command_on(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;
	if let syn::Body::Struct(_) = ast.body {
		let (command_name, subcommands) = {
			let result = get_command_attributes(vec! ["CommandName", "SubCommands"], ast);

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
