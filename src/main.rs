use clap::{Arg, Command};
use sysfetch::core::{DisplayManager, SystemCollector, get_theme};
use sysfetch::utils::cache::get_global_cache;

fn main() {
    let matches = Command::new("sysfetch")
        .version("0.1.0")
        .about("A fast, modular system information fetcher")
        .arg(
            Arg::new("minimal")
                .short('m')
                .long("minimal")
                .help("Show minimal output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("theme")
                .short('t')
                .long("theme")
                .value_name("THEME")
                .help("Theme to use (default, minimal)")
                .default_value("default"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output in JSON format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-cache")
                .long("no-cache")
                .help("Disable caching")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("cache-dir")
                .long("cache-dir")
                .value_name("DIR")
                .help("Cache directory"),
        )
        .get_matches();

    let minimal = matches.get_flag("minimal");
    let theme_name = matches.get_one::<String>("theme").unwrap();
    let json_output = matches.get_flag("json");
    let no_cache = matches.get_flag("no-cache");
    
    if !no_cache {
        if let Some(cache_dir) = matches.get_one::<String>("cache-dir") {
            let _cache = get_global_cache_with_file_cache(cache_dir.clone());
        } else {
            let _cache = get_global_cache();
        }
    }

    let theme = get_theme(theme_name);
    let display_manager = DisplayManager::new(theme, minimal, json_output);
    
    let system_info = SystemCollector::collect_all();
    let output = display_manager.render(&system_info);
    
    println!("{}", output);
}