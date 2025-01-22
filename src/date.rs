#[derive(Clone)]
pub struct Date {
    pub date: u32,
    pub month: u32,
    pub year: u32,
}

impl ToString for Date {
    fn to_string(&self) -> String {
        String::from(format!(
            "{:04}/{:02}/{:02}",
            self.year, self.month, self.date
        ))
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        let components: Vec<u32> = value.split("/").map(|s| s.parse().unwrap()).collect();

        assert!(components.len() == 3);

        Self {
            year: components[0],
            month: components[1],
            date: components[2],
        }
    }
}
