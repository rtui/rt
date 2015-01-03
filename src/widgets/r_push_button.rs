

//
// RPushButton implementation
//
// Author: Dave Willmer
// Licence: MIT Licence, 2015
//

// use r_widget;

// #[derive(RWidget, ViewableWidget)]
pub struct RPushButton {
	id: i32,
}


impl RPushButton {
	pub fn new( text: &str ) -> RPushButton {
		RPushButton { 
			id: 0
		}
	}
}