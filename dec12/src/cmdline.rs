pub struct Config {
    pub filename: String,
    pub generations: u64,
    pub is_first_puzzle: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must pass the filename");
        }
        
        let filename = args[1].clone();
        let generations: u64 = args[2].parse().unwrap();

        let is_first_puzzle = args.len() == 3;
        Ok(Config { filename, generations, is_first_puzzle })
    }
}