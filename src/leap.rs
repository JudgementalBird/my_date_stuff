pub fn is_leap_year(year: u64) -> bool {
// https://en.wikipedia.org/wiki/Leap_year
	
	// An astronomical year lasts slightly less than 3651/4 days. The historic Julian calendar has 
	// three common years of 365 days followed by a leap year of 366 days, by extending February to 29 days 
	// rather than the common 28. The Gregorian calendar, the world's most widely used civil calendar, 
	// makes a further adjustment for the small error in the Julian algorithm. Each leap year has 366 days 
	// instead of 365. This extra leap day occurs in each year that is a multiple of 4, except for years 
	// evenly divisible by 100 but not by 400. 

	((year%4) == 0) && (!((year%100) == 0) | ((year%400) == 0))
}
#[test]
fn test_is_leap_year() {
	assert_eq!(is_leap_year(3), false);
	assert_eq!(is_leap_year(4), true);
	
	assert_eq!(is_leap_year(5), false);
	assert_eq!(is_leap_year(8), true);

	assert_eq!(is_leap_year(200), false);
	assert_eq!(is_leap_year(400), true);
	
	assert_eq!(is_leap_year(1000), false);
	assert_eq!(is_leap_year(2000), true);
	assert_eq!(is_leap_year(2024), true);
}



fn leap_years_up_to(year: u64) -> u64 {
	(year/4) - (year/100) + (year/400)
}
#[test]
fn test_leap_years_up_to() {
	assert_eq!(leap_years_up_to(4),1);
	assert_eq!(leap_years_up_to(2024),491);
}



pub fn leap_years_between(first: u64, second: u64) -> u64 {
	leap_years_up_to(second) - leap_years_up_to(first)
}
#[test]
fn test_leap_years_between() {
	assert_eq!(leap_years_between(1590, 1600), 3);
	assert_eq!(leap_years_between(1700, 1720), 5);
	assert_eq!(leap_years_between(1800, 2024), 55);
}