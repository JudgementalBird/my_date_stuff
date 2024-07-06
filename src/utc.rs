use crate::{leap::is_leap_year, unix};

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

		self.minute = self.second/60;
		self.second = self.second%60;

		self.hour = self.minute/60;
		self.minute = self.minute%60;

		self.day = self.hour/24+1; //I have no idea why I have to add 1 for this to be accurate again!
		self.hour = self.hour%24;
		
		while self.day > std_month_length[self.month as usize - 1] {
			self.day -= std_month_length[self.month as usize - 1];
			self.month += 1;
			if self.month > 12 {
				self.month = 1;
				self.year += 1;
			}
		}
		// remove leap days making sure to wrap backwards, because that happens to be the amount of days we need to remove to make it accurate... I have no idea why...
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

	pub fn now() -> DateTime {
		DateTime::from_unix(unix::now())
	}
}