extern crate rand;
extern crate time;
extern crate dirs;
extern crate crossterm;

use structopt::StructOpt;
use rand::{Rng};
use rand::distributions::{Alphanumeric};
use std::fs::OpenOptions;
use std::fs::create_dir_all;
use std::io::Write;
use time::{PreciseTime};
use std::collections::BTreeSet;
use dirs::home_dir;
use std::fs::{self, File};
use std::io::Error;
use std::path::PathBuf;
use std::result::Result;
use crossterm::Crossterm;
use crossterm::TerminalCursor;
use crossterm::ClearType;
use crossterm::{Colored, Color, Colorize, Styler, Attribute};

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    number_of_codes: u32,
    number_codes_per_file: u32,
}

fn main() {

    let crossterm = Crossterm::new();
    //let color = crossterm.color();
    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal();

    terminal.clear(ClearType::All).ok();

    const CODE_LENGTH: usize = 6;
    let start_time = PreciseTime::now();
    let mut codes_list: BTreeSet<String> = BTreeSet::new();

    let args = Cli::from_args();
    let mut rng = rand::thread_rng();
    let mut generated_counter: u32 = 0;
    let mut duplicate_counter: u32 = 0;
    let mut number_of_files: u32 = 1;

    println!("Preparing folder if not available");
    let codes_directory = home_dir().unwrap().join("campaigncodes");

    create_dir_all(&codes_directory).expect("Failed to create directory for codes.");
    let msg_folder_ok = "Folder for codes already exists or was now created.".green().on(Color::Black);
    println!("{}", msg_folder_ok);
    
    delete_dir_contents(&codes_directory).expect("Failed to empty codes directory");
    println!("Folder for codes is empty and ready.");

    let mut current_code_file = file_for_codes(&cursor, &codes_directory, number_of_files);

    while generated_counter < args.number_of_codes {

        if generated_counter >= number_of_files * args.number_codes_per_file {
            number_of_files = number_of_files + 1;
            current_code_file = file_for_codes(&cursor, &codes_directory, number_of_files);
        }

        let code = generate_code(&mut rng, CODE_LENGTH);
        let code_copy = code.clone(); // how to avoid this copy

        if codes_list.insert(code) {
            writeln!(&mut current_code_file, "{}", code_copy).unwrap();
            generated_counter = generated_counter + 1;
        } else {
            duplicate_counter = duplicate_counter + 1;
        }
    }

    println!("Duplicates skipped {}", duplicate_counter);
    println!("Generated {} codes into {} files", args.number_of_codes, number_of_files);
    println!("Operation took {}", start_time.to(PreciseTime::now()));
}


fn generate_code(rng: &mut rand::rngs::ThreadRng, code_length: usize) -> String {
    rng.sample_iter(&Alphanumeric).take(code_length).collect()
}

fn file_for_codes(cursor: &TerminalCursor, path: &PathBuf, file_counter: u32) -> File {

    cursor.goto(0, 5).expect("failed this");
    println!("Preparing file {}", file_counter);

    // print percentage progress

    let file_name = format!("codes-{}.txt", file_counter);
    let file_path = path.join(file_name);

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path);

    let f = match f {
        Ok(file)  => file,
        Err(_)  => panic!("ERR"),
    };

    f
}

fn delete_dir_contents(src_dir: &PathBuf) -> Result<(), Error> {

    let dir = match src_dir.read_dir() {
        Ok(dir)  => dir,
        Err(e) => return Err(e),
    };

    for entry in dir {
        let file = match entry {
            Ok(file)  => file,
            Err(e) => return Err(e),
        };
        fs::remove_file(file.path()).expect("failed");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_generate_alphanumeric_string_of_set_length() {
        let mut rng = rand::thread_rng();
        assert_eq!(5, generate_code(&mut rng, 5).len());
    }

}
