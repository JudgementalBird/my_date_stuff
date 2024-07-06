//! # Garbo Datetime
//! 
//! This is a bad crate that deals with date and time, made for my own personal uses and uploaded for convenience.<br>
//! Several things are not handled, and many functions will provide incorrect outputs with certain valid inputs. See module documentation for some (not all) of the things that are not handled.

use crate::gregorian_and_utc::DateTime;

fn secs_since_sl_creation() -> u64 {
	let sl_epoch_unix = 1662321182;
	let unix_time = unix::now();
	let slix_time = unix_time-sl_epoch_unix;
	slix_time
}

/// For testing
fn main() {
	//let sl_epoch_gmt = DateTime {
	//	timezone: gregorian_and_utc::TimeZone::GMT,
	//	year: 2022,
	//	month: 8,
	//	day: 4,
	//	hour: 19,
	//	minute: 53,
	//	second: 2,
	//};
//
	//let epoch = DateTime {
	//	timezone: TimeZone::GMT,
	//	year: 1970,
	//	month: 1,
	//	day: 1,
	//	hour: 0,
	//	minute: 0,
	//	second: 0,
	//};

	loop {
		let utcnow = DateTime::now();
		println!("{utcnow:?}");
		
		std::thread::sleep(std::time::Duration::from_secs(1));
	};
}