use super::hpu::*;
use std::boxed::Box;
use std::fs::File;
use super::error::*;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::{hack_report_less};


pub fn create_assembler(path: &std::path::PathBuf) -> Assembler {
    Assembler {
        path: path.clone(),
        hpu: HPU::new(path),
    }
}

pub struct Assembler {
    path: std::path::PathBuf,
    hpu: HPU,
}

impl Assembler {
    pub fn run(&mut self) -> Result<(), Box<HackError>> {
        self.first_pass()?;
        self.second_pass()?;
        Ok(())
    }
    fn polish(s: &str) -> String {
        s.trim().to_owned()
    }
    fn first_pass(&mut self) -> Result<(), Box<HackError>> {
        println!("================= First Pass Begins =================");
        let f = File::open(&self.path).expect("Could not read file");
        let reader = BufReader::new(f);
        for (num, line) in reader.lines().enumerate() {
            self.hpu
                .first_pass(&(num, Assembler::polish(&line.unwrap())))?;
        }
        println!("================= First Pass Ends =================");
        Ok(())
    }

    fn second_pass(&mut self) -> Result<(), Box<HackError>> {
        println!("================= Second Pass Begins =================");
        let f = File::open(&self.path).expect("Could not read file");
        &self.path.set_extension("hack");
        let w = File::create(&self.path).expect("Could not read file");
        let mut writer = BufWriter::new(w);
        let reader = BufReader::new(f);
        for (num, line) in reader.lines().enumerate() {
            let out = self
                .hpu
                .second_pass(num, &Assembler::polish(&line.unwrap()))?;
            println!("[out]: {}", out);
            if out.len() > 0 {
                if let Err(_) = writeln!(&mut writer, "{}", out) {
                        hack_report_less!("Error occured in writeln!")
                }
            }
        }
        println!("================= Second Pass Ends =================");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
