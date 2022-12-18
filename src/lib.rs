use clap::Parser;
use std::fs;
use std::path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    text: String,
    path: path::PathBuf,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    #[arg(short, long, default_value_t)]
    ignore_dir: String,
}

impl Cli {
    pub fn run(&mut self) {
        // get paths inside path
        let paths = self.get_paths();

        for path in paths {
            if self.verbose {
                //print!("\33[2K\r");
                print!("checking {}{}[2K\r", path.display(), 27 as char);
                //print!("checking {:?}\r", path);
            }
            if is_file(&path) {
                let content = match fs::read_to_string(&path) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                for line in content.lines() {
                    if line.contains(&self.text) {
                        println!("found {} in {}", &self.text, path.display());
                    }
                    continue;
                }
            } else {
                self.path = path;
                Cli::run(self);
            }
        }
    }

    fn get_paths(&self) -> Vec<path::PathBuf> {
        let paths = fs::read_dir(&self.path).unwrap();
        let mut list: Vec<path::PathBuf> = Vec::new();
        for path in paths {
            let name = path.unwrap().path();
            if !self.ignore_dir.is_empty() && name.to_string_lossy().contains(&self.ignore_dir) {
                continue;
            }
            list.push(name);
        }
        list
    }
}

fn is_file(file_name: &path::PathBuf) -> bool {
    //let meta = fs::metadata(file_name).unwrap();
    let meta = match fs::metadata(file_name) {
        Ok(name) => name,
        Err(_) => return false,
    };
    if !meta.is_dir() {
        return true;
    }
    false
}
