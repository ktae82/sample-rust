pub struct StatusDB {
    name: String,
    data: String,
}

impl StatusDB {
    pub fn new() -> Self {
        Self {
            name: "StatusDB".to_string(),
            data: "data".to_string(),
        }
    }

    pub fn get_data(&self) -> String {
        "[Name: ".to_string() + &self.name + "], [Data: " + &self.data + "]"
    }

    pub fn set_data(&mut self, data: u32) {
        self.data = data.to_string();
    }
}
