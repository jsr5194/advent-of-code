use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum CommandState {
    Cd,
    Ls,
    None,
}

impl Default for CommandState {
    fn default() -> Self {
        CommandState::None
    }
}

#[derive(Debug, Default)]
pub struct Tree {
    pub limbs: HashMap<String, TreeMember>,
}

//impl Default for Tree {
//    fn default() -> Self {
//        let mut default_tree = Tree {
//            limbs: HashMap::new(),
//        };
//        default_tree
//            .limbs
//            .insert(String::from("/"), TreeMember::default());
//        default_tree
//    }
//}

impl Tree {
    pub fn print_tree(&self) {
        for member in &self.limbs {
            println!("{:?}", member);
        }
    }

    pub fn calculate_member_size(&self, member_name: String) -> usize {
        println!("checking for member name: {:?}", member_name);
        if !self.limbs.contains_key(&member_name) {
            panic!("tree does not contain the provided member name");
        }
        let mut size = 0;
        //println!("Member Name: {:?}", member_name);
        let member = self.limbs.get(&member_name).unwrap();
        for dir in &member.dirs {
            if member_name == "/" {
                size += self
                    .limbs
                    .get(&format!("/{}", dir))
                    .unwrap()
                    .calculate_direct_size();
            } else {
                println!("getting {:?}", format!("{}/{}", member_name, dir));
                size += self
                    .limbs
                    .get(&format!("{}/{}", member_name, dir))
                    .unwrap()
                    .calculate_direct_size();
            }
        }
        size += member.calculate_direct_size();
        size
    }
}

#[derive(Debug, Default)]
pub struct TreeMember {
    pub dirs: HashSet<String>,
    pub files: HashMap<String, usize>,
}

impl TreeMember {
    pub fn calculate_direct_size(&self) -> usize {
        let mut size = 0;
        for (name, cur_size) in &self.files {
            size += cur_size;
        }
        size
    }
}

#[derive(Debug)]
pub struct Handheld {
    version: usize,
    pkt_magic_len: usize,
    msg_magic_len: usize,
    packet_data: VecDeque<char>,
    stdout: VecDeque<String>,
    tree: Tree,
}

impl Default for Handheld {
    fn default() -> Self {
        Handheld {
            version: 0,
            pkt_magic_len: 4,
            msg_magic_len: 14,
            packet_data: VecDeque::default(),
            stdout: VecDeque::default(),
            tree: Tree::default(),
        }
    }
}

impl Handheld {
    pub fn get_system_info(&self) -> String {
        format!("Handheld v{}", self.version)
    }

    pub fn ingest_packet(&mut self, raw_packet_data: &String) {
        self.packet_data = raw_packet_data.chars().collect::<VecDeque<char>>()
    }

    pub fn ingest_stdout(&mut self, raw_stdout: &String) {
        self.stdout = raw_stdout
            .lines()
            .map(|x| x.to_string())
            .collect::<VecDeque<String>>()
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

    pub fn get_tree(&self) -> &Tree {
        &self.tree
    }

    pub fn parse_stdout(&mut self) {
        let mut state = CommandState::default();
        let mut idx = 0;
        let mut working_dir = String::from("/");
        while idx < self.stdout.len() {
            // done with the previous command so reset the parser state
            if self.stdout[idx].chars().next().unwrap() == '$' {
                let mut cmd_line = self.stdout[idx].split(" ");

                // trash the initial '$'
                cmd_line.next();

                // extract the command
                let cmd = cmd_line.next().unwrap();

                // extract the command code and parse as needed
                match cmd {
                    "cd" => {
                        // extract the argument
                        let new_dir = cmd_line.next().unwrap();
                        // update the current working dir
                        let mut new_working_dir = String::default();
                        if new_dir == ".." {
                            for dir_idx in
                                0..working_dir.split("/").collect::<Vec<&str>>().len() - 1
                            {
                                new_working_dir =
                                    format!("/{}", working_dir.split("/").nth(dir_idx).unwrap());
                            }
                        } else if new_dir.chars().next().unwrap() == '/' {
                            new_working_dir = String::from("/");
                        } else {
                            if working_dir.chars().last().unwrap() == '/' {
                                new_working_dir = format!("{}{}", working_dir, new_dir);
                            } else {
                                new_working_dir = format!("{}/{}", working_dir, new_dir);
                            }
                        }

                        // add the new directory to the working dir if it doesn't exist
                        if new_dir != ".." && new_dir != "/" {
                            if self.tree.limbs.contains_key(&working_dir) {
                                let mut cur_tree_member =
                                    self.tree.limbs.get_mut(&working_dir).unwrap();
                                if !cur_tree_member.dirs.contains(&new_dir.to_string()) {
                                    cur_tree_member.dirs.insert(new_dir.to_string());
                                }
                            } else {
                                self.tree
                                    .limbs
                                    .insert(working_dir.clone(), TreeMember::default());
                                self.tree
                                    .limbs
                                    .get_mut(&working_dir)
                                    .unwrap()
                                    .dirs
                                    .insert(new_dir.to_string());
                            }
                        }

                        if new_working_dir == "" {
                            working_dir = String::from("/");
                        } else {
                            working_dir = new_working_dir;
                        }

                        // add the new directory to the working dir if it doesn't exist
                        if &working_dir != ".." {
                            if !self.tree.limbs.contains_key(&working_dir) {
                                self.tree
                                    .limbs
                                    .insert(working_dir.clone(), TreeMember::default());
                            }
                        }
                        idx += 1;
                    }
                    "ls" => {
                        idx += 1;
                        while idx < self.stdout.len()
                            && self.stdout[idx].chars().next().unwrap() != '$'
                        {
                            let comparison = self.stdout[idx].split(" ").next().unwrap();
                            match comparison {
                                "dir" => {
                                    let new_dir = self.stdout[idx].split(" ").nth(1).unwrap();

                                    // add the new directory to the working dir if it doesn't exist
                                    if self.tree.limbs.contains_key(&working_dir) {
                                        let mut cur_tree_member =
                                            self.tree.limbs.get_mut(&working_dir).unwrap();
                                        if !cur_tree_member.dirs.contains(&new_dir.to_string()) {
                                            cur_tree_member.dirs.insert(new_dir.to_string());
                                        }
                                    } else {
                                        self.tree
                                            .limbs
                                            .insert(working_dir.clone(), TreeMember::default());
                                        self.tree
                                            .limbs
                                            .get_mut(&working_dir)
                                            .unwrap()
                                            .dirs
                                            .insert(new_dir.to_string());
                                    }
                                }
                                _ => {
                                    let mut file_details = self.stdout[idx].split(" ");
                                    let size = file_details.next().unwrap();
                                    let filename = file_details.next().unwrap();
                                    //println!("limbs: {:?}", self.tree.limbs);
                                    self.tree.limbs.get_mut(&working_dir).unwrap().files.insert(
                                        filename.clone().to_string(),
                                        size.clone().parse::<usize>().unwrap(),
                                    );
                                }
                            }
                            idx += 1;
                        }
                    }
                    _ => unreachable!("Invalid command detected"),
                }
            }
        }
    }
}
