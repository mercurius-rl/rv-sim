mod parts;
mod asm_f;

use parts::*;
use argh::FromArgs;

use crate::asm_f::one_line_asm;

static VERSION: &str = "v0.5 Beta";
static UPDATETIME: &str = "June 5 2024";

#[cfg(target_os = "windows")]
fn show_platform() -> String {
	"Windows".to_string()
}

#[cfg(target_os = "macos")]
fn show_platform() -> String {
	"MacOS".to_string()
}

#[cfg(target_os = "linux")]
fn show_platform() -> String {
	"Linux".to_string()
}

#[derive(FromArgs)]
/// Reach new heights.
struct Args {

	#[argh(positional)]
    src: Vec<String>,

	/// run assembler
	#[argh(switch, short = 'a')]
	asm: bool,

	/// output assembled binary
	#[argh(option, short = 'o', default = "String::from(\"a.out\")")]
	output: String,

	/// run risc-v simulator
	#[argh(switch, short = 's')]
	sim: bool,

	/// binary data file
	#[argh(option, short = 'b', default = "String::from(\"a.out\")")]
	bin: String,

	/// debug run
	#[argh(switch, short = 'd')]
	dbg: bool,

	/// memory dump (dump length)
	#[argh(option, short = 'm', default = "0")]
	memdump: u32,

	/// count of maximum instruction execute
	#[argh(option, short = 'c', default = "256")]
	ex_count: i32,
}

fn interactive_run () {
	let mut itrvm = VMachine::new(1024);
	println!("...run RISC-V Interactive mode.");
	loop {
		let pc = itrvm.cpu.pc;
		print!("*:{} >>> ", pc);
		let mut buffer = String::new();
		std::io::stdin().read_line(&mut buffer).ok();
		buffer = buffer.lines().collect::<String>();

		println!("{}",buffer);

		let sp: Vec<&str> = buffer.split_whitespace().collect();

		match sp[0] {
			".info" => {
				println!("reg: {:?}", itrvm.cpu.rf);
			},
			".exit" => {
				println!("end...");
				return;
			},
			".faddr" => {
				if let Err(_) = itrvm.cpu.run() {
					println!("Executed HALT");
					println!("end...");
					return;
				}
			},
			".addr++" => {
				itrvm.cpu.pc += 4;
			},
			".addr--" => {
				if itrvm.cpu.pc >= 4 {
					itrvm.cpu.pc -= 4;
				}
			},
			".show" => {
				if sp.len() == 1 {
					println!("No option.")
				} else {
					let d = itrvm.cpu.memory.read(pc);
					match sp[1] {
						"i" => {
							println!("Address[{:08x}]: {:08x} ({:?})", itrvm.cpu.pc, d, Instruction::decode(d).unwrap());
						},
						"d" => {
							println!("Address[{:08x}]: {:08x}", itrvm.cpu.pc, d);
						},
						"b" => {
							println!("Address[{:08x}]: {:032b}", itrvm.cpu.pc, d);
						},
						_ => {
							println!("Wrong option.")
						}
					}
				}
			}
			_ => {
				let st = one_line_asm(&buffer);
				itrvm.cpu.memory.write(pc, st);
				if let Ok(a) = Instruction::decode(st){
					let s = itrvm.cpu.execute(a);
					if let Err(_) = s {
						println!("Executed HALT");
						println!("end...");
						return;
					}
				}
			}
		}
	}
}

fn main() {
	let mut ag: Args = argh::from_env();

	println!("RISC-V ASM Sim {} ({}) on {}", VERSION, UPDATETIME, show_platform());

	if ag.src.len() == 0 {
		interactive_run();
		return;
	}

	if ag.asm == ag.sim {
		if ag.output != ag.bin {
			println!("Fixed binary data path...");
			ag.output = ag.bin.clone();
		}
		asm_f::asm(&ag.src[0], &ag.output);
		let mut vm = VMachine::new(1024);

		vm.cpu.binread(&ag.bin);
		if ag.dbg {
			vm.start_dbg(ag.ex_count);
		} else {
			vm.start(ag.ex_count);
		}

		if ag.memdump != 0 {
			let d: u32 = vm.cpu.memory.read(0);
			println!("[Memory dump mode]");
			println!("Address[{:08x}]: {:08x} ",0, d);

			for i in 1..(ag.memdump / 4 + 1) {
				let d: u32 = vm.cpu.memory.read(i*4);
				println!("Address[{:08x}]: {:08x} ",i*4, d);
			}
		}
	} else {
		if ag.asm {
			asm_f::asm(&ag.src[0], &ag.output);
		}

		if ag.sim {
			let mut vm = VMachine::new(1024);

			vm.cpu.binread(&ag.bin);
			if ag.dbg {
				vm.start_dbg(ag.ex_count);
			} else {
				vm.start(ag.ex_count);
			}

			if ag.memdump != 0 {
				let d: u32 = vm.cpu.memory.read(0);
				println!("[Memory dump mode]");
				println!("Address[{:08x}]: {:08x} ",0, d);

				for i in 1..(ag.memdump / 4 + 1) {
					let d: u32 = vm.cpu.memory.read(i*4);
					println!("Address[{:08x}]: {:08x} ",0, d);
				}
			}
		}
	}
}
