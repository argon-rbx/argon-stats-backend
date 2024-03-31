#[cfg(test)]
mod tests {
	use struct_util::{Get, Iter, Set};

	#[test]
	fn it_works() {
		#[derive(Debug, Iter, Get, Set)]
		struct Test {
			a: u32,
			b: u32,
		}

		let mut test = Test { a: 42, b: 8 };

		for (key, value) in &test {
			println!("{}: {:?}", key, value);
		}

		test.set("b", 1337).unwrap();

		println!("{:?}", test);
	}
}
