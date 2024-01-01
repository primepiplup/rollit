pub struct Args {
    pub filename: String,
    pub count: usize,
}

impl Args {
    pub fn parse(args: Vec<String>) -> Args {
        let mut arg_counter: usize = 1;

        let mut basic_args = Args {
            filename: args[args.len() - 1].clone(),
            count: 1,
        };

        while arg_counter < args.len() {
            if Self::is_argument(&args, arg_counter) {
                arg_counter += basic_args.set_argument(&args, arg_counter);
            } else {
                arg_counter += 1;
            }
        }

        return basic_args;
    }

    fn is_argument(args: &Vec<String>, counter: usize) -> bool {
        let arg = &args[counter];
        let first_char = arg.chars().next().unwrap();
        let second_char = arg.chars().next().unwrap();
        return first_char == '-' && second_char == '-';
    }

    fn set_argument(&mut self, cli_arguments: &Vec<String>, counter: usize) -> usize {
        let selector = &cli_arguments[counter][2..];

        match selector {
            "count" => return self.set_count(cli_arguments, counter + 1),
            "repeats" => return self.set_count(cli_arguments, counter + 1),
            _ => return 1,
        }
    }

    fn set_count(&mut self, cli_arguments: &Vec<String>, counter: usize) -> usize {
        if counter >= cli_arguments.len() {
            println!("No argument following the count flag...");
            return 2;
        }

        let value = &cli_arguments[counter];

        let count = match value.parse::<usize>() {
            Ok(count) => count,
            Err(_) => {
                println!("Incorrect count value provided, defaulting to 1");
                return 2;
            }
        };

        self.count = count;

        return 2;
    }
}
