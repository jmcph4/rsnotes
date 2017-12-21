pub struct Note {
    id: u64,
    name: String,
    data: String,
    tags: Vec<String>
}

#[allow(dead_code)]
impl Note {
    pub fn new() -> Note {
        Note {
            id: 0,
            name: "".to_string(),
            data: "".to_string(),
            tags: vec![]
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn set_id(&mut self, id: u64) -> () {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) -> () {
        self.name = name
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn set_data(&mut self, data: String) -> () {
        self.data = data
    }

    pub fn add_tag(&mut self, tag: String) -> () {
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: String) -> () {
        // self.tags.remove_item(&tag); // "unstable" >:-(
        
        // traverse our vector manually
        for i in 0..self.tags.len() {
            if self.tags[i] == tag {
                self.tags.remove(i);
                break;
            }
        }
    }

    pub fn has_tag(&self, tag: String) -> bool {
        return self.tags.contains(&tag);
    }

    pub fn to_string(&self) -> String {
        let s: String = self.name().to_owned() + "\n---\n" + self.data() + "\n";
        return s;
    }

    pub fn print(&self) -> () {
        println!("{}", self.to_string());
    }
}

