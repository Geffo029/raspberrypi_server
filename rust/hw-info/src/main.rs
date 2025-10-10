mod measure;
mod cpu;
mod memory;
mod disk;


use std::fs::write;
use std::thread;
use std::time::Duration;
use serde::Serialize;

use cpu::{Cpu, CpuParser};
use memory::{Memory, MemoryParser};
use disk::{Disk, DiskParser};



const HW_INFO_PATH: &str = "/tmp/hwinfos";


#[derive(Serialize)]
struct HwInfos {
	cpu: Cpu,
	memory: Memory,
	disk: Disk
}

fn main() {
    let mut cpu_parser = CpuParser::new();
    let mut memory_parser = MemoryParser::new();
	let mut disk_parser = DiskParser::new();

	loop {
		let cpu = cpu_parser.parse(); 
		let memory = memory_parser.parse();
		let disk = disk_parser.parse();

		let infos = HwInfos {
			cpu,
			memory,
			disk
		};

		let infos_json = serde_json::to_string(&infos).unwrap();

		write(HW_INFO_PATH, infos_json.as_bytes()).unwrap();

		thread::sleep(Duration::from_secs(1));
	}
}
