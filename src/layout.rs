
//
// 
//

struct LayoutItem {
	geometry_cache: &List,
	margin_cache: &List
}





struct TopLevelLayout {
	root_item: LayoutItem,
	solver: CassowarySolver,
	edit_stack: &List,
	layout_items: &List
}



pub trait LayoutManager {
	pub fn set_items( self, items: LayoutItem );
	pub fn clear_items( self );
	pub fn resize( self, width: i32, height: i32 );
	pub fn best_size( self );
	pub fn min_size( self );
	pub fn max_size( self );
	pub fn update_geometry( self );
	pub fn update_margins( self );
}


impl LayoutManager for TopLevelLayout {
	pub fn set_items( self, items: LayoutItem ) {

	}

	pub fn clear_items( self ) {

	}

	pub fn resize( self, width: i32, height: i32 ) {

	}

	pub fn best_size( self ) {

	}

	pub fn min_size( self ) {

	}

	pub fn max_size( self ) {

	}

	pub fn update_geometry( self ) {

	}

	pub fn update_margins( self ) {

	}
}
