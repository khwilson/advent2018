pub struct Config {
    pub filename: String,
    pub is_first_puzzle: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("You must pass the filename");
        }
        /*
        let base = match args[1].parse() {
            Ok(num) => num,
            Err(_) => {
                return Err("Your first argument must be an integer");;
            },
        };*/
        let filename = args[1].clone();

        let is_first_puzzle = args.len() == 2;
        Ok(Config { filename, is_first_puzzle })
    }
}