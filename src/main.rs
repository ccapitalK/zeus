extern crate byteorder;
mod cpu;

use std::io::{self,Write};

fn help_string() {
    println!("Commands:");
    println!("    d[ump] <off>:   dump 32 words of ram starting at offset");
    println!("    h[elp]:         print this help message");
    println!("    l[oad]:         load file into offset 0x8000");
    println!("    p[rint]:        print current state of cpu");
    println!("    r[eset]:        reset cpu state (will load rom)");
    println!("    q[uit]:         quit this emulator");
    println!("    s[tep]/t[ick]:  step one instruction");
}

fn flush_stdout() {
    io::stdout().flush()
        .expect("Failed to flush buffer");
}

fn main() {
    let mut cpu = cpu::Cpu::new();
    'main: loop {

        print!("> ");
        flush_stdout();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line from stdin");

        let mut words = input.split_whitespace();

        if let Some(word) = words.next() {
            match word {
                "s"|"step"|"t"|"tick" => cpu.tick(),
                "p"|"print" => println!("{:?}", cpu),
                "q"|"quit" => break 'main,
                "d"|"dump" => {
                    let offset = match words.next() {
                        Some(v)  => v.parse(),
                        None => {
                            cpu.dump_ins();
                            continue 'main;
                        },
                    };
                    let offset = match offset {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Failed to parse offset: {}", e);
                            continue 'main;
                        }
                    };
                    cpu.dump(offset);
                },
                "r"|"reset" => {
                    println!("Reset Cpu.");
                    cpu.reset();
                    cpu.boot();
                },
                "l"|"load" => {
                    print!("File name: ");
                    flush_stdout();
                    let mut file_name = String::new();
                    io::stdin().read_line(&mut file_name)
                        .expect("Failed to read line from stdin");
                    file_name.pop();
                    if let Err(e) = cpu.load_rom(file_name.as_str()) {
                        println!("Failed to read \"{}\": {}", file_name, e);
                        continue 'main;
                    }
                },
                "h"|"help" => help_string(),
                _ => help_string(),
            }
        }

    }
}
