pub struct Args {
    pub filename: String,
    pub count: u16,
}

impl Args {
    pub fn parse(args: Vec<String>) -> Args {
        let basic_args = Args {
            filename: args[1].clone(),
            count: 1,
        };
        return basic_args;
    }
}
