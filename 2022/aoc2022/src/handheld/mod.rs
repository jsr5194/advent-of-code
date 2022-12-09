use std::collections::{HashMap, HashSet, VecDeque};

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
}

impl Default for Handheld {
    fn default() -> Self {
        Handheld {
            version: 0,
            pkt_magic_len: 4,
            msg_magic_len: 14,
            packet_data: VecDeque::default(),
            fs: HandheldFilesystem::default(),
        }
    }
}

impl Handheld {
    pub fn get_system_info(&self) -> String {
        format!("Handheld v{}", self.version)
    }

    pub fn get_fs(&self) -> &VecDeque<HandheldFile> {
        &self.fs.files
    }

    pub fn ingest_packet(&mut self, raw_packet_data: &String) {
        self.packet_data = raw_packet_data.chars().collect::<VecDeque<char>>()
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
