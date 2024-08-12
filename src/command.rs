use crate::config::Config;
use crate::error::{HumorError, HumorResult};
use std::process::Command;

pub struct CommandExecutor<'a> {
    config: &'a Config,
}

impl<'a> CommandExecutor<'a> {
    pub fn new(config: &'a Config) -> Self {
        CommandExecutor { config }
    }

    pub fn execute(&self, args: &[String]) -> HumorResult<()> {
        let command = self.config.find_command(args)?;
        println!("Executing: {}", command);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", command]).output()?
        } else {
            Command::new("sh").arg("-c").arg(command).output()?
        };

        if !output.status.success() {
            eprintln!("Command failed with exit code: {:?}", output.status.code());
            eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
            return Err(HumorError::CommandExecutionFailed);
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}
