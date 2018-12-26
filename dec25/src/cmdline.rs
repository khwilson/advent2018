pub struct Config {
    pub filename: String,
    pub max_dist: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must pass the filename and max dist");
        }
        
        let filename = args[1].clone();

        let max_dist = args[2].parse().unwrap();
        Ok(Config { filename, max_dist })
    }
}