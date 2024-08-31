pub struct SearchOptions {
    pub show_all: bool,
}

impl SearchOptions {
    pub fn from(args: &Vec<String>) -> Self {
        return SearchOptions {
            show_all: args.contains(&"-a".to_string()),
        };
    }
}
