use std::{fs, io::IsTerminal, process::exit};

use clap::{Parser, ValueEnum};

mod tldr_helper;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Platform {
    Android,
    Common,
    FreeBsd,
    Linux,
    NetBsd,
    OpenBsd,
    Osx,
    SunOs,
    Windows,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Parser)]
#[command(about, long_about=None)]
#[derive(Debug)]
struct Cli {
    /// The name of the tool you want to see the tldr page for.
    name: Option<Vec<String>>,

    #[arg(
        short,
        long,
        require_equals = true,
        value_name = "platform",
        help = "Specify the platform of the command.",
        value_enum
    )]
    platform: Option<Platform>,

    #[arg(short, long, help = "Update the TLDR cache.")]
    update: bool,

    #[arg(short, long, help = "Deletes the local tldr cache and refreshes it.")]
    reset: bool,

    #[arg(short, long, help = "Print version.")]
    version: bool,

    #[arg(long, name = "cache-dir", help = "Gets the cache directory.")]
    cache_dir: bool,

    #[arg(long, help = "Prints the version of the tldr pages")]
    pages_version: bool,

    #[arg(short, long, name = "path", help = "Print's only the file path.")]
    file_path: bool,

    #[arg(
        short = 'L',
        long,
        help = "2 letters describing the language to find the command in."
    )]
    language: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    // let tldr_cache = dirs::home_dir().unwrap().join(".config/tldr-2");
    let tldr_cache = dirs::cache_dir().unwrap().join("tldr-rs");

    if !tldr_cache.join("version").exists() {
        print!("TLDR has not been initialized. Initializing now.");
        tldr_helper::initialize(&tldr_cache);
    }

    if cli.version {
        let app_version: &str = env!("CARGO_PKG_VERSION");
        println!("v{}", app_version);

        return;
    }

    if cli.pages_version {
        let version = fs::read_to_string(tldr_cache.join("version")).unwrap();
        println!("{}", version);
        return;
    }

    if cli.cache_dir {
        if std::io::stdout().is_terminal() {
            print!("Cache Directory: {:?}", &tldr_cache.display());
        } else {
            print!("{}", &tldr_cache.display());
        }
        return;
    }

    if cli.update {
        let current_version = fs::read_to_string(tldr_cache.join("version")).unwrap();
        let latest_version = tldr_helper::get_latest_version();

        if current_version == latest_version {
            println!("No new updates...");
        }

        return;
    }

    if cli.reset {
        println!("Deleting {:?}", &tldr_cache);
        std::fs::remove_dir_all(&tldr_cache).unwrap_or_else(|err| {
            println!(
                "Unable to delete directory {:?} due to {}",
                &tldr_cache, err
            )
        });

        tldr_helper::initialize(&tldr_cache);

        return;
    }

    if cli.name.is_none() {
        println!("Please enter a command. use tldr --help to see usage");
        return;
    }

    let command = match cli.name {
        Some(x) if !x.is_empty() => x.join("-"),
        _ => {
            println!("Please enter a command. use tldr --help to see usage");
            exit(-1);
        }
    };

    let languages = match cli.language {
        Some(lang) => vec![lang],
        None => tldr_helper::get_languages_from_environment(),
    };

    let selected_platform = cli.platform.map(|x| x.to_string());

    if cli.file_path {
        let location =
            tldr_helper::get_page_location(&command, &tldr_cache, selected_platform, languages);
        match location {
            Some((file_path, _)) => {
                println!("{}", file_path.to_str().unwrap())
            }
            _ => println!("File not found"),
        }
    } else {
        let theme = markterm::themes::get_default_theme();

        tldr_helper::read_page(&command, &tldr_cache, selected_platform, languages, &theme);
    }
}
