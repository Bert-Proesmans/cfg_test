#![cfg_attr(test, deny(missing_docs))]

pub mod hello {
	pub struct Testing;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
