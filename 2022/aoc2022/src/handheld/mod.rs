use log::info;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

#[derive(Debug, Clone)]
pub struct HandheldCrt {
    display: VecDeque<VecDeque<char>>,
    width: usize,
    height: usize,
}

impl HandheldCrt {
    fn light_pixel(&mut self, cycle: isize) {
        //println!("cycle {:?}", cycle);
        let row = usize::try_from(cycle / isize::try_from(self.width).unwrap()).unwrap();
        let col = usize::try_from(cycle % isize::try_from(self.width).unwrap()).unwrap();
        //println!("lighting: ({},{})", row, col);
        self.display[row][col] = '#'
    }
}

impl fmt::Display for HandheldCrt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg =
            format!("\n         Handheld CRT Display\n========================================\n");
        for row in &self.display {
            for pixel in row {
                msg = format!("{}{}", msg, pixel);
            }
            msg = format!("{}\n", msg);
        }
        write!(f, "{}", msg)
    }
}

impl Default for HandheldCrt {
    fn default() -> Self {
        let height = 6;
        let width = 40;
        let mut crt: VecDeque<VecDeque<char>> = VecDeque::new();
        for i in 0..height {
            let mut row: VecDeque<char> = VecDeque::new();
            for j in 0..width {
                row.push_back('.');
            }
            crt.push_back(row.clone());
        }
        HandheldCrt {
            display: crt.clone(),
            height: height,
            width: width,
        }
    }
}

#[derive(Debug, Clone)]
enum HandheldInstruction {
    Noop,
    Addx { value: isize },
}

#[derive(Debug, Clone)]
pub struct HandheldCpu {
    pause: bool,
    ip: usize,
    cycle: isize,
    executing: bool,
    program: VecDeque<HandheldInstruction>,
    registers: HashMap<String, isize>,
}

impl Default for HandheldCpu {
    fn default() -> Self {
        let mut default_registers = HashMap::new();
        default_registers.insert("X".to_string(), 1);
        HandheldCpu {
            pause: false,
            ip: 0,
            cycle: 1,
            executing: false,
            program: VecDeque::new(),
            registers: default_registers.clone(),
        }
    }
}

impl HandheldCpu {
    fn load_program(&mut self, raw_program: &String) {
        for raw_instr in raw_program.lines() {
            let mut split_instr = raw_instr.split(" ");
            match split_instr.next().unwrap() {
                "noop" => self.program.push_back(HandheldInstruction::Noop),
                "addx" => self.program.push_back(HandheldInstruction::Addx {
                    value: split_instr.next().unwrap().parse::<isize>().unwrap(),
                }),
                _ => unreachable!("Invalid instruction detected during parsing"),
            }
        }
    }
    fn reset(&mut self) {
        let mut default_registers = HashMap::new();
        default_registers.insert("X".to_string(), 1);
        self.pause = false;
        self.ip = 0;
        self.cycle = 1;
        self.executing = false;
        self.program = VecDeque::new();
        self.registers = default_registers.clone();
    }
    fn run(&mut self) {
        self.debug(-1);
    }
    fn debug(&mut self, breakpoint: isize) {
        while !self.pause {
            if self.ip >= self.program.len() {
                // program is finished
                self.pause = true;
            } else if self.cycle == breakpoint {
                // hit our inspection point
                self.pause = true;
            } else {
                match self.program[self.ip] {
                    HandheldInstruction::Noop => {
                        self.ip += 1;
                    }
                    HandheldInstruction::Addx { value } => {
                        if let Some(regX) = self.registers.get_mut(&"X".to_string()) {
                            if self.executing {
                                *regX += value;
                                self.executing = false;
                                self.ip += 1;
                            } else {
                                self.executing = true;
                            }
                        } else {
                            unreachable!("could not get the X register");
                        }
                    }
                }
                self.cycle += 1;
            }
        }
        self.pause = false;
    }
}

#[derive(Debug, Clone)]
pub struct HandheldFile {
    name: String,
    dir: String,
    size: usize,
}

impl HandheldFile {
    pub fn get_dir(&self) -> &String {
        &self.dir
    }

    pub fn get_filesize(&self) -> &usize {
        &self.size
    }
}

#[derive(Default, Debug, Clone)]
pub struct HandheldFilesystem {
    files: VecDeque<HandheldFile>,
}

impl HandheldFilesystem {
    fn add_file(&mut self, raw_working_dir: String, filename: String, filesize: usize) {
        self.files.push_back(HandheldFile {
            name: filename.clone(),
            dir: raw_working_dir.clone(),
            size: filesize.clone(),
        });
    }
}

#[derive(Debug, Clone)]
pub struct Handheld {
    version: usize,
    pkt_magic_len: usize,
    msg_magic_len: usize,
    packet_data: VecDeque<char>,
    fs: HandheldFilesystem,
    cpu: HandheldCpu,
    crt: HandheldCrt,
}

impl Default for Handheld {
    fn default() -> Self {
        Handheld {
            version: 0,
            pkt_magic_len: 4,
            msg_magic_len: 14,
            packet_data: VecDeque::default(),
            fs: HandheldFilesystem::default(),
            cpu: HandheldCpu::default(),
            crt: HandheldCrt::default(),
        }
    }
}

