use std::fs;
use std::io::Write;
use std::error::Error;

pub struct Config {
    pub source: String,
    pub destination: String,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let source = args[1].clone();
        let destination = args[2].clone();
        Ok(Config { source, destination })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let source = &config.source;
    let destination = &config.destination;
    let contents = read_contents(&source)?;
    copy_contents(&contents, &destination)?;
    let validated = validate(&contents, &destination)?;
    if !validated {
        println!("File copied but validation failed");
    }
    Ok(())
}

pub fn read_contents(source: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(source)?;
    Ok(contents)
}

pub fn copy_contents(contents: &String, destination: &String) -> Result<(), Box<dyn Error>> {
    let mut dest_file = fs::File::create(destination)?;
    dest_file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn validate(contents: &String, destination: &String) -> Result<bool, Box<dyn Error>>{
    let dest_contents = read_contents(&destination)?;
    Ok(contents.to_string() == dest_contents.to_string())
}
