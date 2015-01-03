
//
// RConstraintsLayout definition
//
// Author: Dave Willmer
// Licence: MIT Licence, 2015
//

// use base_layout::{ RLayout };

// #[derive(RLayout)]
use super::r_layout::{RLayout};

pub struct RConstraintsLayout {
	id: i32
}


impl RConstraintsLayout {
	pub fn new() -> RConstraintsLayout {
		RConstraintsLayout { id: 0 }
	}
}

impl RLayout for RConstraintsLayout {
	fn layout( &self ) {

	}
}