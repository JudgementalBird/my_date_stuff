
use super::utc;

pub struct DateTime {
	pub year_raw: i32,
	pub month_raw: i32,
	pub day_raw: i32,
	pub year_offs: i32,
	pub month_offs: i32,
	pub day_offs: i32,
}
/*pub fn from_unix(unix: u64) -> DateTime {
	use super::{TimeZone,utc};

	let uts_start_gmt = utc::DateTime {
		timezone: TimeZone::GMT,
		year: 2022,
		month: 8,
		day: 0,
		hour: 0,
		minute: 0,
		second: 0,
	};
}*/
pub fn from_utc(utc: super::utc::DateTime) -> DateTime {
	let uts_now = DateTime {
		year_raw: 0,
		month_raw: 0,
		day_raw: 0,
		year_offs: 0,
		month_offs: 0,
		day_offs: 0,
	};
	let uts_start_gmt = utc::DateTime {
		timezone: utc::TimeZone::GMT,
		year: 2022,
		month: 8,
		day: 0,
		hour: 0,
		minute: 0,
		second: 0,
	};

	// years
	//uts_now.year_raw = 
	uts_now //PLACEHOLDER SO NO ERROR
}