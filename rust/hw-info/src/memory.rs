use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use serde::Serialize;

use crate::measure::Measure;


const MEMINFO_FILE_PATH: &str = "/proc/meminfo";


#[derive(Serialize)]
pub struct Memory {
	total_mb: Measure,
	used_mb: Measure,
	used_perc: Measure
}

struct RawMemInfo {
	mem_total_kb: i32,
	mem_free_kb: i32,
	swap_total_kb: i32,
	swap_free_kb: i32
} impl RawMemInfo {
	
}


pub struct MemoryParser {
	mem_info_file: File
}
impl MemoryParser {
	pub fn new() -> Self {
		let mem_info_file = File::open(MEMINFO_FILE_PATH).expect("No meminfo?!");

		Self {
			mem_info_file
		}
	}

	pub fn parse(&mut self) -> Memory {
		let mut buf = String::new();
		self.mem_info_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.mem_info_file.read_to_string(&mut buf).expect("Cannot read file");
		let lines = buf.lines();

		fn parse_line(line: &str) -> f32 {
			let mut line_splitted = line.split_whitespace();
			line_splitted.nth(1).unwrap().parse::<f32>().unwrap()
		}
		
 		/*let mem_total_kb = Self::parse_line(&lines.next());
 		let mem_free_kb = Self::parse_line(&lines.next());
 		let mem_avail_kb = Self::parse_line(&lines.next());
		let buffers_kb = Self::parse_line(&lines.next());
		let cached_kb = Self::parse_line(&lines.nth(4));
		let swap_total_kb = Self::parse_line(&lines.nth(14));
		let swap_free_kb = Self::parse_line(&lines.nth(15));*/
		let lines = lines.collect::<Vec<&str>>();
		let mem_total_kb = parse_line(lines[0]);
		let mem_free_kb = parse_line(lines[1]);
		let mem_avail_kb = parse_line(lines[2]);
		let buffers_kb = parse_line(lines[3]);
		let cached_kb = parse_line(lines[4]);
		let swap_total_kb = parse_line(lines[14]);
		let swap_free_kb = parse_line(lines[15]);
		
		let mem_used_kb = mem_total_kb - mem_avail_kb;

		let total_mb = Measure {
			value: mem_total_kb / 1000.0,
			unit: String::from("MB")
		};
		let used_mb = Measure {
			value: mem_used_kb / 1000.0,
			unit: String::from("MB")
		};
		let used_perc = Measure {
			value: 100.0 * mem_used_kb / mem_total_kb,
			unit: String::from("%")
		};

		Memory {
			total_mb,
			used_mb,
			used_perc
		}
	}

	/*
	fn read_file(&mut self) -> RawMemInfo {
		let mut buf = String::new();
		self.mem_info_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.mem_info_file.read_to_string(&mut buf).expect("Cannot read file");

		let mut lines = buf.lines();
 		let mem_total_kb = Self::parse_line(&lines.nth(0));
		let mem_free_kb = Self::parse_line(&lines.nth(1));
		let swap_total_kb = Self::parse_line(&lines.nth(14));
		let swap_free_kb = Self::parse_line(&lines.nth(15));

		RawMemInfo {
			mem_total_kb,
			mem_free_kb,
			swap_total_kb,
			swap_free_kb
		}
	}
	*/

	/*
	fn parse_line(/*line_opt: &Option::<&str>*/ line: &str) -> i32 {
		//dbg!(line_opt);
		let mut line_splitted = line.split_whitespace();
		// dbg!(&line_splitted);
		line_splitted.nth(1).unwrap().parse::<i32>().unwrap()
	}
	*/

}
