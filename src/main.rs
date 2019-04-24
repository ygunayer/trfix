use std::fs;
use std::env;
use glob::glob;

use std::io::{Write, BufRead, BufReader, BufWriter};
use std::fs::{OpenOptions, File};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn fix_string(input: String) -> String {
    input.replace("ý", "ı")
        .replace("þ", "ş")
        .replace("ð", "ğ")

        .replace("Þ", "Ş")
        .replace("Ý", "İ")
        .replace("Ð", "Ğ")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = args.get(1).map(|s| s.as_str()).unwrap_or("*.srt");

    for entry in glob(pattern).expect("Failed to execute glob") {
        match entry {
            Ok(path) => {
                let filename = path.into_os_string().into_string().expect("Failed to materialize path");
                let tmp_filename = filename.clone() + ".tmp";

                {
                    let input_file = File::open(filename.clone()).expect("Failed to open file");
                    let output_file = OpenOptions::new().read(true).write(true).create(true).truncate(true).open(tmp_filename.clone()).expect("Failed to create output file");

                    let mut writer = BufWriter::new(output_file);
                    for line in BufReader::new(input_file).lines() {
                        let input_line = line.expect("Unable to read line");
                        let output_line = fix_string(input_line);
                        let output_bytes = output_line.as_bytes();
                        writer.write(output_bytes).expect("Unable to write to output");
                        writer.write(LINE_ENDING.as_bytes()).expect("Unable to write to output");
                    }
                }

                fs::remove_file(filename.clone()).expect("Failed to remove original file");
                fs::rename(tmp_filename, filename.clone()).expect("Failed to replace old file wıth the replacement");
                println!("Successfully replaced the contents of file {:?}", filename);
            },
            Err(e) => println!("Error handling file {:?}", e)
        }
    }
}
