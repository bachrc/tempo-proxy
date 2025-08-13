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
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve(args) => {
            tracing::info!("Démarrage du serveur sur l'interface : {}", args.interface);

            tempo_proxy_api::server::run(&args.interface).await?;
        }
    }

    Ok(())
}