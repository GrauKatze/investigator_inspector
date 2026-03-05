enum Config {
    Version,
    Help,
    Command(Option<Vec<String>>),
}

impl Config {
    fn build(init_options: &Option<Vec<String>>) -> Result<Config, String> {
        match init_options {
            None => Ok(Config::Help),
            Some(options) => match options.get(0) {
                Some(p) => match p.as_str() {
                    "-h" | "--help" | "h" | "help" => Ok(Config::Help),
                    "-v" | "--version" | "v" | "version" => Ok(Config::Version),
                    _ => Err(format!(
                        "not found this arguments\n{{ {:?} }}",
                        options.iter().collect::<Vec<&String>>()
                    )),
                },
                None => Err("Need args".to_string()),
            },
        }
    }
}

pub struct Inspector {
    configuration: Config,
}

impl Inspector {
    pub fn build(init_options: &Option<Vec<String>>) -> Result<Inspector, String> {
        let conf = Config::build(init_options);
        match conf {
            Ok(conf) => Ok(Inspector {
                configuration: conf,
            }),
            Err(msg) => Err(msg),
        }
    }
    pub fn run(&self) -> Result<(), String> {
        match &self.configuration {
            Config::Help => Self::write_help_msg(),
            Config::Version => Self::write_version_msg(),
            _ => Err("Error aanlis config".to_string()),
        }
    }
    pub fn write_help_msg() -> Result<(), String> {
        println!(env!("CARGO_PKG_NAME"));
        println!(env!("CARGO_PKG_VERSION"));
        Ok(())
    }
    pub fn write_version_msg() -> Result<(), String> {
        println!(env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
