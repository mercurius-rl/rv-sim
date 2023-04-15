use std::fs::File;
use std::io::{BufRead, BufReader};

const K: u32 = 1024;
const M: u32 = K * K; 


// Memory implementation
#[derive(Debug)]
pub struct Memory {
	data: Vec<u8>,
}

const MEM_SIZE: u32 = 1 * M;

impl Default for Memory {
	fn default() -> Self {
		Self {
			data: vec![0; MEM_SIZE as usize],
		}
	}
}

#[allow(dead_code)]
impl Memory {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn size(s: u32) -> Self{
		Memory {
			data: vec![0; s as usize],
		}
	}

	pub fn len(&self) -> usize{
		self.data.len()
	}

	pub fn initialize(&mut self, data: Vec<u8>) {
		self.data.splice(0..data.len(), data);
	}
}

pub trait ReadWrite<T> {
	fn write(&mut self, addr: u32, data: T);

	fn read(&self, addr: u32) -> T;
}

impl ReadWrite<u8> for Memory {
	fn write(&mut self, addr: u32, data: u8) {
		let index = addr as usize;
		self.data[index] = data;
	}

	fn read(&self, addr: u32) -> u8 {
		let index = addr as usize;
		self.data[index]
	}
}

impl ReadWrite<u16> for Memory {
	fn write(&mut self, addr: u32, data: u16) {
		let index = addr as usize;
		self.data[index] = (data & 0xff) as u8;
		self.data[index + 1] = ((data >> 8) & 0xff) as u8;
	}

	fn read(&self, addr: u32) -> u16 {
		let index = addr as usize;
		self.data[index] as u16 
		| (self.data[index + 1] as u16) << 8
	}
}

impl ReadWrite<u32> for Memory {
	fn write(&mut self, addr: u32, data: u32) {
		let index = addr as usize;
		self.data[index] = (data & 0xff) as u8;
		self.data[index + 1] = ((data >> 8) & 0xff) as u8;
		self.data[index + 2] = ((data >> 16) & 0xff) as u8;
		self.data[index + 3] = ((data >> 24) & 0xff) as u8;
	}

	fn read(&self, addr: u32) -> u32 {
		let index = addr as usize;
		self.data[index] as u32 
		| (self.data[index + 1] as u32) << 8
		| (self.data[index + 2] as u32) << 16
		| (self.data[index + 3] as u32) << 24
	}
}

// Register
#[derive(Debug)]
pub struct Reg {
	f: [u32; 32],
}

#[allow(dead_code)]
impl Reg {
	pub fn new() -> Self {
		Default::default()
	}
}

impl Default for Reg {
	fn default() -> Self {
		Self {
			f: [0; 32]
		}
	}
}

impl ReadWrite<u8> for Reg {
	fn write(&mut self, addr: u32, data: u8) {
		let index = addr as usize;
		self.f[index] = (self.f[index] & 0x000000FF) | data as u32;
	}

	fn read(&self, addr: u32) -> u8 {
		let index = addr as usize;
		self.f[index] as u8
	}
}

impl ReadWrite<u16> for Reg {
	fn write(&mut self, addr: u32, data: u16) {
		let index = addr as usize;
		self.f[index] = (self.f[index] & 0x0000FFFF) | data as u32;
	}

	fn read(&self, addr: u32) -> u16 {
		let index = addr as usize;
		self.f[index] as u16
	}
}

impl ReadWrite<u32> for Reg {
	fn write(&mut self, addr: u32, data: u32) {
		let index = addr as usize;
		self.f[index] = data;
	}

	fn read(&self, addr: u32) -> u32 {
		let index = addr as usize;
		self.f[index] as u32 
	}
}

// Control and Status Register implementation
#[derive(Debug)]
pub struct Csr {
	mstatus : u32,
	mie     : u32,
	mtvec   : u32,
	mepc    : u32,
	mcause  : u32,
	mtval   : u32,
	mip     : u32,
}

impl Default for Csr {
	fn default() -> Self {
		Self {
			mstatus : 0,
			mie     : 0,
			mtvec   : 0,
			mepc    : 0,
			mcause  : 0,
			mtval   : 0,
			mip     : 0,
		}
	}
}

