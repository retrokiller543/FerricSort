#![allow(clippy::items_after_test_module)]
use clap::Parser;
use ferric_sort::file;

#[derive(Parser)]
#[clap(
    version,
    author = "Retrokiller543",
    name = "FerricSort",
    about = "A fast sorting algorithm for large files"
)]
struct Opts {
    file: Option<String>,
    #[clap(short, long)]
    new_name: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.file {
        Some(file) => {
            let path = std::path::Path::new(&file);
            let file = file::File::read_file(path);
            let file = file.sort();

            match opts.new_name {
                Some(name) => {
                    file.write_file(Some(name));
                }
                None => {
                    file.write_file(None);
                }
            }
        }
        None => {
            println!("No file specified");
        }
    }
}
