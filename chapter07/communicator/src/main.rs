extern crate communicator;

//use communicator::a::series::of;
use communicator::a::series::of::nested_modules;

fn main() {
    communicator::client::connect();
    communicator::network::connect();
    communicator::network::server::connect();
	
	// Calling a qualified module name
	communicator::a::series::of::nested_modules();
	
	//of::nested_modules();
	nested_modules();
}
