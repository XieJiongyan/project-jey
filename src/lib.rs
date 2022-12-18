use std::fs;

use memory::Memory;

mod memory;
pub struct Config {
    command: Command
}

impl Config {
    pub fn build(args: &[String]) -> Config {
        if args.len() <= 1 {
            return Config{command: Command::None};
        }
        return match &*(args[1].to_lowercase()) {
            "desc" => Config { 
                command: Command::Desc(Desc{
                    file_path: if args.len() >= 3 {
                        args[2].clone()
                    } else {
                        String::from("")
                    }
                })
            },
            _ => Config{command: Command::None},
        }
    }
}

pub fn run(config: Config) -> Result<String, String> {
    match config.command {
        Command::Desc(desc) => {
            desc.run()
        }
        _ => Ok(String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let args = [
            "".to_string(),
            "desc".to_string(), 
            "test/test1.txt".to_string(),
            ];

        if let Command::Desc(desc) = Config::build(&args).command {
            let wanted = "
[classes]
horse
".trim();
            assert_eq!(
                desc.run().unwrap_or_else(|err| {
                    eprintln!("Error desc.run(): {err}");
                    String::from("")
                }).trim(),
                wanted,
            )
        } else {
            println!("Command: {:?}", Config::build(&args).command);
            panic!("The result of config build isn't Desc");
        }
    }
}

#[derive(Debug)]
enum Command {
    Desc(Desc),
    None
}
#[derive(Debug)]
struct Desc {
    file_path: String,
}

impl Desc {
    fn run(&self) -> Result<String, String> {
        let content = fs::
            read_to_string(String::from("src/jey_files/") 
            + &self.file_path)
            .expect("Cannot read file");
        
        
        let mut memory = Memory::new();
        memory.read(content);
        Ok(memory.get_desc())
    }
}