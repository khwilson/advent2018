pub struct Config {
    pub filename: String,
    pub is_first_puzzle: bool,
    pub num_workers: usize,
    pub base_amount: i64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("You must pass the filename");
        }
        
        let filename = args[1].clone();

        if args.len() == 2 {
            return Ok(Config { filename: filename, is_first_puzzle: true, num_workers: 0, base_amount: 0 });
        } else {
            return Ok(Config { filename: filename, is_first_puzzle: false, num_workers: args[2].parse().unwrap(), base_amount: 
            args[3].parse().unwrap() })
        }
    }
}