impl ReadWrite<u32> for Csr {
	fn write(&mut self, addr: u32, data: u32) {
		match addr {
			0x300 => {self.mstatus = data}
			0x304 => {self.mie = data}
			0x305 => {self.mtvec = data}
			0x341 => {self.mepc = data}
			0x342 => {self.mcause = data}
			0x343 => {self.mtval = data}
			0x344 => {self.mip = data}
			_ =>{}
		}
	}

	fn read(&self, addr: u32) -> u32 {
		match addr {
			0x300 => {self.mstatus},
			0x304 => {self.mie},
			0x305 => {self.mtvec},
			0x341 => {self.mepc},
			0x342 => {self.mcause},
			0x343 => {self.mtval},
			0x344 => {self.mip},
			_ => 0
		}
	}
}



// Decoder implementation
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	Lui  {rd: u32, imm: u32},
	Auipc{rd: u32, imm: u32},

	Add  {rd: u32, rs1: u32, rs2: u32},
	Sub  {rd: u32, rs1: u32, rs2: u32},
	Or   {rd: u32, rs1: u32, rs2: u32},
	And  {rd: u32, rs1: u32, rs2: u32},
	Xor  {rd: u32, rs1: u32, rs2: u32},
	Sll  {rd: u32, rs1: u32, rs2: u32},
	Srl  {rd: u32, rs1: u32, rs2: u32},
	Sra  {rd: u32, rs1: u32, rs2: u32},
	Slt  {rd: u32, rs1: u32, rs2: u32},
	Sltu {rd: u32, rs1: u32, rs2: u32},

	Addi {rd: u32, rs1: u32, imm: u32},
	Ori  {rd: u32, rs1: u32, imm: u32},
	Andi {rd: u32, rs1: u32, imm: u32},
	Xori {rd: u32, rs1: u32, imm: u32},
	Slli {rd: u32, rs1: u32, imm: u32},
	Srli {rd: u32, rs1: u32, imm: u32},
	Srai {rd: u32, rs1: u32, imm: u32},
	Slti {rd: u32, rs1: u32, imm: u32},
	Sltiu{rd: u32, rs1: u32, imm: u32},

	Beq  {rs1: u32, rs2: u32, off: u32},
	Bne  {rs1: u32, rs2: u32, off: u32},
	Blt  {rs1: u32, rs2: u32, off: u32},
	Bge  {rs1: u32, rs2: u32, off: u32},
	Bltu {rs1: u32, rs2: u32, off: u32},
	Bgeu {rs1: u32, rs2: u32, off: u32},

	Lw   {rd: u32, rs1: u32, off: u32},
	Lh   {rd: u32, rs1: u32, off: u32},
	Lb   {rd: u32, rs1: u32, off: u32},
	Lhu  {rd: u32, rs1: u32, off: u32},
	Lbu  {rd: u32, rs1: u32, off: u32},
	Sw   {rs1: u32, rs2: u32, off: u32},
	Sh   {rs1: u32, rs2: u32, off: u32},
	Sb   {rs1: u32, rs2: u32, off: u32},

	Jal  {rd: u32, off: u32},
	Jalr {rd: u32, rs1: u32, off: u32},

	Csrrw{rd: u32, csr: u32, rs1: u32},
	Csrrs{rd: u32, csr: u32, rs1: u32},
	Csrrc{rd: u32, csr: u32, rs1: u32},
	Csrrwi{rd: u32, csr: u32, imm: u32},
	Csrrsi{rd: u32, csr: u32, imm: u32},
	Csrrci{rd: u32, csr: u32, imm: u32},

	Nop,
	Halt,
}