impl Handheld {
    pub fn get_system_info(&self) -> String {
        format!("Handheld v{}", self.version)
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn get_fs(&self) -> &VecDeque<HandheldFile> {
        &self.fs.files
    }

    pub fn print_display(&self) {
        info!("{}", self.crt)
    }

    pub fn ingest_packet(&mut self, raw_packet_data: &String) {
        self.packet_data = raw_packet_data.chars().collect::<VecDeque<char>>()
    }

    pub fn run_program(&mut self) {
        for cycle in 0..240 {
            self.cpu.debug(cycle + 1);
            let sprite_idx = self.cpu.registers.get(&"X".to_string()).unwrap();
            //println!("sprite_idx: {}, cycle: {}", sprite_idx, cycle % 40);
            let c2 = cycle % 40;
            if sprite_idx - 1 == c2 || *sprite_idx == c2 || sprite_idx + 1 == c2 {
                self.crt.light_pixel(cycle);
            }
        }
    }

    pub fn debug_program(&mut self, breakpoint: isize) {
        self.cpu.debug(breakpoint);
    }

    pub fn load_program(&mut self, program: &String) {
        self.cpu.load_program(&program);
    }

    pub fn get_signal_strength(&self) -> isize {
        let sum = self.cpu.cycle * self.cpu.registers.get(&"X".to_string()).unwrap();
        sum
    }

    pub fn build_fs_from_stdout(&mut self, raw_stdout: &String) {
        let mut working_dir = String::from("/");
        let raw_stdout_lines = raw_stdout
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut idx = 0;
        while idx < raw_stdout_lines.len() {
            let mut split_line = raw_stdout_lines[idx].split(" ");
            if split_line.next().unwrap() == "$" {
                match split_line.next().unwrap() {
                    "cd" => {
                        let dir = split_line.next().unwrap();
                        if dir == "/" {
                            working_dir = String::from("/");
                        } else if dir == ".." {
                            let mut new_working_dir = String::default();
                            for dir_idx in
                                0..working_dir.split("/").collect::<Vec<&str>>().len() - 1
                            {
                                if new_working_dir == "/".to_string() {
                                    new_working_dir = format!(
                                        "/{}",
                                        working_dir.split("/").nth(dir_idx).unwrap()
                                    );
                                } else {
                                    new_working_dir = format!(
                                        "{}/{}",
                                        new_working_dir,
                                        working_dir.split("/").nth(dir_idx).unwrap()
                                    );
                                }
                            }
                            working_dir = new_working_dir;
                        } else {
                            if working_dir == String::from("/") {
                                working_dir = format!("/{}", dir);
                            } else {
                                working_dir = format!("{}/{}", working_dir, dir);
                            }
                        }
                        idx += 1;
                    }
                    "ls" => {
                        idx += 1;
                        while idx < raw_stdout_lines.len() {
                            if raw_stdout_lines[idx].chars().next().unwrap() == '$' {
                                break;
                            } else {
                                let split_line = raw_stdout_lines[idx]
                                    .split(" ")
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>();
                                if split_line[0] == String::from("dir") {
                                    //self.fs.add_dir(working_dir.clone(), split_line[1].clone());\
                                } else {
                                    self.fs.add_file(
                                        working_dir.clone(),
                                        split_line[1].clone(),
                                        split_line[0].parse::<usize>().unwrap(),
                                    );
                                }
                                idx += 1;
                            }
                        }
                    }
                    _ => {
                        unreachable!("invalid command detected");
                    }
                }
            }
        }
    }

    pub fn calculate_fs_size(&self) -> HashMap<String, usize> {
        let mut dir_sizes: HashMap<String, usize> = HashMap::new();
        for file in self.get_fs() {
            let dir = file.get_dir();
            if dir == &"/".to_string() {
                let mut cur_dir = dir.to_string();
                if dir_sizes.contains_key(&cur_dir) {
                    if let Some(size) = dir_sizes.get_mut(&cur_dir.to_string()) {
                        *size += file.get_filesize();
                    } else {
                        unreachable!("could not get mutable dir size 1");
                    }
                } else {
                    dir_sizes.insert(cur_dir.to_string(), *file.get_filesize());
                }
            } else {
                let mut cur_dir = String::default();
                for raw_dir in file.get_dir().split("/") {
                    cur_dir = format!("{}/{}", cur_dir, raw_dir.to_string());
                    if cur_dir == "".to_string() {
                        cur_dir = "/".to_string()
                    }
                    if dir_sizes.contains_key(&cur_dir) {
                        if let Some(size) = dir_sizes.get_mut(&cur_dir.to_string()) {
                            *size += file.get_filesize();
                        } else {
                            unreachable!("could not get mutable dir size 2");
                        }
                    } else {
                        dir_sizes.insert(cur_dir.to_string(), *file.get_filesize());
                    }
                }
            }
        }

        dir_sizes
    }

    fn find_unique_sequence_idx(&self, size: usize) -> usize {
        let mut packet_window = VecDeque::new();
        let mut idx = 0;
        for c in &self.packet_data {
            if packet_window.len() < size {
                packet_window.push_back(c);
            } else {
                let mut magic_found = true;
                for i in 0..packet_window.len() - 1 {
                    for j in i + 1..packet_window.len() {
                        if packet_window[i] == packet_window[j] {
                            magic_found = false;
                            break;
                        }
                    }
                    if !magic_found {
                        break;
                    }
                }
                if magic_found {
                    break;
                } else {
                    packet_window.pop_front();
                    packet_window.push_back(c);
                }
            }
            idx += 1;
        }

        idx
    }

    pub fn find_start_of_pkt(&self) -> usize {
        self.find_unique_sequence_idx(self.pkt_magic_len)
    }

    pub fn find_start_of_msg(&self) -> usize {
        self.find_unique_sequence_idx(self.msg_magic_len)
    }
}
