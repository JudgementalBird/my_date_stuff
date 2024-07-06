
use crate::gregorian_and_utc::DateTime;

use super::gregorian_and_utc;

pub struct UTSDateTime {
	year_raw: i32,
	month_raw: i32,
	day_raw: i32,
	year_offs: i32,
	month_offs: i32,
	day_offs: i32,
}
impl UTSDateTime {
	pub fn from_utc(utc: DateTime) -> UTSDateTime {
		let uts_now = UTSDateTime {
			year_raw: 0,
			month_raw: 0,
			day_raw: 0,
			year_offs: 0,
			month_offs: 0,
			day_offs: 0,
		};
		let uts_start_gmt = DateTime::initialize(2022, 8, 1, 0, 0, 0);
	
		// years
		//uts_now.year_raw = 
		uts_now //PLACEHOLDER SO NO ERROR
	}
}