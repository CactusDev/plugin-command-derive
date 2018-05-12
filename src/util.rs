
use syn;
use quote;
use std::vec::Vec;
use std::collections::HashMap;

pub fn get_command_attributes(our_attributes: Vec<&str>, ast: &syn::DeriveInput) -> HashMap<String, String> {
	let mut attributes: HashMap<String, String> = HashMap::new();

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
