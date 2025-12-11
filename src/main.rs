use clap::{Arg, Command};
use std::io::{self, Write};
use std::process::Command as ProcessCommand;
use ifch::*;

fn main() {
    clear_terminal();
    display_ascii_art();
    println!("Welcome to IFCH - Iain's Financial Calculation Helper");

    loop {
        main_menu();
        let choice = get_user_input();

        match choice.trim() {
            "1" => ratios_menu(),
            "2" => time_value_of_money_menu(),
            "3" => build_ups_menu(),
            "4" => wacc_menu(),
            "5" => valuation_menu(),
            "6" => options_menu(),
            "q" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn main_menu() {
    println!("\nMain Menu:");
    println!("1. Ratios");
    println!("2. Time Value of Money");
    println!("3. Build Ups");
    println!("4. WACC");
    println!("5. Valuation");
    println!("6. Options");
    println!("q. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn ratios_menu() {
    println!("\nRatios Menu:");
    println!("1. Current Ratio");
    println!("q. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_current_ratio(),
        "q" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn options_menu() {
    println!("\nOptions Menu:");
    println!("1. Black-Scholes-Merton");
    println!("q. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_bsm(),
        "q" => return,
        _ => println!("Invalid option, please try again."),
    }
}

// Implement similar functions for other menus
fn time_value_of_money_menu() {
    // Placeholder for future implementation
    println!("\nTime Value of Money Menu:");
    println!("q. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn build_ups_menu() {
    // Placeholder for future implementation
    println!("\nBuild Ups Menu:");
    println!("q. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn wacc_menu() {
    // Placeholder for future implementation
    println!("\nWACC Menu:");
    println!("q. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn valuation_menu() {
    // Placeholder for future implementation
    println!("\nValuation Menu:");
    println!("q. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

// Existing functions remain unchanged
// ...

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

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn run_current_ratio() {
    println!("Enter current assets: ");
    let current_assets = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter current liabilities: ");
    let current_liabilities = get_user_input().trim().parse::<f64>().unwrap();

    let ratio = current_r(current_assets, current_liabilities);
    println!("Current Ratio: {:.2}", ratio);
}

fn run_bsm() {
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

