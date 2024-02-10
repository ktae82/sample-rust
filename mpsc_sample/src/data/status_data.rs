pub struct StatusData {
    name: String,
    data: String,
}

impl StatusData {
    pub fn new() -> Self {
        Self {
            name: "status".to_string(),
            data: "data".to_string(),
        }
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    pub fn to_string(&self) -> String {
        "[Name: ".to_string() + &self.name + "], [Data: " + &self.data + "]"
    }
}
