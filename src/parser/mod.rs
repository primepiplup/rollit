pub struct Args {
    pub filename: String,
    pub count: u16,
}

impl Args {
    pub fn parse(args: Vec<String>) -> Args {
        let mut arg_counter: usize = 1;

        let mut basic_args = Args {
            filename: args[args.len() - 1].clone(),
            count: 1,
        };

        if Self::is_argument(&args, arg_counter) {
            basic_args = Self::set_argument(basic_args, args.clone(), arg_counter);
        };

        arg_counter = arg_counter + 2;

        return basic_args;
    }

    fn is_argument(args: &Vec<String>, counter: usize) -> bool {
        let arg = &args[counter];
        let first_char = arg.chars().next().unwrap();
        let second_char = arg.chars().next().unwrap();
        return first_char == '-' && second_char == '-';
    }

    fn set_argument(mut settings: Args, cli_arguments: Vec<String>, counter: usize) -> Args {
        let mut selector = &cli_arguments[counter][2..];

        let value: u16 = cli_arguments[counter + 1].parse().unwrap(); // write proper handling for this returning feedback to the user

        if selector == "count" || selector == "repeats" {
            settings.count = value;
        }

        return settings;
    }
}
