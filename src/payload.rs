#[derive(Debug)] //This line helps print the struct
struct View_Change {
    type : u32,
    server_id : u32,
    attempted : u32,
}

#[derive(Debug)] //This line helps print the struct
struct VC_Proof {
    type : u32,
    server_id : u32,
    installed : u32,
}

impl View_Change {
    fn encode(&self) -> String {
        let mut temp : u32;
        let s = String::new();
        temp = htonl(self.type);
        s.push_str(temp.to_string());
        temp = htonl(self.server_id);
        s.push_str(temp.to_string());
        temp = htonl(self.attempted);
        s.push_str(temp.to_string());
        s
    }
}

impl VC_Proof {
    fn encode(&self) -> String {
        let mut temp : u32;
        let s = String::new();
        temp = htonl(self.type);
        s.push_str(temp.to_string());
        temp = htonl(self.server_id);
        s.push_str(temp.to_string());
        temp = htonl(self.installed);
        s.push_str(temp.to_string());
        s
    }
}
