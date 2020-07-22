use cursive::theme::{BaseColor, Color};
use directories::ProjectDirs;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use serde_derive::Deserialize;
use toml;

pub struct AppConfig {
    pub data_path: PathBuf,

    pub true_chr: char,
    pub false_chr: char,
    pub future_chr: char,

    // view dimensions
    pub view_width: usize,
    pub view_height: usize,

    // app dimensions
    pub grid_width: usize,

    pub reached_color: Color,
    pub todo_color: Color,
    pub future_color: Color,
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    pub data_path: String,
}

impl AppConfig {
    fn new() -> AppConfig {
        let project_dirs = ProjectDirs::from("rs", "nerdypepper", "dijo")
            .unwrap_or_else(|| panic!("Invalid home directory!"));

        let mut config_path = PathBuf::from(project_dirs.config_dir());
        fs::create_dir_all(&config_path);
        config_path.push("config.toml");

        let read_config = |file: PathBuf| -> io::Result<TomlConfig> {
            let mut f = File::open(file)?;
            let mut buffer = String::new();
            f.read_to_string(&mut buffer)?;
            return Ok(toml::from_str(&buffer).unwrap());
        };

        let data_path = match read_config(config_path) {
            Ok(x) => PathBuf::from(x.data_path),
            Err(_) => {
                let dir = project_dirs.data_dir();
                fs::create_dir_all(dir);
                PathBuf::from(dir)
            }
        };

        AppConfig {
            data_path: data_path,
            true_chr: '·',
            false_chr: '·',
            future_chr: '·',
            view_width: 25,
            view_height: 8,
            grid_width: 3,
            reached_color: Color::Dark(BaseColor::Cyan),
            todo_color: Color::Dark(BaseColor::Magenta),
            future_color: Color::Light(BaseColor::Black),
        }
    }
}

pub fn load_configuration_file() -> AppConfig {
    AppConfig::new()
}

pub fn habit_file() -> PathBuf {
    let config = load_configuration_file();
    let mut data_file = config.data_path;
    data_file.push("habit_record.json");
    data_file
}

pub fn auto_habit_file() -> PathBuf {
    let config = load_configuration_file();
    let mut data_file = config.data_path;
    data_file.push("habit_record[auto].json");
    data_file
}
