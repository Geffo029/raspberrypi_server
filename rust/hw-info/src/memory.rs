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
		
		let lines = lines.collect::<Vec<&str>>();
		let mem_total_kb = parse_line(lines[0]);
		let _mem_free_kb = parse_line(lines[1]);
		let mem_avail_kb = parse_line(lines[2]);
		let _buffers_kb = parse_line(lines[3]);
		let _cached_kb = parse_line(lines[4]);
		let _swap_total_kb = parse_line(lines[14]);
		let _swap_free_kb = parse_line(lines[15]);

		// `free` "used memory" formula: total - available
		let mem_used_kb = mem_total_kb - mem_avail_kb;

		// `htop` "used memory" formula: ...
		
		
		let total_mb = Measure::new(mem_total_kb/1000.0, String::from("MB"));
		let used_mb = Measure::new(mem_used_kb/1000.0, String::from("MB"));
		let used_perc = Measure::new(100.0*mem_used_kb/mem_total_kb, String::from("%"));

		Memory {
			total_mb,
			used_mb,
			used_perc
		}
	}

}
