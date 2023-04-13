use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "Count the number of days between two dates"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selina Liu")]
    Days {
        #[clap(short, long)]
        first: String,
        #[clap(short, long)]
        second: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Days { first, second }) => {
            let result = days_between::days_between_dates(first.clone(), second.clone());
            println!(
                "Number of days between {} and {} is: {}",
                first, second, result
            );
        }
        None => println!("No subcommand was used"),
    }
}
