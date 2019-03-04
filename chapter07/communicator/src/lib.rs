pub mod client;

pub mod network;

pub mod a {
	pub mod series {
		pub mod of {
			pub fn nested_modules(){}
		}
	}
}

pub enum TrafficLight {
	Red,
	Yellow,
	Green,
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
