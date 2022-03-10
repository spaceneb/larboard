use std::env;
use rttp_client::HttpClient;
use json::parse;
use json::JsonValue;

static NAME:&str =                  "larboard";
static COMMAND:&str =               "lar";
static VERSION:&str =               "2022.3.9";
static PACKAGES_URL:&str =          "https://formulae.brew.sh/api/formula.json";
static CASKS_URL:&str =             "https://formulae.brew.sh/api/cask.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <2 {
        print_help();
    } else {
        match args[1].as_str() {
            "-h" | "--help" => print_help(),
            "-S" | "--sync" => sync(),
            "-V" | "--version" => print_version(),
            _ => unknown(&args[1]),
        }
    }
}    

fn sync() {
    let data = HttpClient::new()
        .get()
        .url(PACKAGES_URL)
        .emit();
    println!("{}",data.unwrap());
    
}

fn print_version() {
    println!("Version of {}: {}", NAME, VERSION);
}

fn print_help() {
    println!("{} help page", NAME);
    println!("Usage: {} <operation> [options] [targets]", COMMAND);
    println!(" -I, --install:        Download, build, and install the package(s)");
    println!("  -R, --remove:        Remove the package(s) from the system");
    println!("    -S, --sync:        Sync with the remote server.");
    println!(" -U, --upgrade:        Upgrade the package(s) on the system");
    println!(" -V, --version:        Display the version and exit");
    println!("    -h, --help:        Show this help message and exit");
}

fn unknown(text:&String) {
    println!("{} is an unknown argument. Try running `-h` if you are unsure what to do.", text);
}