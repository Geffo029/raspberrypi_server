use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::{Duration, Instant};
use serde::Serialize;

use crate::measure::Measure;



const DISKSTATS_FILE_PATH: &str = "/proc/diskstats";
const DISK_NAME: &str = "mmcblk0";


#[derive(Serialize)]
pub struct Disk {
	read: Measure,
	write: Measure
}

struct DiskStats {
	parse_instant: Instant,
	stats: Vec<i32>
} impl DiskStats {
	fn sectors_read(&self) -> i32 { self.stats[2] }
	fn sectors_written(&self) -> i32 { self.stats[6] }
}

pub struct DiskParser {
	disk_stats_file: File,
	disk_line_number: usize,
	// prev_total_read_bytes: i32,
	// prev_total_written_bytes: i32,
	prev_disk_stats: DiskStats
} 
impl DiskParser {
	pub fn new() -> Self {
		let mut disk_stats_file = File::open(DISKSTATS_FILE_PATH).expect("No disktats file?!");
		let disk_line_number = Self::find_disk_line_number(&mut disk_stats_file);
		let empty_stats = DiskStats { 
			parse_instant: Instant::now(), 
			stats: vec![0;15] 
		};

		Self {
			disk_stats_file,
			disk_line_number,
			prev_disk_stats: empty_stats
		}
	}

	pub fn parse(&mut self) -> Disk {
		let (read, write) = self.parse_usage();

		Disk {
			read, write
		}
	}

	fn find_disk_line_number(file: &mut File) -> usize {
		let mut buf = String::new();
		file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		file.read_to_string(&mut buf).expect("Cannot read file");
		let mut lines = buf.lines();

		let line_number = lines.position(|line| line.contains(DISK_NAME));
		line_number.unwrap()
	}

	fn parse_usage(&mut self) -> (Measure, Measure) {
		let mut buf = String::new();
		self.disk_stats_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.disk_stats_file.read_to_string(&mut buf).expect("Cannot read file");
		let mut lines = buf.lines();

		fn parse_line(line: &str) -> DiskStats {
			let parse_instant = Instant::now();
			let mut line_splitted = line.split_whitespace();
			line_splitted.nth(2); // skip first values, some NaN
			let stats = line_splitted
				.map(|s| s.parse::<i32>().unwrap())
				.collect::<Vec<_>>();

			DiskStats {
				parse_instant, 
				stats
			 }
		}

		fn calculate_disk_bandwith(disk_stats: &DiskStats, prev_disk_stats: &DiskStats) -> (f32, f32) {
			let delta_read_kb = 0.512 * (disk_stats.sectors_read() - prev_disk_stats.sectors_read()) as f32;
			let delta_written_kb = 0.512 * (disk_stats.sectors_written() - prev_disk_stats.sectors_written()) as f32;
			let delta_time = disk_stats.parse_instant.duration_since(prev_disk_stats.parse_instant);

			let read_kb_bandwith = delta_read_kb / delta_time.as_secs() as f32;
			let write_kb_bandwith = delta_written_kb  / delta_time.as_secs() as f32;
	
			(read_kb_bandwith, write_kb_bandwith)
		}

		let stats = parse_line(lines.nth(self.disk_line_number).unwrap());
		let (read_kb_bandwith, write_kb_bandwith) = calculate_disk_bandwith(&stats, &self.prev_disk_stats);

		self.prev_disk_stats = stats;

		let read = Measure {
			value: read_kb_bandwith,
			unit: String::from("KB/s")
		};
		let write = Measure {
			value: write_kb_bandwith,
			unit: String::from("KB/s")
		};

		(read, write)
	}
}
