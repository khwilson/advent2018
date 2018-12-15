pub struct Config {
    pub filename: String,
    pub elf_power: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("You must pass the filename");
        }
        
        let filename = args[1].clone();

        let elf_power: u64;
        if args.len() >= 3 {
            elf_power = args[2].parse().unwrap();
        } else {
            elf_power = 3;
        }
        Ok(Config { filename, elf_power })
    }
}