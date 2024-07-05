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

	loop {
		let utcnow = DateTime::from_unix(unix::now());
		println!("{utcnow:?}");
		std::thread::sleep(std::time::Duration::from_secs(1));
	};
}