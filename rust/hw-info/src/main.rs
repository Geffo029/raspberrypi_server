mod atomicdata;
mod cpu;
mod memory;
mod disk;
mod system;


use std::fs::write;
use std::thread;
use std::time::{Duration, Instant};
use serde::Serialize;

use cpu::{Cpu, CpuParser};
use memory::{Memory, MemoryParser};
use disk::{Disk, DiskParser};
use system::{System, SystemParser};



const HW_INFO_PATH: &str = "/tmp/hwinfos";


#[derive(Serialize)]
struct HwInfos {
	cpu: Cpu,
	memory: Memory,
	disk: Disk,
	system: System
}


fn main() {
    let mut cpu_parser = CpuParser::new();
    let mut memory_parser = MemoryParser::new();
	let mut disk_parser = DiskParser::new();
	let system_parser = SystemParser::new();

	loop {
		let cpu = cpu_parser.parse(); 
		let memory = memory_parser.parse();
		let disk = disk_parser.parse();
		let system = system_parser.parse();

		//let parse_instant = Instant::now();

		let infos = HwInfos {
			cpu,
			memory,
			disk,
			system
		};

		let infos_json = serde_json::to_string(&infos).unwrap();

		write(HW_INFO_PATH, infos_json.as_bytes()).unwrap();

		thread::sleep(Duration::from_secs(1));
	}
}
