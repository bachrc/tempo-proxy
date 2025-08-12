mod server;

use clap::{Parser, Subcommand};

/// Un proxy fictif pour Tempo, configuré via la ligne de commande.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve(ServeArgs),
}

/// Arguments pour la sous-commande `serve`
#[derive(Parser, Debug)]
struct ServeArgs {
    #[arg(
        long,
        short,
        env = "TEMPO_PROXY_INTERFACE",
        default_value = "127.0.0.1:8080"
    )]
    interface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve(args) => {
            println!("Démarrage du serveur sur l'interface : {}", args.interface);

            server::run(&args.interface).await?;
        }
    }

    Ok(())
}
