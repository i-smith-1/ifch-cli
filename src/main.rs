use clap::{Arg, Command};
use std::io::{self, Write};
use std::process::Command as ProcessCommand;
use ifch::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Define global variables using once_cell
static LAST_RESULT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static HISTORY: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

// Helper Functions

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
    println!("Welcome to IFCH - Iain's Financial Calculation Helper");
    let last_result = LAST_RESULT.lock().unwrap();
    println!("{: >60}", *last_result);
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

// Function to display history
fn display_history() {
    clear_terminal();
    display_ascii_art();
    println!("\nHistory of Results:");
    let history = HISTORY.lock().unwrap();
    for result in history.iter() {
        println!("{}", result);
    }
    println!("\nPress Enter to return to the main menu...");
    let _ = get_user_input();
}

fn main() {
    clear_terminal();
    display_ascii_art();
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
            "H" | "h" => display_history(),
            "q" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn main_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nMain Menu:");
    println!("1. Ratios");
    println!("2. Time Value of Money");
    println!("3. Build Ups");
    println!("4. WACC");
    println!("5. Valuation");
    println!("6. Options");
    println!("--------------");
    println!("h. History");
    println!("q. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}


fn ratios_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nRatios Menu:");
    println!("1. Liquidity Ratios");
    println!("2. Profitability Ratios");
    println!("3. Leverage Ratios");
    println!("4. Activity Ratios");
    println!("5. Valuation Ratios");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => liquidity_ratios_menu(),
        "2" => profitability_ratios_menu(),
        "3" => leverage_ratios_menu(),
        "4" => activity_ratios_menu(),
        "5" => valuation_ratios_menu(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn liquidity_ratios_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nLiquidity Ratios Menu:");
    println!("1. Quick Ratio");
    println!("2. Acid Test Ratio");
    println!("3. Cash Ratio");
    println!("4. Current Ratio");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_quick_ratio(),
        "2" => run_acid_test_ratio(),
        "3" => run_cash_ratio(),
        "4" => run_current_ratio(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_quick_ratio() {
    println!("Enter current assets: ");
    let current_assets = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter inventory: ");
    let inventory = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter current liabilities: ");
    let current_liabilities = get_user_input().trim().parse::<f64>().unwrap();

    let ratio = quick_r(current_assets, inventory, current_liabilities);
    println!("Quick Ratio: {:.2}", ratio);
    {
        let result = format!("Quick Ratio = {:.2}", ratio);
        *LAST_RESULT.lock().unwrap() = format!("Last: {}", result);
        HISTORY.lock().unwrap().push(result);
    }
}


fn run_acid_test_ratio() {
    println!("Enter cash: ");
    let cash = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter inventory: ");
    let inventory = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter accounts receivable: ");
    let accounts_receivable = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter current liabilities: ");
    let current_liabilities = get_user_input().trim().parse::<f64>().unwrap();

    let ratio = acid_r(cash, inventory, accounts_receivable, current_liabilities);
    println!("Acid Test Ratio: {:.2}", ratio);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
    *last_result = format!("Last: Acid Test Ratio = {:.2}", ratio);
}

fn run_cash_ratio() {
    println!("Enter cash and equivalents: ");
    let cash_and_equivalents = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter current liabilities: ");
    let current_liabilities = get_user_input().trim().parse::<f64>().unwrap();

    let ratio = cash_r(cash_and_equivalents, current_liabilities);
    println!("Cash Ratio: {:.2}", ratio);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
     *last_result = format!("Last: Cash Ratio = {:.2}", ratio);
}

fn run_current_ratio() {
    println!("Enter current assets: ");
    let current_assets = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter current liabilities: ");
    let current_liabilities = get_user_input().trim().parse::<f64>().unwrap();

    let ratio = current_r(current_assets, current_liabilities);
    println!("Current Ratio: {:.2}", ratio);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
     *last_result = format!("Last: Current Ratio = {:.2}", ratio);
}


fn profitability_ratios_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nProfitability Ratios Menu:");
    println!("1. Gross Margin");
    println!("2. Operating Margin");
    println!("3. Net Margin");
    println!("4. Return on Assets (ROA)");
    println!("5. Return on Equity (ROE)");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_gross_margin(),
        "2" => run_operating_margin(),
        "3" => run_net_margin(),
        "4" => run_return_on_assets(),
        "5" => run_return_on_equity(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_gross_margin() {
    println!("Enter gross profit: ");
    let gross_profit = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter revenue: ");
    let revenue = get_user_input().trim().parse::<f64>().unwrap();

    let margin = gross_m(gross_profit, revenue);
    println!("Gross Margin: {:.2}", margin);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
     *last_result = format!("Last: Gross Margin = {:.2}", margin);
}

fn run_operating_margin() {
    println!("Enter operating income: ");
    let operating_income = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter revenue: ");
    let revenue = get_user_input().trim().parse::<f64>().unwrap();

    let margin = operating_m(operating_income, revenue);
    println!("Operating Margin: {:.2}", margin);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
     *last_result = format!("Last: Operating Margin = {:.2}", margin);
}

fn run_net_margin() {
    println!("Enter net income: ");
    let net_income = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter revenue: ");
    let revenue = get_user_input().trim().parse::<f64>().unwrap();

    let margin = net_m(net_income, revenue);
    println!("Net Margin: {:.2}", margin);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
     *last_result = format!("Last: Net Margin = {:.2}", margin);
}
fn run_return_on_assets() {
    println!("Enter net income: ");
    let net_income = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter total assets: ");
    let total_assets = get_user_input().trim().parse::<f64>().unwrap();

    let roa = r_o_a(net_income, total_assets);
    println!("Return on Assets (ROA): {:.2}", roa);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
     *last_result = format!("Last: ROA = {:.2}", roa);
}
fn run_return_on_equity() {
    println!("Enter net income: ");
    let net_income = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter shareholders' equity: ");
    let shareholders_equity = get_user_input().trim().parse::<f64>().unwrap();

    let roe = r_o_e(net_income, shareholders_equity);
    println!("Return on Equity (ROE): {:.2}", roe);
    // Update the last result
    let mut last_result = LAST_RESULT.lock().unwrap();
     *last_result = format!("Last: ROE = {:.2}", roe);
}
fn leverage_ratios_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nLeverage Ratios Menu:");
    println!("1. Debt to Equity Ratio");
    println!("2. Debt Ratio");
    println!("3. EBIT Interest Coverage Ratio");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_debt_to_equity_ratio(),
        "2" => run_debt_ratio(),
        "3" => run_ebit_interest_coverage(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_debt_to_equity_ratio() {
    println!("Enter total debt: ");
    let total_debt = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter shareholders' equity: ");
    let shareholders_equity = get_user_input().trim().parse::<f64>().unwrap();

    let dte = d_t_e(total_debt, shareholders_equity);
    println!("Debt to Equity Ratio: {:.2}", dte);
}

fn run_debt_ratio() {
    println!("Enter total debt: ");
    let total_debt = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter total assets: ");
    let total_assets = get_user_input().trim().parse::<f64>().unwrap();

    let dr = d_r(total_debt, total_assets);
    println!("Debt Ratio: {:.2}", dr);
}

fn run_ebit_interest_coverage() {
    println!("Enter EBIT: ");
    let ebit = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter interest expense: ");
    let interest_expense = get_user_input().trim().parse::<f64>().unwrap();

    let ebit_ic = ebit_i_c(ebit, interest_expense);
    println!("EBIT Interest Coverage Ratio: {:.2}", ebit_ic);
}

fn activity_ratios_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nActivity Ratios Menu:");
    println!("1. Inventory Turnover");
    println!("2. Receivables Turnover");
    println!("3. Asset Turnover");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_inventory_turnover(),
        "2" => run_receivables_turnover(),
        "3" => run_asset_turnover(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_inventory_turnover() {
    println!("Enter cost of goods sold: ");
    let cost_of_goods_sold = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter average inventory: ");
    let average_inventory = get_user_input().trim().parse::<f64>().unwrap();

    let inv_turnover = inv_t(cost_of_goods_sold, average_inventory);
    println!("Inventory Turnover: {:.2}", inv_turnover);
}

fn run_receivables_turnover() {
    println!("Enter revenue: ");
    let revenue = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter average accounts receivable: ");
    let average_accounts_receivable = get_user_input().trim().parse::<f64>().unwrap();

    let rec_turnover = rec_t(revenue, average_accounts_receivable);
    println!("Receivables Turnover: {:.2}", rec_turnover);
}

fn run_asset_turnover() {
    println!("Enter revenue: ");
    let revenue = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter total assets: ");
    let total_assets = get_user_input().trim().parse::<f64>().unwrap();

    let asset_turnover = a_t(revenue, total_assets);
    println!("Asset Turnover: {:.2}", asset_turnover);
}

fn valuation_ratios_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nValuation Ratios Menu:");
    println!("1. Price to Earnings (P/E) Ratio");
    println!("2. Price to Book (P/B) Ratio");
    println!("3. Dividend Yield");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_price_to_earnings_ratio(),
        "2" => run_price_to_book_ratio(),
        "3" => run_dividend_yield(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_price_to_earnings_ratio() {
    println!("Enter share price: ");
    let share_price = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter earnings per share: ");
    let earnings_per_share = get_user_input().trim().parse::<f64>().unwrap();

    let pte = p_t_e(share_price, earnings_per_share);
    println!("Price to Earnings (P/E) Ratio: {:.2}", pte);
}

fn run_price_to_book_ratio() {
    println!("Enter share price: ");
    let share_price = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter book value per share: ");
    let book_value_per_share = get_user_input().trim().parse::<f64>().unwrap();

    let ptb = p_t_b(share_price, book_value_per_share);
    println!("Price to Book (P/B) Ratio: {:.2}", ptb);
}

fn run_dividend_yield() {
    println!("Enter annual dividends per share: ");
    let annual_dividends_per_share = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter share price: ");
    let share_price = get_user_input().trim().parse::<f64>().unwrap();

    let div_yield = div_y(annual_dividends_per_share, share_price);
    println!("Dividend Yield: {:.2}", div_yield);
}

fn options_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nOptions Menu:");
    println!("1. Black-Scholes-Merton");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_bsm(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
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


fn time_value_of_money_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nTime Value of Money Menu:");
    println!("1. Calculate XNPV");
    println!("2. Calculate XIRR");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_xnpv(),
        "2" => run_xirr(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_xnpv() {
    let cashflows = get_cashflows();
    println!("Enter discount rate (as a decimal, e.g., 0.05 for 5%): ");
    let discount_rate = get_user_input().trim().parse::<f64>().unwrap();

    // Convert Vec<(f64, String)> to Vec<(f64, &str)>
    let cashflows_ref: Vec<(f64, &str)> = cashflows.iter().map(|(amt, date)| (*amt, date.as_str())).collect();

    let npv = xnpv(cashflows_ref, discount_rate);
    println!("XNPV: {:.2}", npv);
}

fn run_xirr() {
    let cashflows = get_cashflows();

    // Convert Vec<(f64, String)> to Vec<(f64, &str)>
    let cashflows_ref: Vec<(f64, &str)> = cashflows.iter().map(|(amt, date)| (*amt, date.as_str())).collect();

    let irr = xirr(cashflows_ref);
    println!("XIRR: {:.6}", irr);
}

fn get_cashflows() -> Vec<(f64, String)> {
    let mut cashflows = Vec::new();
    loop {
        println!("Enter cash flow amount (or 'done' to finish): ");
        let amount_input = get_user_input().trim().to_string();
        if amount_input.to_lowercase() == "done" {
            break;
        }
        let amount = amount_input.parse::<f64>().expect("Invalid amount");

        println!("Enter date for this cash flow (YYYY-MM-DD): ");
        let date = get_user_input().trim().to_string();

        cashflows.push((amount, date));
    }
    cashflows
}


fn build_ups_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nBuild Ups Menu:");
    println!("1. FCFF using Net Income");
    println!("2. FCFF using CFO");
    println!("3. FCFF using EBIT");
    println!("4. FCFF using EBITDA");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_fcff_ni(),
        "2" => run_fcff_cfo(),
        "3" => run_fcff_ebit(),
        "4" => run_fcff_ebitda(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_fcff_ni() {
    println!("Enter net income: ");
    let net_income = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter non-cash charges: ");
    let non_cash_charges = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter interest: ");
    let interest = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter capital expenditures (CapEx): ");
    let capex = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter change in working capital: ");
    let change_in_working_capital = get_user_input().trim().parse::<f64>().unwrap();

    let fcff = fcff_ni(net_income, non_cash_charges, interest, tax_rate, capex, change_in_working_capital);
    println!("FCFF using Net Income: {:.2}", fcff);
}

fn run_fcff_cfo() {
    println!("Enter cash flow from operations (CFO): ");
    let cfo = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter interest expense: ");
    let interest_expense = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter capital expenditures (CapEx): ");
    let capex = get_user_input().trim().parse::<f64>().unwrap();

    let fcff = fcff_cfo(cfo, interest_expense, tax_rate, capex);
    println!("FCFF using CFO: {:.2}", fcff);
}

fn run_fcff_ebit() {
    println!("Enter EBIT: ");
    let ebit = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter depreciation: ");
    let depreciation = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter capital expenditures (CapEx): ");
    let capex = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter change in working capital: ");
    let change_in_working_capital = get_user_input().trim().parse::<f64>().unwrap();

    let fcff = fcff_ebit(ebit, tax_rate, depreciation, capex, change_in_working_capital);
    println!("FCFF using EBIT: {:.2}", fcff);
}

fn run_fcff_ebitda() {
    println!("Enter EBITDA: ");
    let ebitda = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter depreciation: ");
    let depreciation = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter capital expenditures (CapEx): ");
    let capex = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter change in working capital: ");
    let change_in_working_capital = get_user_input().trim().parse::<f64>().unwrap();

    let fcff = fcff_ebitda(ebitda, tax_rate, depreciation, capex, change_in_working_capital);
    println!("FCFF using EBITDA: {:.2}", fcff);
}


fn wacc_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nWACC Menu:");
    println!("1. Calculate WACC using Cost of Equity");
    println!("2. Calculate Cost of Equity (COE)");
    println!("3. Calculate WACC using Equity Beta");
    println!("4. Calculate Market Risk Premium (MRP)");
    println!("5. Calculate Equity Beta");
    println!("6. Calculate Asset Beta");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_wacc_coe(),
        "2" => run_coe(),
        "3" => run_wacc_beta(),
        "4" => run_mrp(),
        "5" => run_equity_beta(),
        "6" => run_asset_beta(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_wacc_coe() {
    println!("Enter cost of equity (COE): ");
    let coe = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter weight of equity (WE): ");
    let we = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter cost of debt (COD): ");
    let cod = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter weight of debt (WD): ");
    let wd = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter cost of preferred stock (COP): ");
    let cop = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter weight of preferred stock (WP): ");
    let wp = get_user_input().trim().parse::<f64>().unwrap();

    let wacc = wacc_coe(coe, we, tax_rate, cod, wd, cop, wp);
    println!("WACC using Cost of Equity: {:.2}", wacc);
}

fn run_coe() {
    println!("Enter risk-free rate (RFR): ");
    let rfr = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter equity beta: ");
    let equity_beta = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter market risk premium (MRP): ");
    let mrp = get_user_input().trim().parse::<f64>().unwrap();

    let coe_value = coe(rfr, equity_beta, mrp);
    println!("Cost of Equity (COE): {:.2}", coe_value);
}

fn run_wacc_beta() {
    println!("Enter equity beta: ");
    let equity_beta = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter risk-free rate (RFR): ");
    let rfr = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter market risk premium (MRP): ");
    let mrp = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter weight of equity (WE): ");
    let we = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter cost of debt (COD): ");
    let cod = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter weight of debt (WD): ");
    let wd = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter cost of preferred stock (COP): ");
    let cop = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter weight of preferred stock (WP): ");
    let wp = get_user_input().trim().parse::<f64>().unwrap();

    let wacc = wacc_beta(equity_beta, rfr, mrp, we, tax_rate, cod, wd, cop, wp);
    println!("WACC using Equity Beta: {:.2}", wacc);
}

fn run_mrp() {
    println!("Enter equity market return: ");
    let equity_market_return = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter risk-free rate (RFR): ");
    let rfr = get_user_input().trim().parse::<f64>().unwrap();

    let mrp_value = mrp(equity_market_return, rfr);
    println!("Market Risk Premium (MRP): {:.2}", mrp_value);
}

fn run_equity_beta() {
    println!("Enter equity: ");
    let equity = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter debt: ");
    let debt = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter asset beta: ");
    let asset_beta_value = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();

    let equity_beta_value = equity_beta(equity, debt, asset_beta_value, tax_rate);
    println!("Equity Beta: {:.2}", equity_beta_value);
}

fn run_asset_beta() {
    println!("Enter equity: ");
    let equity = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter debt: ");
    let debt = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter equity beta: ");
    let equity_beta_value = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter tax rate: ");
    let tax_rate = get_user_input().trim().parse::<f64>().unwrap();

    let asset_beta_value = asset_beta(equity, debt, equity_beta_value, tax_rate);
    println!("Asset Beta: {:.2}", asset_beta_value);
}

fn valuation_menu() {
    clear_terminal();
    display_ascii_art();
    println!("\nValuation Models Menu:");
    println!("1. Gordon Growth Model - One Phase");
    println!("2. Gordon Growth Model - Two Phase");
    println!("--------------");
    println!("h. History");
    println!("b. Back to Main Menu");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let choice = get_user_input();
    match choice.trim() {
        "1" => run_ggm_p1(),
        "2" => run_ggm_p2(),
        "H" | "h" => display_history(),
        "b" => return,
        _ => println!("Invalid option, please try again."),
    }
}

fn run_ggm_p1() {
    println!("Enter initial cash flow (Cashflow_0): ");
    let cashflow_0 = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter required rate of return (as a decimal, e.g., 0.05 for 5%): ");
    let required_rate_of_return = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter growth rate (as a decimal, e.g., 0.02 for 2%): ");
    let growth_rate = get_user_input().trim().parse::<f64>().unwrap();

    match ggm_p1(cashflow_0, required_rate_of_return, growth_rate) {
        Some(value) => println!("Valuation using GGM One Phase: {:.2}", value),
        None => println!("Invalid input: Required rate of return must be greater than growth rate."),
    }
}

fn run_ggm_p2() {
    println!("Enter initial cash flow (Cashflow_0): ");
    let cashflow_0 = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter required rate of return (as a decimal, e.g., 0.05 for 5%): ");
    let required_rate_of_return = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter first phase growth rate (as a decimal, e.g., 0.03 for 3%): ");
    let growth_rate_1 = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter second phase growth rate (as a decimal, e.g., 0.02 for 2%): ");
    let growth_rate_2 = get_user_input().trim().parse::<f64>().unwrap();
    println!("Enter number of periods for first phase: ");
    let periods = get_user_input().trim().parse::<u32>().unwrap();

    match ggm_p2(cashflow_0, required_rate_of_return, growth_rate_1, growth_rate_2, periods) {
        Some(value) => println!("Valuation using GGM Two Phase: {:.2}", value),
        None => println!("Invalid input: Required rate of return must be greater than second phase growth rate."),
    }
}

// end of file
