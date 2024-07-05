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
		let start_year = self.year;

		let std_month_length = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
		// every whole dynamic length month is taken from day count and added to month count,
		// incrementing the year if we reach month 13
		while self.day > std_month_length[self.month as usize - 1] as u64 {
			self.day -= std_month_length[self.month as usize - 1] as u64;
			self.month += 1;
			if self.month > 12 {
				self.month = 1;
				self.year += 1;
			}
		}
		// remove leap days making sure to wrap backwards, because that happens to be the amount of days we need to remove to make it accurate...
		let leap_days = crate::leap::leap_years_between(start_year, self.year);
		if leap_days < self.day {
			self.day -= leap_days;
		} else if leap_days >= self.day {
			self.month -= 1;
			if self.month == 0 { // if we went past january, go to previous year (month 0 is month 12)
				self.month = 12;
				self.year -= 1;
			}

			self.day += std_month_length[self.month as usize - 1];
			self.day -= leap_days;
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