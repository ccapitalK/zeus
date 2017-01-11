use std::fmt;
use std::io::prelude::*;
use std::fs::File;

const RAM_SIZE:   usize = 0x10000;
const PROG_START: usize = 0x8000;
const PROG_SIZE:  usize = 0x8000;
const NUM_REGS:   usize = 8;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;
const REG_X: usize = 3;
const REG_Y: usize = 4;
const REG_Z: usize = 5;
const REG_I: usize = 6;
const REG_J: usize = 7;

pub struct Cpu {
    reg: [u16; NUM_REGS],
    pc: u16,
    sp: u16,
    ex: u16,
    ia: u16,
    ram: [u16; RAM_SIZE],
    rom: [u16; PROG_SIZE],

    ins_ticks: u16,
    rom_file: String,
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "{{"));
        try!(writeln!(f, "    pc: {:X}", self.pc));
        try!(writeln!(f, "    sp: {:X}", self.sp));
        try!(writeln!(f, "    ex: {:X}", self.ex));
        try!(writeln!(f, "    ia: {:X}", self.ia));
        try!(writeln!(f, "    File Name: {:?}", self.rom_file));
        writeln!(f, "}}")
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: [0u16; NUM_REGS],
            pc: 0u16,
            sp: 0u16,
            ex: 0u16,
            ia: 0u16,
            ram: [0u16; RAM_SIZE],
            rom: [0u16; PROG_SIZE],

            ins_ticks: 0u16,
            rom_file: String::new(),
        }
    }
    pub fn reset(&mut self) {
        self.reg=[0u16; NUM_REGS];
        self.pc=0u16;
        self.sp=0u16;
        self.ex=0u16;
        self.ia=0u16;
        self.ins_ticks=0u16;
        self.ram=[0u16; RAM_SIZE];
        self.rom=[0u16; PROG_SIZE];
    }
    pub fn boot(&mut self) {
        self.pc = PROG_START as u16;
        self.ram[PROG_START..PROG_START+PROG_SIZE]
            .clone_from_slice(&self.rom);
    }
    pub fn tick(&mut self) {
        self.pc+=1;
        println!("pc: 0x{:X}", self.pc);
    }
    pub fn dump(&self, offset: u16) {
        println!("Dumping rom from address 0x{:04X}", offset);
        for i in 0u16..32u16 {
            if (i%8) == 0 {
                print!("\n{:04X}: ", offset+i);
            }
            print!("{:04X} ", self.ram[(offset+i) as usize]);
        }
        println!();
    }
    pub fn dump_ins(&self) {
        self.dump(self.pc);
    }
    pub fn load_rom(&mut self, file_name: String) -> Result<(), String> {
        self.rom_file = file_name;
        Err(format!("Not implemented"))
    }
}
