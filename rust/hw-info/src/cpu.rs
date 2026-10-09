use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use serde::Serialize;

use crate::atomicdata::Measure;



const STAT_FILE_PATH: &str = "/proc/stat";
const CPUINFO_FILE_PATH: &str = "/proc/cpuinfo";
// const FREQ_FILE_PATH: &str = "/sys/devices/system/cpu/cpufreq/policy0/scaling_cur_freq";
const FREQ_FILE_PATH: &str = "/sys/devices/system/cpu/cpu*/cpufreq/scaling_cur_freq";
const TEMP_FILE_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";


#[derive(Serialize)]
pub struct Cpu {
	model: String,
	cores: Vec<Core>,
	temp: Measure,
}

#[derive(Serialize)]
struct Core {
	usage: Measure,
    freq: Measure,
    //temp: Measure
}

#[derive(Default, Clone)]
struct CpuTimes {
	times: Vec<i32>
} impl CpuTimes {
	fn total(&self) -> i32 { self.times.iter().sum::<i32>() }
	fn idle(&self) -> i32 { 
        if !self.times.is_empty() {self.times[3] + self.times[4]} else {0} 
    }	
}

/* Note: no need to implement Drop trait to close gracefully a `File`: `File`s
 * are automatically dropped when out of scope
 */
pub struct CpuParser {
	stat_file: File,
	freq_files: Vec<File>,
	temp_file: File,
	core_count: usize,
	model: String,
	prev_cpus_times: Vec<CpuTimes>
}
impl CpuParser {
	pub fn new() -> Self {
		let mut cpuinfo_file = File::open(CPUINFO_FILE_PATH).expect("[CPU] No cpuinfo file?!");
		let core_count = CpuParser::parse_core_count(&mut cpuinfo_file);
		let model = CpuParser::parse_cpu_model(&mut cpuinfo_file);

		let freq_files = std::iter::repeat(FREQ_FILE_PATH).take(core_count).enumerate().map(
			|(index, file_path)| {
				let file_path = file_path.replace("*", format!("{index}").as_str());
				File::open(file_path).expect("")
			}).collect::<Vec<File>>();


		let stat_file = File::open(STAT_FILE_PATH).expect("[CPU] No stat file?!");
        // let freq_file = File::open(FREQ_FILE_PATH).expect("[CPU] No freq file?!");
		let temp_file = File::open(TEMP_FILE_PATH).expect("[CPU] No temp file?!");
 		//let empty_times = CpuTimes { times: vec![0;10] };

 		let empty_times_vec = Vec::new();


		Self { 
			stat_file,
			freq_files,
			temp_file,
			core_count,
			model,
			prev_cpus_times: empty_times_vec
		}
	}

	pub fn parse(&mut self) -> Cpu {
		let usages = self.parse_usages();
		let freqs = self.parse_freq();
		let temp = self.parse_temp();

		let cores = usages.into_iter().zip(freqs.into_iter()).map( 
            |(usage, freq)| Core{usage, freq} 
        ).collect::<Vec<Core>>();

		Cpu {
			model: self.model.clone(),
			cores,
			temp
		}
	}

	fn parse_usages(&mut self) -> Vec<Measure> {
		let mut buf = String::new();
		self.stat_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		self.stat_file.read_to_string(&mut buf).expect("Cannot read file");
		let mut lines = buf.lines();

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
			let usage = 100.0 * (1.0 - delta_idle as f32 / delta_total as f32);
			usage
		}

		//let cpu_times = parse_cpu_times(lines[0]);
		//let usage = calculate_cpu_usage(&cpu_times, &self.prev_cpu_times);

		lines.next(); //skip `cpu ` line, first one
		let cpus_times = lines
			//.filter( |line| line.starts_with("cpu") )
			.take(self.core_count)
			.map( |line| parse_cpu_times(line) );

		let curr_cpus_times = cpus_times.clone();
		let prev_cpus_times = self.prev_cpus_times.clone().into_iter();
		
		let usages = {
			curr_cpus_times.zip(
				prev_cpus_times
					.chain(std::iter::repeat_with(|| CpuTimes::default()))
			)
			.map( |(times, prev_times)| calculate_cpu_usage(&times, &prev_times) )
		};

		self.prev_cpus_times = cpus_times.collect::<Vec<CpuTimes>>();

		// //Measure::new(usage, String::from("%"))
		usages.map( |usage| Measure::new(usage, String::from("%")) ).collect::<Vec<Measure>>()
	}

	fn parse_freq(&mut self) -> Vec<Measure> {
		// self.freq_file.seek(SeekFrom::Start(0)).expect("");
		// self.freq_file.read_to_string(&mut buf).expect("");

		// for mut freq_file in self.freq_files {
		// 	freq_file.seek(SeekFrom::Start(0)).expect("");
		// 	freq_file.read_to_string(&mut buf).expect("");
		// }

		let freqs = self.freq_files.iter().map(
			|mut freq_file| {
				let mut buf = String::new();
				//dbg!(freq_file);
				freq_file.seek(SeekFrom::Start(0)).expect("");
				freq_file.read_to_string(&mut buf).expect("");
				let freq = buf.trim().parse::<f32>().unwrap();
				freq
			});

		

		//Measure::new(freq/1000.0, String::from("MHz"))
		freqs.map( |freq| Measure::new(freq, String::from("MHz")) ).collect::<Vec<Measure>>()
	}

	fn parse_temp(&mut self) -> Measure {
		let mut buf = String::new();
		self.temp_file.seek(SeekFrom::Start(0)).expect("");
		self.temp_file.read_to_string(&mut buf).expect("Cannot read file");

		let temp = buf.trim().parse::<f32>().unwrap(); 

		Measure::new(temp/1000.0, String::from("^C"))
	}

	fn parse_core_count(cpuinfo_file: &mut File) -> usize {
		let mut buf = String::new();
		cpuinfo_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		cpuinfo_file.read_to_string(&mut buf).expect("Cannot read file");
		let lines =  buf.lines();

		let cores_count = lines.filter(|line| line.starts_with("processor")).count();
		cores_count
	}

	fn parse_cpu_model(cpuinfo_file: &mut File) -> String {
		let mut buf = String::new();
		cpuinfo_file.seek(SeekFrom::Start(0)).expect("Cannot seek file");
		cpuinfo_file.read_to_string(&mut buf).expect("Cannot read file");
		let mut lines =  buf.lines();

		let cpu_name = {
			match lines.find(|line| line.starts_with("model name")) {
				Some(name) => String::from(name),
				None => String::from(""),
			}
		};

		cpu_name
	}

}
