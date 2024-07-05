use std::time::{SystemTime,UNIX_EPOCH};

pub fn now() -> u64 {
	match SystemTime::now().duration_since(UNIX_EPOCH) {
		Ok(diff) => diff.as_secs(),
		Err(_) => panic!("FUCK you bitch and your stupid fucking time BITCH!!"),
	}
}