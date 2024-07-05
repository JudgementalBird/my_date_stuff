use super::unix;

pub fn now() -> u64 {
	let sl_epoch_unix = 1662321182;
	let unix_time = unix::now();
	let slix_time = unix_time-sl_epoch_unix;
	slix_time
}