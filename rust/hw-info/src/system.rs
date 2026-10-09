use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::atomicdata::Measure;

#[derive(Serialize)]
pub struct System {
	current_time_str: String,
}

pub struct SystemParser {
	
} 
impl SystemParser {
	pub fn new() -> Self {
		/*use std::time::{SystemTime, UNIX_EPOCH};
		println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
		let date_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
		Self {
			current_time_str: format!("{date_now:?}")
		}*/
		Self {}
	}

	pub fn parse(&self) -> System {
		/*use std::time::{SystemTime, UNIX_EPOCH};
		println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
		let date_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();*/
		
		let now = Utc::now();
		System {
			current_time_str: format!("{}", now)
		}
	}
}