impl Instruction {
	pub fn decode(inst: u32) -> Result<Instruction, i32> {
		let op = inst & 0x0000007F;
		let rd = (inst & 0x00000F80) >> 7;
		let rs1 = (inst & 0x000F8000) >> 15;
		let rs2 = (inst & 0x01F00000) >> 20;

		let funct3 = (inst & 0x00007000) >> 12;
		let funct7 = (inst & 0xFE000000) >> 25;

		match op {
			// U-Type
			0b0110111 => {
				let imm = inst & 0xFFFFF000;
				Ok(Instruction::Lui{rd, imm})
			},
			0b0010111 => {
				let imm = inst & 0xFFFFF000;
				Ok(Instruction::Auipc{rd, imm})
			},

			// I-Type
			0b0010011 => {
				let imm = (inst & 0xFFF00000) >> 20;

				match funct3 {
					0x0 => Ok(Instruction::Addi{rd, rs1, imm}),
					0x1 => Ok(Instruction::Slli{rd, rs1, imm}),
					0x2 => Ok(Instruction::Slti{rd, rs1, imm}),
					0x3 => Ok(Instruction::Sltiu{rd, rs1, imm}),
					0x4 => Ok(Instruction::Xori{rd, rs1, imm}),
					0x5 => match funct7 {
						0x20 => Ok(Instruction::Srai{rd, rs1, imm: rs2}),
						0x00 => Ok(Instruction::Srli{rd, rs1, imm}),
						_ => Ok(Instruction::Nop),
					},
					0x6 => Ok(Instruction::Ori{rd, rs1, imm}),
					0x7 => Ok(Instruction::Andi{rd, rs1, imm}),
					_ => Ok(Instruction::Nop),
				}
			},
			0b0000011 => {
				let off = (inst & 0xFFF00000) >> 20;

				match funct3 {
					0x0 => Ok(Instruction::Lb{rd, rs1, off}),
					0x1 => Ok(Instruction::Lh{rd, rs1, off}),
					0x2 => Ok(Instruction::Lw{rd, rs1, off}),
					//0x3
					0x4 => Ok(Instruction::Lbu{rd, rs1, off}),
					0x5 => Ok(Instruction::Lhu{rd, rs1, off}),
					_ => Ok(Instruction::Nop),
				}
			},

			// R-Type
			0b0110011 => {
				match funct3 {
					0x0 => match funct7 {
						0x20 => Ok(Instruction::Sub{rd, rs1, rs2}),
						0x00 => Ok(Instruction::Add{rd, rs1, rs2}),
						_ => Ok(Instruction::Nop),
					},
					0x1 => Ok(Instruction::Sll{rd, rs1, rs2}),
					0x2 => Ok(Instruction::Slt{rd, rs1, rs2}),
					0x3 => Ok(Instruction::Sltu{rd, rs1, rs2}),
					0x4 => Ok(Instruction::Xor{rd, rs1, rs2}),
					0x5 => match funct7 {
						0x20 => Ok(Instruction::Sra{rd, rs1, rs2}),
						0x00 => Ok(Instruction::Srl{rd, rs1, rs2}),
						_ => Ok(Instruction::Nop),
					},
					0x6 => Ok(Instruction::Or{rd, rs1, rs2}),
					0x7 => Ok(Instruction::And{rd, rs1, rs2}),
					_ => Ok(Instruction::Nop),
				}
			},

			// B-TYpe
			0b1100011 => {
				let imm = (rd & 0x1E) 
					| (funct7 & 0x3F) << 5 
					| (rd & 0x1) << 11 
					| (funct7 & 0x40) << 21;

				match funct3 {
					0x0 => Ok(Instruction::Beq  {rs1, rs2, off: imm}),
					0x1 => Ok(Instruction::Bne  {rs1, rs2, off: imm}),
					//0x2
					//0x3
					0x4 => Ok(Instruction::Blt  {rs1, rs2, off: imm}),
					0x5 => Ok(Instruction::Bge  {rs1, rs2, off: imm}),
					0x6 => Ok(Instruction::Bltu {rs1, rs2, off: imm}),
					0x7 => Ok(Instruction::Bgeu {rs1, rs2, off: imm}),
					_ => Ok(Instruction::Nop),
				}
			},

			// S-Type
			0b0100011 => {
				let off = ((inst & 0xFE000000) >> 20) + ((inst & 0xF80) >> 7);

				match funct3 {
					0x0 => Ok(Instruction::Sb{rs1, rs2, off}),
					0x1 => Ok(Instruction::Sh{rs1, rs2, off}),
					0x2 => Ok(Instruction::Sw{rs1, rs2, off}),

					0x7 => Ok(Instruction::Halt), // Addition
					_ => Ok(Instruction::Nop),
				}
			},

			// J-Type
			0b1101111 => {
				let off = (funct3 << 12)
						+ (rs1 << 15)
						+ (rs2 & 0x1E)
						+ ((rs2 & 0x1) << 11)
						+ ((funct7 & 0x3F) << 5)
						+ ((funct7 & 0x40) << 20);

				Ok(Instruction::Jal{rd, off})
			},
			0b1100111 => {
				let off = (inst & 0xFFF00000) >> 20;

				Ok(Instruction::Jalr{rd, rs1, off})
			},

			//csr
			0b1110011 => {
				let csr = (inst & 0xFE000000) >> 20;

				match funct3 {
					0x0 => Ok(Instruction::Csrrw{rd, csr, rs1}),
					0x1 => Ok(Instruction::Csrrs{rd, csr, rs1}),
					0x2 => Ok(Instruction::Csrrc{rd, csr, rs1}),

					0x5 => Ok(Instruction::Csrrwi{rd, csr, imm:rs1}),
					0x6 => Ok(Instruction::Csrrsi{rd, csr, imm:rs1}),
					0x7 => Ok(Instruction::Csrrci{rd, csr, imm:rs1}),
					
					_ => Ok(Instruction::Nop),
				}
			}

			// HALT
			0b1111111 => {
				Ok(Instruction::Halt)
			},
			_ => Ok(Instruction::Nop),
		}
	}
}

