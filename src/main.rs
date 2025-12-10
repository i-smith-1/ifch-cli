use clap::{Arg, Command};
use std::io::{self, Write};
use std::process::Command as ProcessCommand;
use ifch::*;

fn main() {
    clear_terminal();
    display_ascii_art();
    println!("Welcome to IFCH - Iain's Financial Calculation Helper");
    println!("Please choose a command to execute:");

    let matches = Command::new("IFCH")
        .version("1.0")
        .author("Iain Smith <i.smith.code@proton.me")
        .about("Iain's Financial Calculation Helper")
        .subcommand(Command::new("current_ratio")
            .about("Calculates the current ratio")
            .arg(Arg::new("current_assets")
                .short('a')
                .long("assets")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Current assets"))
            .arg(Arg::new("current_liabilities")
                .short('l')
                .long("liabilities")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Current liabilities")))
        .subcommand(Command::new("bsm")
            .about("Calculates Black-Scholes-Merton option pricing")
            .arg(Arg::new("stock_price")
                .short('s')
                .long("stock")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Current stock price"))
            .arg(Arg::new("strike_price")
                .short('k')
                .long("strike")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Option strike price"))
            .arg(Arg::new("time_to_expiration")
                .short('t')
                .long("time")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Time to expiration in years"))
            .arg(Arg::new("risk_free_rate")
                .short('r')
                .long("rate")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Annual risk-free interest rate"))
            .arg(Arg::new("volatility")
                .short('v')
                .long("volatility")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Annual volatility"))
            .arg(Arg::new("dividend_yield")
                .short('q')
                .long("dividend")
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Dividend yield")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("current_ratio") {
        let current_assets: f64 = *matches.get_one("current_assets").unwrap();
        let current_liabilities: f64 = *matches.get_one("current_liabilities").unwrap();
        let ratio = current_r(current_assets, current_liabilities);
        println!("Current Ratio: {:.2}", ratio);
    }

    if let Some(matches) = matches.subcommand_matches("bsm") {
        let stock_price: f64 = *matches.get_one("stock_price").unwrap();
        let strike_price: f64 = *matches.get_one("strike_price").unwrap();
        let time_to_expiration: f64 = *matches.get_one("time_to_expiration").unwrap();
        let risk_free_rate: f64 = *matches.get_one("risk_free_rate").unwrap();
        let volatility: f64 = *matches.get_one("volatility").unwrap();
        let dividend_yield: f64 = *matches.get_one("dividend_yield").unwrap();

        let (call_price, put_price, nd1, nd2) = bsm(
            stock_price,
            strike_price,
            time_to_expiration,
            risk_free_rate,
            volatility,
            dividend_yield,
        );

        println!("Call Price: {:.2}", call_price);
        println!("Put Price: {:.2}", put_price);
        println!("N(d1): {:.2}", nd1);
        println!("N(d2): {:.2}", nd2);
    }

    // Display the menu
    loop {
        print_menu();
        let choice = get_user_input();

        match choice.trim() {
            "1" => run_current_ratio(),
            "2" => run_bsm(),
            "q" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}


fn clear_terminal() {
    if cfg!(target_os = "windows") {
        ProcessCommand::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        ProcessCommand::new("clear")
            .status()
            .unwrap();
    }
}

fn display_ascii_art() {
    println!(
        r#"
 _________ _______  _______          
\__   __/(  ____ \(  ____ \|\     /|
   ) (   | (    \/| (    \/| )   ( |
   | |   | (__    | |      | (___) |
   | |   |  __)   | |      |  ___  |
   | |   | (      | |      | (   ) |
___) (___| )      | (____/\| )   ( |
\_______/|/       (_______/|/     \|
                                    
        "#
    );
}

fn print_menu() {
    println!("\nMenu:");
    println!("1. Calculate Current Ratio");
    println!("2. Calculate Black-Scholes-Merton");
    println!("q. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn run_current_ratio() {
    // Prompt user for input values
    println!("Enter current assets: ");
    let current_assets = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter current liabilities: ");
    let current_liabilities = get_user_input().trim().parse::<f64>().unwrap();

    // Calculate and display the result
    let ratio = current_r(current_assets, current_liabilities);
    println!("Current Ratio: {:.2}", ratio);
}

fn run_bsm() {
    // Prompt user for input values
    println!("Enter stock price: ");
    let stock_price = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter strike price: ");
    let strike_price = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter time to expiration (years): ");
    let time_to_expiration = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter risk-free rate: ");
    let risk_free_rate = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter volatility: ");
    let volatility = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter dividend yield: ");
    let dividend_yield = get_user_input().trim().parse::<f64>().unwrap();

    // Calculate and display the results
    let (call_price, put_price, nd1, nd2) = bsm(
        stock_price,
        strike_price,
        time_to_expiration,
        risk_free_rate,
        volatility,
        dividend_yield,
    );

    println!("Call Price: {:.2}", call_price);
    println!("Put Price: {:.2}", put_price);
    println!("N(d1): {:.2}", nd1);
    println!("N(d2): {:.2}", nd2);
}

