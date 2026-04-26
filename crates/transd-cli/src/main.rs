use clap::{Args, Parser, Subcommand};
use rootcause::prelude::*;
use std::{env, sync::LazyLock};
use transd_provider_mozhi::MozhiTranslator;
use transd_translate::Translator;

static MOZHI_INSTANCE: LazyLock<String> = LazyLock::new(|| {
    env::var("MOZHI_INSTANCE").expect("MOZHI_INSTANCE environment variable is not set")
});

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Translate text from one language to another
    Translate(TranslateArgs),
}

#[derive(Args)]
struct TranslateArgs {
    /// Source language (e.g., "en" for English)
    #[arg(short, long, value_name = "LANG")]
    from: String,

    /// Target language (e.g., "zh-CN" for Chinese (Simplified))
    #[arg(short, long, value_name = "LANG")]
    to: String,

    /// Text to translate
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    let cli = Cli::parse();

    match cli.command {
        Command::Translate(args) => cmd_translate(args).await?,
    }

    Ok(())
}

async fn cmd_translate(args: TranslateArgs) -> Result<(), Report> {
    let translator = MozhiTranslator::new(MOZHI_INSTANCE.clone());
    match translator.translate(&args.text, &args.from, &args.to).await {
        Ok(result) => println!("Translation: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
