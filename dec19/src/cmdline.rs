pub struct Config {
    pub filename: String,
    pub num_registers: usize,
    pub start_val: i64
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must pass the filename and number of registers");
        }
        
        let filename = args[1].clone();
        let num_registers = args[2].parse().unwrap();
        if args.len() == 3 {
            Ok(Config { filename, num_registers, start_val: 0 })
        } else {
            let start_val: i64 = args[3].parse().unwrap();
            Ok(Config { filename, num_registers, start_val })
        }
    }
}