use std::error::Error;

pub struct Config {
    command: Command
}

impl Config {
    pub fn build(args: &[String]) -> Config {
        Config{command: Command::None}
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let args = ["desc".to_string(), "test/test1.txt".to_string()];

        if let Command::Desc(desc) = Config::build(&args).command {
            let wanted = "
[classes]
horse
";
            assert_eq!(
                desc.run().unwrap_or_else(|err| {
                    eprintln!("Error desc.run(): {err}");
                    String::from("")
                }),
                wanted,
            )
        } else {
            panic!("The result of config build isn't Desc");
        }
    }
}

enum Command {
    Desc(Desc),
    None
}

struct Desc {
    file_path: String,
}

impl Desc {
    fn run(&self) -> Result<String, String> {
        Ok(String::from(""))
    }
}