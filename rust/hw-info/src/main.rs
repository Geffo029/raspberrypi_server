use std::fs::{File, write};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::thread;
use std::time::{Duration, Instant};
use serde::Serialize;

mod measure;

mod cpu;
use cpu::{Cpu, CpuParser};
mod memory;
use memory::{Memory, MemoryParser};


const HW_INFO_PATH: &str = "/tmp/hwinfos";



#[derive(Serialize)]
struct HwInfos {
	cpu: Cpu,
	memory: Memory
}

fn main() {
    let mut cpu_parser = CpuParser::new();
    let mut memory_parser = MemoryParser::new();
	//thread::sleep(Duration::from_secs(1));

	loop {
		let cpu = cpu_parser.parse(); 
		let memory = memory_parser.parse();

		let infos = HwInfos {
			cpu,
			memory
		};

		let infos_json = serde_json::to_string(&infos).unwrap();

		write(HW_INFO_PATH, infos_json.as_bytes()).unwrap();

		thread::sleep(Duration::from_secs(1));
	}
}


/*
fn main() -> io::Result<()> {
    let mut stat_file = File::open(STAT_FILE_PATH)?;
    let mut hwinfo_file = File::open(HW_INFO_PATH)?;

    let mut buf = String::new();
    loop {
        buf.clear();

        // torna all'inizio e rileggi
        stat_file.seek(SeekFrom::Start(0))?;
        stat_file.read_to_string(&mut buf)?;

        write(HW_INFO_PATH, buf.as_bytes())?;
 
        thread::sleep(Duration::from_secs(1));
    }
}
*/
