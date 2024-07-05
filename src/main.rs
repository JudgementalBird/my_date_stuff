use utc::DateTime;

mod unix;
mod slix;
mod uts;
mod utc;
mod leap;

fn main() {
	let sl_epoch_gmt = utc::DateTime {
		timezone: utc::TimeZone::GMT,
		year: 2022,
		month: 8,
		day: 4,
		hour: 19,
		minute: 53,
		second: 2,
	};

	println!("{}",unix::now());
	let utcnow = DateTime::from_unix(unix::now());

	println!("{utcnow:?}");
	println!("{}",leap::leap_years_between(1970,2024));
}