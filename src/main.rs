extern crate imghdr;
use clap::Parser;

mod file_walker;
mod resizer;

/// Image resizer with compression support
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// A directory with images to be resized or\and compressed
    #[arg(short, long, default_value_t = String::from("."))]
    dir: String,
}

fn main() {
    let args = Args::parse();
    let images = file_walker::get_all_images_in_path(&args.dir)
        .expect("Unexpected error during walking oveer the directory!");

    for img in images {
        println!("{}", img.as_os_str().to_str().expect("expected path!"));
    }
}
