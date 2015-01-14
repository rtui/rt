


extern crate "rust-windows" as windows;


struct RMainWindowDelegate {
	mut title: ~str,
	height: int.
	width: int,
	app: int, // FIXME
	window: int // FIXME
}

impl RMainWindowDelegate {
	pub fn new() -> RMainWindow {
		unsafe {
		}
	}

	pub fn show( &self ) {

	}

	pub fn setTitle( &self, title: &str ) {
		self.title = title;
	}
}