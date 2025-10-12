use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use serde::Serialize;

use crate::measure::Measure;



const STAT_FILE_PATH: &str = "/proc/stat";
const TEMP_FILE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";
const FREQ_FILE_PATH: &str = "/sys/devices/system/cpu/cpufreq/policy0/scaling_cur_freq";


#[derive(Serialize)]
pub struct Cpu {
    usage: Measure,
    freq: Measure,
    temp: Measure
}

struct CpuTimes {
	times: Vec<i32>
} impl CpuTimes {
	fn total(&self) -> i32 { self.times.iter().sum::<i32>() }
	fn idle(&self) -> i32 { self.times[3] + self.times[4] }	
}

/* Note: no need to implement Drop trait to close gracefully a File: File are automatically
dropped when out of scope
 */
pub struct CpuParser {
	stat_file: File,
	temp_file: File,
	freq_file: File,
	prev_cpu_times: CpuTimes
}
impl CpuParser {
	pub fn new() -> Self {
		let stat_file = File::open(STAT_FILE_PATH).expect("[CPU] No stat file?!");
		let temp_file = File::open(TEMP_FILE_PATH).expect("[CPU] No temp file?!");
        let freq_file = File::open(FREQ_FILE_PATH).expect("[CPU] No freq file?!");
 		let empty_times = CpuTimes { times: vec![0;10] };

		Self { 
			stat_file,
			temp_file,
			freq_file,
			prev_cpu_times: empty_times 
		}
	}

	pub fn parse(&mut self) -> Cpu {
		let usage = self.parse_usage();
		let freq = self.parse_freq();
		let temp = self.parse_temp();

		Cpu {
			usage,
			freq,
			temp
		}
	}

	fn parse_usage(&mut self) -> Measure {
		let mut buf = String::new();
		self.stat_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.stat_file.read_to_string(&mut buf).expect("Cannot read file");
		let lines =  buf.lines().collect::<Vec<&str>>();

		fn parse_cpu_times(line: &str) -> CpuTimes {
			let mut line_splitted = line.split_whitespace();
			line_splitted.next(); // skip line header
			CpuTimes {
				times: line_splitted
					.map(|s| s.parse::<i32>().unwrap())
					.collect::<Vec<i32>>()
			}
		}

		fn calculate_cpu_usage(curr_times: &CpuTimes, prev_times: &CpuTimes) -> f32 {
			let delta_idle = curr_times.idle() - prev_times.idle();
			let delta_total = curr_times.total() - prev_times.total(); 
			//dbg!(delta_idle);
			//dbg!(delta_total);		
			let usage = 100.0 * (1.0 - delta_idle as f32 / delta_total as f32);
			//dbg!(usage);
			usage
		}

		let cpu_times = parse_cpu_times(lines[0]);
		let usage = calculate_cpu_usage(&cpu_times, &self.prev_cpu_times);

		self.prev_cpu_times = cpu_times;

	 	Measure::new(usage, String::from("%"))
	}

	fn parse_freq(&mut self) -> Measure {
		let mut buf = String::new();
		self.freq_file.seek(SeekFrom::Start(0)).expect("");
		self.freq_file.read_to_string(&mut buf).expect("");

		let freq = buf.trim().parse::<f32>().unwrap();

		Measure::new(freq/1000.0, String::from("MHz"))
	}

	fn parse_temp(&mut self) -> Measure {
		let mut buf = String::new();
		self.temp_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.temp_file.read_to_string(&mut buf).expect("Cannot read file");

		let temp = buf.trim().parse::<f32>().unwrap(); 

		Measure::new(temp/1000.0, String::from("^C"))
	}

}
