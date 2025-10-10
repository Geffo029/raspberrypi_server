use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::{Duration, Instant};
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

struct RawStats {
	instant: Instant,
	times: Vec<i32>
} impl RawStats {
	fn total(&self) -> i32 { self.times.iter().sum::<i32>() }
	fn idle(&self) -> i32 { self.times[3] + self.times[4] }
}

struct CpuTimes {
	times: Vec<i32>
} impl CpuTimes {
	fn total(&self) -> i32 { self.times.iter().sum::<i32>() }
	fn idle(&self) -> i32 { self.times[3] + self.times[4] }	
}

/* Note: do not need to implement Drop trait to close gracefully a File: File are automatically
dropped when out of scope
 */
pub struct CpuParser {
    stat_file: File,
    temp_file: File,
    freq_file: File,
    //prev_stats: RawStats,
    prev_cpu_times: CpuTimes
}
impl CpuParser {
    pub fn new() -> Self {
        let stat_file = match File::open(STAT_FILE_PATH) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };
        let temp_file = match File::open(TEMP_FILE_PATH) {
             Ok(file) => file,
             Err(err) => panic!("{}", err),
        };
		let freq_file = File::open(FREQ_FILE_PATH).expect("No freq file?!");
 		/*let empty_stats = RawStats {
 			instant: Instant::now(),
 			times: vec![0; 10]
 		};*/
 		let empty_times = CpuTimes {
 			times: vec![0;10]
 		};

		Self { 
			stat_file,
			temp_file,
			freq_file,
			prev_cpu_times: empty_times 
		}
	}

	pub fn parse(&mut self) -> Cpu {
		/*let usage = self.parse_usage_from_file().expect("Cannot parse usage");
		let temp = self.parse_temp_from_file().expect("Cannot parse temp");*/
		// USAGE
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
		let usage = Measure {
			value: calculate_cpu_usage(&cpu_times, &self.prev_cpu_times),
			unit: String::from("%")
		};
		self.prev_cpu_times = cpu_times;

		// FREQ
		let mut buf = String::new();
		self.freq_file.seek(SeekFrom::Start(0)).expect("");
		self.freq_file.read_to_string(&mut buf).expect("");

		let freq = buf.trim().parse::<f32>().unwrap();
		let freq = Measure {
			value: freq/1000.0,
			unit: String::from("MHz")
		};

		// TEMP
		let mut buf = String::new();
		self.temp_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.temp_file.read_to_string(&mut buf).expect("Cannot read file");

		let temp = buf.trim().parse::<f32>().unwrap(); 
		let temp = Measure {
			value: temp / 1000.0,
			unit: String::from("^C")
		};

		Cpu {
			usage,
			freq,
			temp
		}
	}
	
	/*
	fn parse_usage_from_file(&mut self) -> Option::<Measure> {
		let mut measure_opt = None::<Measure>;

		let mut buf = String::new();
       	self.stat_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
       	self.stat_file.read_to_string(&mut buf).expect("Cannot read file");

		let read_instant = Instant::now();
		
		for line in buf.lines() {
			let mut line_splitted = line.split_whitespace();
			let header = line_splitted.next().unwrap();

			match header {
				"cpu" => {
					let times_iter = line_splitted.map(|time_str| time_str.parse::<i32>().unwrap());
					
					let raw_stats = RawStats { 
						instant: read_instant,
						times: times_iter.collect::<Vec<i32>>()
					};

					let usage = self.calculate_cpu_usage(raw_stats);
					let unit = String::from("%");

					/*return Ok(Measure {
						value: usage,
						unit	
					});*/
					measure_opt = Some(
						Measure {
							value: usage,
							unit	
						}
					);
				},
				//"procs_running" => println!("PR"),
				_ => ()
			}
		}

		measure_opt
    }

	
	fn calculate_cpu_usage(&mut self, raw_stats: RawStats) -> f32 {
		let delta_idle = raw_stats.idle() - self.prev_stats.idle();
		let delta_total = raw_stats.total() - self.prev_stats.total(); 
		//dbg!(delta_idle);
		//dbg!(delta_total);		

		let usage = 100.0 * (1.0 - delta_idle as f32 / delta_total as f32);
		//dbg!(usage);
		
		self.prev_stats = raw_stats;

		usage
	}

	fn parse_temp_from_file(&mut self) -> Option::<Measure> {
		let mut measure_opt = None::<Measure>;

		let mut buf = String::new();
		self.temp_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.temp_file.read_to_string(&mut buf).expect("Cannot read file");

		let temp = buf.trim().parse::<f32>().unwrap(); 
		//let temp = 20000.0;
		let temp = temp / 1000.0;
		//dbg!(temp);

		if temp > 0.0 {
			measure_opt = Some(
				Measure {
					value: temp,
					unit: String::from("^C")
				}
			)
		}

		measure_opt
	}
	*/

}
