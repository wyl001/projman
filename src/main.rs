use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    query_string: String,
    #[arg(short, long)]
    file_path: String,
    #[arg(short, long)]
    ignore_case: bool
}

fn main() {
    let args = Args::parse();
    println!("{:#?}",args);

}
