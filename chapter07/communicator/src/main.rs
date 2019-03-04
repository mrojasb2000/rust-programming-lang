extern crate communicator;

//use communicator::a::series::of;
//use communicator::a::series::of::nested_modules;
use communicator::a::series::of::*;

//use communicator::TrafficLight::{Red, Yellow};

use communicator::TrafficLight::*;

fn main() {
    communicator::client::connect();
    communicator::network::connect();
    communicator::network::server::connect();
	
	// Calling a qualified module name
	communicator::a::series::of::nested_modules();
	
	//of::nested_modules();
	nested_modules();
	
	let _red = Red;
	let _yellow = Yellow;
	let _green = communicator::TrafficLight::Green;
}
