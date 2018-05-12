
use syn;
use quote;

use std::vec::Vec;

use util::get_command_attributes;

struct CommandManagerInternals {
	
}

pub fn impl_command_manager_setup(ast: &syn::DeriveInput) -> quote::Tokens {
	let name = &ast.ident;

	if let syn::Body::Struct(_) = ast.body {
		let commands = get_command_attributes(vec! ["Commands"], ast).get("Commands").unwrap().replace(" ", "").to_string();
		let commands = commands.split(",").collect::<Vec<&str>>();

		quote! {
			impl CommandManager for #name {
				fn setup() {
					let commands: Vec<&str> = #commands.to_vec();


				}

				fn commands() -> Vec<&'static str> {
					#commands.to_vec()
				}
			}
		}
	} else {
		panic!("#[derive(CommandManager)] is only intended for structs.");
	}
}
