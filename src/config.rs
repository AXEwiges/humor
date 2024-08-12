use crate::error::{HumorError, HumorResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    import: Vec<String>,
    #[serde(default)]
    commands: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

impl Config {
    pub fn load(path: &Path) -> HumorResult<Self> {
        let content =
            fs::read_to_string(path).map_err(|_| HumorError::FileNotFound(path.to_path_buf()))?;
        let mut config: Config = serde_yaml::from_str(&content)?;
        config.process_imports(path)?;
        Ok(config)
    }

    fn process_imports(&mut self, base_path: &Path) -> HumorResult<()> {
        let imports = std::mem::take(&mut self.import);
        for import_path in imports {
            let full_path = base_path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(import_path);
            let imported_config = Config::load(&full_path)?;
            self.merge(imported_config)?;
        }
        Ok(())
    }

    fn merge(&mut self, other: Config) -> HumorResult<()> {
        for (domain, commands) in other.commands {
            self.commands.entry(domain.clone()).or_default();
            for (category, subcmds) in commands {
                let domain_entry = self.commands.get_mut(&domain).unwrap();
                domain_entry.entry(category.clone()).or_default();
                for (name, cmd) in subcmds {
                    if let Some(_existing_cmd) = domain_entry.get_mut(&category).unwrap().get(&name)
                    {
                        return Err(HumorError::DuplicateCommand {
                            domain: domain.clone(),
                            command: name,
                        });
                    }
                    domain_entry.get_mut(&category).unwrap().insert(name, cmd);
                }
            }
        }
        Ok(())
    }

    pub fn find_command(&self, args: &[String]) -> HumorResult<&str> {
        match args.len() {
            1 => self.find_unique_command(&args[0]),
            2 => self.find_domain_command(&args[0], &args[1]),
            3 => self.find_full_command(&args[0], &args[1], &args[2]),
            _ => Err(HumorError::InvalidCommandStructure),
        }
    }

    fn find_unique_command(&self, cmd: &str) -> HumorResult<&str> {
        let mut found_cmd = None;
        for domain in self.commands.values() {
            for category in domain.values() {
                if let Some(command) = category.get(cmd) {
                    if found_cmd.is_some() {
                        return Err(HumorError::CommandNotFound(cmd.to_string()));
                    }
                    found_cmd = Some(command.as_str());
                }
            }
        }
        found_cmd.ok_or_else(|| HumorError::CommandNotFound(cmd.to_string()))
    }

    fn find_domain_command(&self, domain: &str, cmd: &str) -> HumorResult<&str> {
        if let Some(domain_cmds) = self.commands.get(domain) {
            for category in domain_cmds.values() {
                if let Some(command) = category.get(cmd) {
                    return Ok(command);
                }
            }
        }
        Err(HumorError::CommandNotFound(format!("{}.{}", domain, cmd)))
    }

    fn find_full_command(&self, domain: &str, category: &str, cmd: &str) -> HumorResult<&str> {
        self.commands
            .get(domain)
            .and_then(|d| d.get(category))
            .and_then(|c| c.get(cmd))
            .map(String::as_str)
            .ok_or_else(|| HumorError::CommandNotFound(format!("{}.{}.{}", domain, category, cmd)))
    }

    pub fn merge_configs(mut base: Config, other: Config) -> HumorResult<Config> {
        base.merge(other)?;
        Ok(base)
    }
}

pub fn load_default_config() -> HumorResult<Config> {
    let home_dir = dirs::home_dir().ok_or_else(|| HumorError::FileNotFound(PathBuf::from("~")))?;
    let default_config_path = home_dir.join(".humors").join("humor-base.yaml");
    if default_config_path.exists() {
        Config::load(&default_config_path)
    } else {
        Ok(Config {
            import: vec![],
            commands: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_config(content: &str) -> (PathBuf, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.yaml");
        fs::write(&file_path, content).unwrap();
        (file_path, dir)
    }

    #[test]
    fn test_load_config() {
        let content = r#"
        commands:
          rust:
            build:
              debug: cargo build
              release: cargo build --release
        "#;
        let (file_path, _dir) = create_test_config(content);

        let config = Config::load(&file_path).unwrap();
        assert_eq!(
            config
                .commands
                .get("rust")
                .unwrap()
                .get("build")
                .unwrap()
                .get("debug")
                .unwrap(),
            "cargo build"
        );
        assert_eq!(
            config
                .commands
                .get("rust")
                .unwrap()
                .get("build")
                .unwrap()
                .get("release")
                .unwrap(),
            "cargo build --release"
        );
    }

    #[test]
    fn test_merge_configs() {
        let base_content = r#"
        commands:
          rust:
            build:
              debug: cargo build
        "#;
        let (base_path, _dir1) = create_test_config(base_content);

        let other_content = r#"
        commands:
          rust:
            test:
              unit: cargo test
          python:
            run:
              script: python script.py
        "#;
        let (other_path, _dir2) = create_test_config(other_content);

        let base_config = Config::load(&base_path).unwrap();
        let other_config = Config::load(&other_path).unwrap();

        let merged_config = Config::merge_configs(base_config, other_config).unwrap();

        assert_eq!(
            merged_config
                .commands
                .get("rust")
                .unwrap()
                .get("build")
                .unwrap()
                .get("debug")
                .unwrap(),
            "cargo build"
        );
        assert_eq!(
            merged_config
                .commands
                .get("rust")
                .unwrap()
                .get("test")
                .unwrap()
                .get("unit")
                .unwrap(),
            "cargo test"
        );
        assert_eq!(
            merged_config
                .commands
                .get("python")
                .unwrap()
                .get("run")
                .unwrap()
                .get("script")
                .unwrap(),
            "python script.py"
        );
    }

    #[test]
    fn test_find_command() {
        let content = r#"
        commands:
          rust:
            build:
              debug: cargo build
              release: cargo build --release
          python:
            run:
              script: python script.py
        "#;
        let (file_path, _dir) = create_test_config(content);

        let config = Config::load(&file_path).unwrap();

        assert_eq!(
            config
                .find_command(&["rust".to_string(), "build".to_string(), "debug".to_string()])
                .unwrap(),
            "cargo build"
        );
        assert_eq!(
            config
                .find_command(&[
                    "python".to_string(),
                    "run".to_string(),
                    "script".to_string()
                ])
                .unwrap(),
            "python script.py"
        );
        assert!(config
            .find_command(&["java".to_string(), "compile".to_string()])
            .is_err());
    }
}
