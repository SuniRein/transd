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

    /// List available translation engines
    ListEngines,

    /// List supported source languages for a specific translation engine
    ListSourceLanguages(ListLanguagesArgs),

    /// List supported target languages for a specific translation engine
    ListTargetLanguages(ListLanguagesArgs),
}

#[derive(Args)]
struct TranslateArgs {
    /// Translation engine to use (e.g., "google")
    #[arg(short, long)]
    engine: String,

    /// Source language (e.g., "en" for English)
    #[arg(short, long, value_name = "LANG")]
    from: String,

    /// Target language (e.g., "zh-CN" for Chinese (Simplified))
    #[arg(short, long, value_name = "LANG")]
    to: String,

    /// Text to translate
    text: String,
}

#[derive(Args)]
struct ListLanguagesArgs {
    /// Translation engine to query (e.g., "google")
    #[arg(short, long, value_name = "ENGINE")]
    engine: String,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    let cli = Cli::parse();

    match cli.command {
        Command::Translate(args) => cmd_translate(args).await?,
        Command::ListEngines => cmd_list_engines().await?,
        Command::ListSourceLanguages(args) => cmd_list_source_languages(args).await?,
        Command::ListTargetLanguages(args) => cmd_list_target_languages(args).await?,
    }

    Ok(())
}

async fn cmd_translate(args: TranslateArgs) -> Result<(), Report> {
    let translator = MozhiTranslator::new(MOZHI_INSTANCE.clone());
    match translator
        .translate(&args.text, &args.engine, &args.from, &args.to)
        .await
    {
        Ok(result) => println!("Translation: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

async fn cmd_list_engines() -> Result<(), Report> {
    let translator = MozhiTranslator::new(MOZHI_INSTANCE.clone());
    match translator.list_engines().await {
        Ok(engines) => {
            println!("Available translation engines:");
            for engine in engines {
                println!("- {} ({})", engine.name, engine.id);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

async fn cmd_list_source_languages(args: ListLanguagesArgs) -> Result<(), Report> {
    let translator = MozhiTranslator::new(MOZHI_INSTANCE.clone());
    match translator.list_source_languages(&args.engine).await {
        Ok(languages) => {
            println!("Supported source languages for engine '{}':", args.engine);
            for lang in languages {
                println!("- {} ({})", lang.name, lang.id);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

async fn cmd_list_target_languages(args: ListLanguagesArgs) -> Result<(), Report> {
    let translator = MozhiTranslator::new(MOZHI_INSTANCE.clone());
    match translator.list_target_languages(&args.engine).await {
        Ok(languages) => {
            println!("Supported target languages for engine '{}':", args.engine);
            for lang in languages {
                println!("- {} ({})", lang.name, lang.id);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
