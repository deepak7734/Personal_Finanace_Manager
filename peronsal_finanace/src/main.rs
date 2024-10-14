use chrono::NaiveDate;
use std::io;

#[derive(Debug, PartialEq)]
enum TransactionType {
    Income,
    Expense,
}

#[derive(Debug)]
struct Transaction {
    date: NaiveDate,
    type_: TransactionType,
    amount: f64,
    category: String,
}

impl Transaction {
    fn new(date: NaiveDate, type_: TransactionType, amount: f64, category: String) -> Self {
        Transaction {
            date,
            type_,
            amount,
            category,
        }
    }
}

fn add_transaction(transactions: &mut Vec<Transaction>) {
    println!("Enter date (YYYY-MM-DD):");
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read input");
    let date = NaiveDate::parse_from_str(&date.trim(), "%Y-%m-%d").expect("Invalid date");

    println!("Enter transaction type (income/expense):");
    let mut type_ = String::new();
    io::stdin().read_line(&mut type_).expect("Failed to read input");
    let type_ = match type_.trim().to_lowercase().as_str() {
        "income" => TransactionType::Income,
        "expense" => TransactionType::Expense,
        _ => {
            println!("Invalid transaction type");
            return;
        }
    };

    println!("Enter amount:");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read input");
    let amount = amount.trim().parse::<f64>().expect("Invalid amount");

    println!("Enter category:");
    let mut category = String::new();
    io::stdin().read_line(&mut category).expect("Failed to read input");

    let transaction = Transaction::new(date, type_, amount, category.trim().to_string());
    transactions.push(transaction);
}

fn view_transactions(transactions: &Vec<Transaction>) {
    for transaction in transactions {
        println!("Date: {}", transaction.date);
        println!("Type: {:?}", transaction.type_);
        println!("Amount: {:.2}", transaction.amount);
        println!("Category: {}", transaction.category);
        println!();
    }
}

fn summary(transactions: &Vec<Transaction>) {
    let total_income: f64 = transactions.iter()
        .filter(|t| t.type_ == TransactionType::Income)
        .map(|t| t.amount)
        .sum();

    let total_expense: f64 = transactions.iter()
        .filter(|t| t.type_ == TransactionType::Expense)
        .map(|t| t.amount)
        .sum();

    let balance = total_income - total_expense;

    println!("Total Income: {:.2}", total_income);
    println!("Total Expense: {:.2}", total_expense);
    println!("Balance: {:.2}", balance);
}

fn filter_transactions(transactions: &Vec<Transaction>) {
    println!("Enter filter criteria (date/category):");
    let mut criteria = String::new();
    io::stdin().read_line(&mut criteria).expect("Failed to read input");

    match criteria.trim() {
        "date" => {
            println!("Enter date (YYYY-MM-DD):");
            let mut date = String::new();
            io::stdin().read_line(&mut date).expect("Failed to read input");
            let date = NaiveDate::parse_from_str(&date.trim(), "%Y-%m-%d").expect("Invalid date");

            let filtered_transactions: Vec<_> = transactions.iter()
                .filter(|t| t.date == date)
                .collect();

            for transaction in filtered_transactions {
                println!("Date: {}", transaction.date);
                println!("Type: {:?}", transaction.type_);
                println!("Amount: {:.2}", transaction.amount);
                println!("Category: {}", transaction.category);
                println!();
            }
        }
        "category" => {
            println!("Enter category:");
            let mut category = String::new();
            io::stdin().read_line(&mut category).expect("Failed to read input");

            let filtered_transactions: Vec<_> = transactions.iter()
                .filter(|t| t.category == category.trim())
                .collect();

            for transaction in filtered_transactions {
                println!("Date: {}", transaction.date);
                println!("Type: {:?}", transaction.type_);
                println!("Amount: {:.2}", transaction.amount);
                println!("Category: {}", transaction.category);
                println!();
            }
        }
        _ => println!("Invalid filter criteria."),
    }
}

fn main() {
    println!("Personal Finance Manager");

    let mut transactions: Vec<Transaction> = Vec::new();

    loop {
        println!("1. Add Transaction");
        println!("2. View Transactions");
        println!("3. Summary");
        println!("4. Filter Transactions");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => add_transaction(&mut transactions),
            "2" => view_transactions(&transactions),
            "3" => summary(&transactions),
            "4" => filter_transactions(&transactions),
            "5" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
