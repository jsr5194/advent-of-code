use std::collections::VecDeque;

#[derive(Debug)]
pub struct Handheld {
    version: usize,
    pkt_magic_len: usize,
    msg_magic_len: usize,
    packet_data: VecDeque<char>,
}

impl Default for Handheld {
    fn default() -> Self {
        Handheld {
            version: 0,
            pkt_magic_len: 4,
            msg_magic_len: 14,
            packet_data: VecDeque::default(),
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