pub struct CPU {
	pub pc: u32,
	pub rf: Reg,
	pub csr: Csr,
	pub memory: Memory,
}

#[allow(dead_code)]
impl CPU {
	pub fn init(&mut self, data: Vec<u8>) {
		self.memory.initialize(data);
	}

	pub fn fetch(&self) -> u32 {
		self.memory.read(self.pc)
	}

	#[allow(unused_must_use)]
	pub fn run(&mut self) -> Result<(),()> {
		let raw_inst = self.fetch();
		let inst = Instruction::decode(raw_inst);

		if let Ok(i) = inst {
			self.execute(i.clone());
			if i != Instruction::Halt {
				return Ok(());
			}
		}

		Err(())
	}

	#[warn(unused_must_use)]
	pub fn execute(&mut self, inst: Instruction) -> Result<(), i32> {
		match inst {
			Instruction::Lui{rd, imm} => {
				self.rf.write(rd, imm);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Auipc{rd, imm} => {
				let pc = self.pc;
				self.rf.write(rd, imm + pc);
				self.pc += 4;
				Result::Ok(())
			}
			Instruction::Add{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = r1 + r2;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sub{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = r1 - r2;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Or{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = r1 | r2;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::And{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = r1 & r2;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Xor{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = r1 ^ r2;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sll{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = r1 << r2;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Srl{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = r1 >> r2;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sra{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = ((r1 as i32) << (r2 as i32)) as u32;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Slt{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = if (r1 as i32) < (r2 as i32) {
					1
				}
				else {
					0
				};
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sltu{rd, rs1, rs2} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				let value: u32 = if r1 < r2 {
					1
				}
				else {
					0
				};
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Addi{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value = r1 + imm;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Ori{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value = r1 | imm;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Andi{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value = r1 & imm;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Xori{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value = r1 ^ imm;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Slli{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value: u32 = r1 << imm;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Srli{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value = r1 as u32 >> imm;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Srai{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value: u32 = (r1 as i32 >> imm as i32) as u32;
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Slti{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value: u32 = if (r1 as i32) < (imm as i32) {
					1
				}
				else {
					0
				};
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sltiu{rd, rs1, imm} => {
				let r1: u32 = self.rf.read(rs1); 
				let value: u32 = if r1 < imm {
					1
				}
				else {
					0
				};
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Beq{rs1, rs2, off} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				if r1 == r2 {
					self.pc += off;
				}
				Result::Ok(())
			},
			Instruction::Bne{rs1, rs2, off} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				if r1 != r2 {
					self.pc += off;
				}
				Result::Ok(())
			},
			Instruction::Blt{rs1, rs2, off} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				if (r1 as i32) < (r2 as i32) {
					self.pc += off;
				}
				Result::Ok(())
			},
			Instruction::Bge{rs1, rs2, off} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				if (r1 as i32) >= (r2 as i32) {
					self.pc += off;
				}
				Result::Ok(())
			},
			Instruction::Bltu{rs1, rs2, off} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				if r1 < r2  {
					self.pc += off;
				}
				Result::Ok(())
			},
			Instruction::Bgeu{rs1, rs2, off} => {
				let (r1, r2): (u32,u32) = (self.rf.read(rs1), self.rf.read(rs2)); 
				if r1 >= r2 {
					self.pc += off;
				}
				Result::Ok(())
			},

			Instruction::Lw{rd, rs1, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = (r1 as i32 + off as i32) as u32;
				let value: u32 = self.memory.read(addr);
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Lh{rd, rs1, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = (r1 as i32 + off as i32) as u32;
				let value: u16 = self.memory.read(addr);
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Lb{rd, rs1, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = (r1 as i32 + off as i32) as u32;
				let value: u8 = self.memory.read(addr);
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Lhu{rd, rs1, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = r1 + off;
				let value: u16 = self.memory.read(addr);
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Lbu{rd, rs1, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = r1 + off;
				let value: u8 = self.memory.read(addr);
				self.rf.write(rd, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sw{rs1, rs2, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = r1 + off;
				let value: u32 = self.rf.read(rs2);
				self.memory.write(addr, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sh{rs1, rs2, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = r1 + off;
				let value: u16 = self.rf.read(rs2);
				self.memory.write(addr, value);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Sb{rs1, rs2, off} => {
				let r1: u32 = self.rf.read(rs1); 
				let addr = r1 + off;
				let value: u8 = self.rf.read(rs2);
				self.memory.write(addr, value);
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Jal{rd, off} => {
				self.rf.write(rd, self.pc + 4);
				self.pc += off;
				Result::Ok(())
			},

			Instruction::Jalr{rd, rs1, off} => {
				let doff: u32 = self.rf.read(rs1);
				self.rf.write(rd, self.pc + 4);
				self.pc = doff + off;
				Result::Ok(())
			},

			Instruction::Csrrw{rd, csr, rs1} => {
				let t = self.csr.read(csr);
				let s: u32 = self.rf.read(rs1);
				self.csr.write(csr, s); 
				self.rf.write(rd, t);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Csrrs{rd, csr, rs1} => {
				let t = self.csr.read(csr);
				let s: u32 = self.rf.read(rs1);
				self.csr.write(csr, t | s); 
				self.rf.write(rd, t);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Csrrc{rd, csr, rs1} => {
				let t = self.csr.read(csr);
				let s: u32 = self.rf.read(rs1);
				self.csr.write(csr, t & !s); 
				self.rf.write(rd, t);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Csrrwi{rd, csr, imm} => {
				let t = self.csr.read(csr);
				self.csr.write(csr, imm); 
				self.rf.write(rd, t);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Csrrsi{rd, csr, imm} => {
				let t = self.csr.read(csr);
				self.csr.write(csr, t | imm); 
				self.rf.write(rd, t);
				self.pc += 4;
				Result::Ok(())
			},
			Instruction::Csrrci{rd, csr, imm} => {
				let t = self.csr.read(csr);
				self.csr.write(csr, t & !imm); 
				self.rf.write(rd, t);
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Nop => {
				self.pc += 4;
				Result::Ok(())
			},

			Instruction::Halt => {
				self.pc += 4;
				Result::Err(0)
			}
		}
	}

	pub fn binread(&mut self, path: &str) {
		let file = File::open(path).unwrap();
		let reader = BufReader::new(file);

		let mut i: u32 = 0;
		for line in reader.lines() {
			let line = line.unwrap();

			let s = u32::from_str_radix(&line, 16).unwrap();
			self.memory.write(i, s);
			i += 4;
		}
	}
}

pub struct VMachine {
	pub cpu: CPU,
	pub icount: i32,
}

impl VMachine {
	pub fn new(size: u32) -> VMachine {
		Self {
			cpu: CPU{
				pc: 0,
				rf: Reg::default(),
				csr: Csr::default(),
				memory: Memory::size(size),
			},
			icount: 0,
		}
	}

	pub fn start(&mut self, ic: i32) {
		loop {
			match self.cpu.run() {
				Ok(_) => {},

				Err(_) => {
					return;
				}
			}
			self.icount += 1;
			if (self.icount == ic) {
				println!("Maximum run cycle reached: {}.\nStop the RISC-V Core sim", ic);
				return;
			}
		}
	}

	pub fn start_dbg(&mut self, ic: i32) {
		loop {
			let d: u32 = self.cpu.memory.read(self.cpu.pc);
			println!("Address[{:08x}]: {:08x} ({:?})", self.cpu.pc, d, Instruction::decode(d).unwrap());
			println!("reg: {:?}", self.cpu.rf);
			match self.cpu.run() {
				Ok(_) => {},

				Err(_) => {
					println!("finish");
					break;
				}
			}
			self.icount += 1;
			if (self.icount == ic) {
				println!("Maximum run cycle reached: {}.\nStop the RISC-V Core sim", ic);
				return;
			}
		}
	}
}