use trfix::errors::{Result, Error};
use trfix::settings::Settings;

use std::path::{PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write, BufReader, BufWriter};

use encoding_rs::{Encoding, DecoderResult};

fn fix_string(input: &String) -> String {
    input.replace("ý", "ı")
        .replace("þ", "ş")
        .replace("ð", "ğ")

        .replace("Þ", "Ş")
        .replace("Ý", "İ")
        .replace("Ð", "Ğ")
}

trait PushExtension<T> {
    fn push_extension(&self, ext: &str) -> T;
}

impl PushExtension<PathBuf> for PathBuf {
    fn push_extension(&self, ext: &str) -> PathBuf {
        let old_ext = &self.extension();
        let mut new_extension = String::new();

        if old_ext.is_some() {
            if old_ext.unwrap().to_str().is_some() {
                new_extension.push_str(old_ext.unwrap().to_str().unwrap());
            }
        }

        new_extension.push_str(ext);

        let mut new_path = PathBuf::from(&self);
        new_path.set_extension(new_extension);
        new_path.to_owned()
    }
}

fn process_file(file_path: &PathBuf, settings: &Settings) -> Result<()> {
    let metadata = fs::metadata(file_path)?;

    if !metadata.is_file() {
        return Err(Error::not_a_file(file_path))
    }

    let file_size = metadata.len();
    let input_settings = &settings.input;
    let output_settings = &settings.output;
    let max_file_size = input_settings.max_file_size;

    if file_size > max_file_size {
        // handling larger files is not yet supported
        return Err(Error::file_too_large(file_path))
    }

    let initial_read_size = std::cmp::min(file_size, max_file_size) as usize;

    let mut contents = Vec::with_capacity(initial_read_size as usize);

    {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut contents)?;
    }

    let mime_type = tree_magic::from_u8(contents.as_ref());
    if ("text/plain" != mime_type) && ("application/x-subrip" != mime_type) {
        return Err(Error::unsupported_mime_type(mime_type.as_ref()))
    }

    let (charset_name, _, _) = chardet::detect(&contents);
    let encoding_name = chardet::charset2encoding(&charset_name);
    println!("Found file {:?} to have encoding {:?})", file_path, charset_name);

    let encoding = Encoding::for_label(encoding_name.as_bytes())
        .ok_or_else(|| Error::unsupported_encoding(encoding_name))?;

    let mut decoder = encoding.new_decoder();
    let mut input_str = &mut String::with_capacity(initial_read_size * 2);
    let (decode_result, _) = decoder.decode_to_string_without_replacement(contents.as_ref(), &mut input_str, true);

    match decode_result {
        DecoderResult::InputEmpty => {
            let output_str = fix_string(&input_str);
            if &output_str == input_str {
                println!("Content has not changed, ignoring file {:?}", file_path)
            } else {
                let backup_file_path = file_path.push_extension(".bak");

                fs::copy(file_path, &backup_file_path)?;

                let output_file = fs::OpenOptions::new().create(true).write(true).truncate(true).open(file_path)?;
                let mut writer = BufWriter::new(output_file);
                writer.write_all(output_str.as_bytes())?;

                if !output_settings.keep_backups {
                    std::fs::remove_file(&backup_file_path)?;
                }
                println!("Successfully fixed file {:?}", file_path);
            }


            Ok(())
        },
        DecoderResult::Malformed(_, _) => Err(Error::malformed_input(file_path)),
        DecoderResult::OutputFull => Err(Error::output_buffer_exhausted(file_path))
    }
}

fn process_files(settings: &Settings) -> Result<()> {
    let mut files: Vec<PathBuf> = Vec::new();
    settings.input.find_files("./", &mut files)
        .and_then(|_| {
            files.into_iter()
                .fold(Ok(()), |acc, file| {
                    acc.and_then(|_| process_file(&file, &settings))
                })
        })
}

fn main() {
    let settings = Settings::default();

    let result = process_files(&settings);

    match result {
        Ok(()) => println!("Successfully finished processing files"),
        Err(e) => println!("An error has occurred {:?}", e)
    }
}
