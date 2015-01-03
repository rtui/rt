
//
// RPushButton mac implementation
//
// Author: Dave Willmer
// Licence: MIT Licence, 2015
//

extern crate cocoa;
use self::cocoa::base::{ nil };
use self::cocoa::appkit::{ NSButton, NSPoint, NSSize, NSRect };
use super::super::widgets::{ RWidget, RPushButton };


impl RWidget for RPushButton {
	fn createToolkitWidget( self ) {
		// let button = NSButton::alloc(nil).initWithFrame_(
		// 	NSRect::new( NSPoint::new(50., 50.), NSSize::new(50., 50.) )
		// );

		// self.toolkit_widget = button;
	}
}