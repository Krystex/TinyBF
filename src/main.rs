use std::io;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Virtual Machine
pub struct VM {
	/// The Program Content
	pub program: Vec<u8>,
	/// The VM Stack Memory
	pub stack: Vec<u8>,
	/// The stack pointer
	pub sp: usize,
	/// The program counter (current position)
	pub pc: usize,
}

impl VM {
	/// Creates a VM instance
	pub fn new() -> VM {
		VM {
			program: Vec::new(),
			stack: Vec::with_capacity(16),
			sp: 0,
			pc: 0,
		}
	}
	/// Load a file in the vm program space
	pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
		//let mut data = Vec::new();
		File::open(path)?.read_to_end(&mut self.program)?;
		Ok(())
	}
	/// Run the loaded program
	pub fn run(&mut self) {
		loop {
			//println!("Stack: {:?}", self.stack);
			//let now = self.program.get(self.pc).map(|x| *x);
			match self.get_instruction() {
				Some('>') => { self.inc_sp(); },
				Some('<') => { self.dec_sp(); },
				Some('+') => { self.inc_val(); },
				Some('-') => { self.dec_val(); },
				Some('.') => { self.print(); },
				Some('[') => { self.jump_fwd(); },
				Some(']') => { self.jump_bkw(); },
				Some( _ ) => (),
				None => break,
			}
			self.pc += 1;
		}
	}
	fn inc_val(&mut self) {
		let val = self.stack.get(self.sp).map(|x| *x);
		match val {
			Some(x) => { self.stack[self.sp] = x + 1; },
			None => { self.stack.push(1) },
		}
	}
	fn dec_val(&mut self) {
		let val = self.stack.get(self.sp).map(|x| *x);
		match val {
			Some(x) => { self.stack[self.sp] = x - 1 },
			None => { self.stack.push(1) },
		}
	}
	fn inc_sp(&mut self) {
		self.sp += 1;
	}
	fn dec_sp(&mut self) {
		self.sp -= 1;
	}
	fn print(&self) {
		print!("{}", self.stack[self.sp] as char);
	}
	fn jump_fwd(&mut self) {
		let mut counter = 0;
		// Jump if current value is 0
		if self.get_val() == Some(0 as char) {
			loop {
				match self.get_instruction() {
					Some(']') => {
						counter -= 1;
						if counter == 0 {
							return;
						}
					},
					Some('[') => {
						counter += 1;
					}
					_ => (),
				}
				self.pc += 1;
			}
		}
	}
	fn get_instruction(&self) -> Option<char> {
		self.program.get(self.pc).map(|x| *x as char)
	}
	fn get_val(&self) -> Option<char> {
		self.stack.get(self.sp).map(|x| *x as char)
	}
	fn jump_bkw(&mut self) {
		let mut counter = 0;
		// Jump back if current value isn't 0
		if self.get_val() != Some(0 as char) {
			loop {
				match self.get_instruction() {
					Some('[') => {
						counter -= 1;
						if counter == 0 {
							return;
						}
					},
					Some(']') => {
						counter += 1;
					}
					_ => (),
				}
				self.pc -= 1;
			}
		}
	}
}

fn main() -> Result<(), io::Error> {
	let filename = std::env::args()
		.nth(1)
		.ok_or(io::Error::new(io::ErrorKind::Other, "No file specified"))?;

	let mut vm = VM::new();
	vm.load_file(filename)?;
	vm.run();

	Ok(())
}
