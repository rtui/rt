

struct RConstraintsWidget {
	left: ConstraintMember,
	top: ConstraintMember,
	width: ConstraintMember,
	height: ConstraintMember,

	right: Constant,
	bottom: Constant,
	h_center: Constant,
	v_center: Constant,

	hug_width: StrengthPolicy,
	hug_height: StrengthPolicy,
	resist_height: StrengthPolicy,
	resist_width: StrengthPolicy,
	limit_width: StrengthPolicy,
	limit_height: StrengthPolicy
}


pub trait Constrainable {
	fn default_right( self ) -> f64;
	fn default_bottom( self ) -> f64;
	fn default_h_center( self ) -> f64;
	fn default_v_center( self ) -> f64;
}


impl Constrainable for ConstraintsWidget {
	fn default_right( self ) -> f64 {
		self.left + self.width
	}

	fn default_bottom( self ) -> f64 {
		self.top + self.height
	}

	fn default_h_center( self ) -> f64 {
		self.left + 0.5 * self.width
	}

	fn default_v_center( self ) -> f64 {
		self.top + 0.5 * self.height
	}
}


//
// 
//
struct RContentsConstraintsWidget {
	contents_left: ConstraintMember,
	contents_right: ConstraintMember,
	contents_top: ConstraintMember,
	contents_bottom: ConstraintMember,

	contents_width: Constant,
	contents_height: Constant,
	contents_h_center: Constant,
	contents_v_center: Constant
}


pub trait ContentsConstrainable {
	fn default_contents_width( self ) -> f64;
	fn default_contents_height( self ) -> f64;
	fn default_contents_h_center( self ) -> f64;
	fn default_contents_c_center( self ) -> f64;
}


impl ContentsConstrainable for ContentsConstrainableWidget {
	fn default_contents_width( self ) -> f64 {
		self.contents_right - self.contents_left
	}

	fn default_contents_height( self ) -> f64 {
		self.contents_bottom - self.contents_top
	}

	fn default_contents_h_center( self ) -> f64 {
		self.contents_left + 0.5*self.contents_width
	}

	fn default_contents_v_center( self ) -> f64 {
		self.contents_top + 0.5*self.contents_height
	}
}





