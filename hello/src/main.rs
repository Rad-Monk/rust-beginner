use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Rad", about = "QNA")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Rad")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args: Cli = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result: String = hello::daddy(&name);
            println!("{}", result);
        }
        None => println!("no command given"),
    }
}
