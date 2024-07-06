//! # Leap year related functions
//! 
//! Several functions related to leap years provide objectively wrong outputs with certain years, for simplicity of implementation and because I don't need them to work in those cases.<br>
//! For example, they do not prevent the use of years of the Gregorian calendar from before it was adopted. In reality, skipped years, timezones, the julian calender, and other stuff should probably be factored in.


/// Calculates if a given u64 year is a leap year in the Gregorian calendar.
/// 
/// Made based on what wikipedia writes here: https://en.wikipedia.org/wiki/Leap_year <br>
/// "[...] Each leap year has 366 days instead of 365. This extra leap day occurs in each year that is a multiple of 4, except for years evenly divisible by 100 but not by 400."
/// ```
///	assert_eq!(is_leap_year(3), false);
///	assert_eq!(is_leap_year(4), true);
///	
///	assert_eq!(is_leap_year(5), false);
///	assert_eq!(is_leap_year(8), true);
///
///	assert_eq!(is_leap_year(200), false);
///	assert_eq!(is_leap_year(400), true);
///	
///	assert_eq!(is_leap_year(1000), false);
///	assert_eq!(is_leap_year(2000), true);
///	assert_eq!(is_leap_year(2024), true);
/// ```
pub fn is_leap_year(year: u64) -> bool {
	((year%4) == 0) && (!((year%100) == 0) | ((year%400) == 0))
}


/// Calculates how many of the years up until a given u64 year, were leap years.
/// 
/// ```
/// assert_eq!(leap_years_up_to(4),1);
/// assert_eq!(leap_years_up_to(2024),491);
/// ```
fn leap_years_up_to(year: u64) -> u64 {
	(year/4) - (year/100) + (year/400)
}


/// Calculates how many of the years between two given u64 years, were leap years.
/// 
/// ```
/// assert_eq!(leap_years_between(1590, 1600), 3);
/// assert_eq!(leap_years_between(1700, 1720), 5);
/// assert_eq!(leap_years_between(1800, 2024), 55);
/// ```
pub fn leap_years_between(first: u64, second: u64) -> u64 {
	leap_years_up_to(second) - leap_years_up_to(first)
}