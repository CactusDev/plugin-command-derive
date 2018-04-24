
mod command;

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(BaseCommand, attributes(CommandName))]
pub fn on(input: TokenStream) -> TokenStream {
	let name = input.to_string();
	let ast = syn::parse_derive_input(&name).unwrap();

	let gen = command::impl_base_command_on(&ast);
	gen.parse().unwrap()
}
