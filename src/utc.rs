use crate::leap::is_leap_year;

#[derive(Debug)]
pub enum TimeZone {
	GMT
}

#[derive(Debug)]
pub struct DateTime {
	pub timezone: TimeZone,
	pub year: u64,
	pub month: u64,
	pub day: u64,
	pub hour: u64,
	pub minute: u64,
	pub second: u64,
}
impl DateTime {
	pub fn add_sec_rollover(&mut self, sec: u64) {
		
		let std_month_length = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
		let mut used_month_length = std_month_length;
		if is_leap_year(self.year) {
				used_month_length[1] = 29;
		}
		let used_month_length = used_month_length;

		self.second += sec;

		// every whole minute is taken from second count and added to minute count
		while self.second > 59 {
				self.second -= 60;
				self.minute += 1;
		};
		// every whole hour is taken from minute count and added to hour count
		while self.minute > 59 {
				self.minute -= 60;
				self.hour += 1;
		};
		// every whole day cycle is taken from hour count and added to day cycle count
		while self.hour > 23 {
				self.hour -= 24;
				self.day += 1;
		};
		let kind_to_use = true;
		if kind_to_use {
			// every whole dynamic length month is taken from day count and added to month count
			// if month count surpasses amount in a year, year is taken from month count and added to year count,
			// and leap year check is performed
			loop {
				let used_month_length = if self.month > 12 {
					self.year += 1;
					self.month -= 12;
	
					let mut used_month_length = std_month_length;
					if is_leap_year(self.year) {
						used_month_length[1] = 29;
					}
					used_month_length
				} else {
					used_month_length
				};
	
				if self.day > used_month_length[self.month as usize - 1] {
					self.day -= used_month_length[self.month as usize - 1];
					self.month += 1;
				} else {
					break;
				};
			};
		} else {
			// every whole dynamic length month is taken from day count and added to month count,
			// incrementing the year if we reach month 13
			// This code does not yet account for leap years
			while self.day > std_month_length[self.month as usize - 1] as u64 {
				self.day -= std_month_length[self.month as usize - 1] as u64;
				self.month += 1;
				if self.month > 12 {
					self.month = 1;
					self.year += 1;
				}
			}
		}
		
	}
	pub fn from_unix(time: u64) -> DateTime {
		let mut utc_unix_epoch = DateTime {
			timezone: TimeZone::GMT,
			year: 1970,
			month: 1,
			day: 1,
			hour: 0,
			minute: 0,
			second: 0,
		};
		
		utc_unix_epoch.add_sec_rollover(time);
		
		utc_unix_epoch
	}
}