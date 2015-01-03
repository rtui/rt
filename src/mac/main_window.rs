


// extern crate libc;
extern crate core_foundation;
extern crate cocoa;

use cocoa::base::{ NSUInteger, selector, nil};
use cocoa::appkit::{ NSApp, NSRect, NSPoint, NSSize,
					 NSApplication, NSWindow, NSString,
					 NSMenu, NSMenuItem, NSTitledWindowMask,
					 NSBackingStoreBuffered };



struct RMainWindowDelegate {
	mut title: ~str,
	height: int,
	width: int,
	app: NSApp,
	window: NSWindow
}

impl RMainWindowDelegate {
	
	pub fn new() -> RMainWindow {

		unsafe {
			self.app = NSApp();
			self.window = window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
				NSRect::new(NSPoint::new(0.,0.), NSSize::new(200., 200.)),
				NSTitledWindowMask as NSUInteger,
				NSBackingStoreBuffered,
				false
			);

			window.cascadeTopLeftFromPoint_( NSPoint::new(20., 20.) );
			window.center();

			let title = NSString::alloc(nil).init_str("Howdy\0");
			window.setTitle_(title);
			window.makeKeyAndOrderFront_(nil);
		}

		self.RMainWindow { title:"Test", height:480, width:640 }
	}

	pub fn show( &self ) {
		app.activateIgnoringOtherApps_(true);
		app.run();
	}

	pub fn setTitle( &self, title: &str ) {
		self.title = title;
	}
}