use crate::errors::Result;

use std::path::PathBuf;
use std::collections::HashMap;

use walkdir::WalkDir;

#[derive(Debug)]
pub struct Settings {
    /**
     * Settings regarding the input.
     */
    pub input: InputSettings,

    /**
     * Settings regarding the output.
     */
    pub output: OutputSettings,
}

#[derive(Debug)]
pub struct InputSettings {
    /**
     * Maximum file size allowed. (default: 33554432)
     * Files larger than this size will be ignored.
     */
    pub max_file_size: u64,

    /**
     * Whether or not to recurse into subfolders. (default: false)
     */
    pub recursive: bool,

    /**
     * Whether or not to skip hidden files (default: false)
     */
    pub skip_hidden: bool,

    /**
     * Whether or not to follow symbolic links (default: true)
     */
    pub follow_symlinks: bool,

    /**
     * Comma-separated list of file extens to look for. (default: "srt,sub")
     */
    pub extensions: String,
}

#[derive(Debug)]
pub struct OutputSettings {
    /**
     * Whether or not to keep backups of the original files. (default: true)
     */
    pub keep_backups: bool,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            input: InputSettings::default(),
            output: OutputSettings::default()
        }
    }
}

impl InputSettings {
    pub fn default() -> InputSettings {
        InputSettings {
            max_file_size: 32 * 1024 * 1024,
            recursive: true,
            skip_hidden: false,
            follow_symlinks: true,
            extensions: String::from("srt,sub")
        }
    }

    pub fn find_files(&self, working_dir: &str, buf: &mut Vec<PathBuf>) -> Result<()> {
        let mut extensions: HashMap<String, bool> = HashMap::new();

        self.extensions.split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .for_each(|ext| {
                extensions.insert(String::from(ext), true);
            });

        if extensions.len() < 1 {
            return Ok(())
        }

        WalkDir::new(working_dir)
            .follow_links(self.follow_symlinks)
            .contents_first(true)
            .into_iter()
            .flat_map(|e| {
                e.map(|entry| entry.path().to_path_buf())
            })
            .fold(Ok(()), |acc, e| {
                acc.and_then(|_| {
                    let path = e.as_path();
                    let metadata = std::fs::metadata(path)?;
                    let is_valid_extension = path.extension()
                        .map(|os_str| {
                            match os_str.to_str() {
                                Some(ext) => extensions.contains_key(ext),
                                None => false
                            }
                        })
                        .unwrap_or(false);

                    let is_valid_size = metadata.len() <= self.max_file_size;

                    if metadata.is_file() && is_valid_extension && is_valid_size {
                        buf.push(e);
                    }

                    Ok(())
                })
            })
    }
}

impl OutputSettings {
    pub fn default() -> OutputSettings {
        OutputSettings {
            keep_backups: true
        }
    }
}
