use heck::ToPascalCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input};

#[proc_macro]
pub fn getter(input: TokenStream) -> TokenStream {
	let ident = parse_macro_input!(input as Ident);
	let base = ident.to_string();

	let pascal = base.to_pascal_case();
	let plural = format!("{base}s");
	let method = format!("get_{base}_by_id");

	let pascal_ident = syn::Ident::new(&format!("Skyblock{pascal}"), ident.span());
	let plural_ident = syn::Ident::new(&plural, ident.span());
	let method_ident = syn::Ident::new(&method, ident.span());

	let expanded = quote! {
		#[doc = concat!("Retrieves a `", stringify!($name), "` by its `internalId`.")]
		#[must_use]
		#[inline]
		pub fn #method_ident(&self, id: &str) -> Option<#pascal_ident> {
			self.#plural_ident.get(&id.to_uppercase()).cloned()
		}
	};

	expanded.into()
}
