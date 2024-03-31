use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod util;

#[proc_macro_derive(Iter)]
pub fn derive_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = input.data;
	let fields = util::get_fields(&data);
	let ty = util::get_type(&fields);

	let arms = {
		let mut arms = TokenStream::new();

		for (index, field) in fields.iter().enumerate() {
			let ident = field.ident.as_ref().unwrap().to_string();

			arms.extend(quote! {
				#index => #ident,
			});
		}

		arms
	};

	let expanded = quote! {
		pub struct IntoIter<'a> {
			inner: &'a #name,
			index: usize,
		}

		impl<'a> Iterator for IntoIter<'a> {
			type Item = (&'a str, #ty);

			fn next(&mut self) -> Option<Self::Item> {
				let index = match self.index {
					#arms
					_ => return None,
				};

				self.index += 1;

				Some((index, self.inner.get(index).unwrap()))
			}
		}

		impl<'a> IntoIterator for &'a #name {
			type Item = (&'a str, #ty);
			type IntoIter = IntoIter<'a>;

			fn into_iter(self) -> Self::IntoIter {
				IntoIter {
					inner: self,
					index: 0,
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Get)]
pub fn derive_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = input.data;
	let fields = util::get_fields(&data);
	let ty = util::get_type(&fields);

	let arms = {
		let mut arms = TokenStream::new();

		for field in fields {
			let ident = field.ident.as_ref().unwrap();
			let index = ident.to_string();

			arms.extend(quote! {
				#index => Some(self.#ident),
			});
		}

		arms
	};

	let expanded = quote! {
		impl #name {
			pub fn get(&self, index: &str) -> Option<#ty> {
				match index {
					#arms
					_ => None,
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Set)]
pub fn derive_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = input.data;
	let fields = util::get_fields(&data);
	let ty = util::get_type(&fields);

	let arms = {
		let mut arms = TokenStream::new();

		for field in fields {
			let ident = field.ident.as_ref().unwrap();
			let index = ident.to_string();

			arms.extend(quote! {
				#index => self.#ident = value,
			});
		}

		arms
	};

	let expanded = quote! {
		impl #name {
			pub fn set(&mut self, index: &str, value: #ty) -> Result<(), Box<dyn std::error::Error>> {
				match index {
					#arms
					_ => return Err(format!("Field: {} does not exist", index).into()),
				}

				Ok(())
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}
