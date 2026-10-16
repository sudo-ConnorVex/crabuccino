use rustpresso::commands::{
    ch_stat_nx, ch_stat_pr, change, clear, find_coffee, help_message, list_orders, new_order,
    remove_order,
};
use std::io::{self, Write};
use std::process::exit;

fn main() {
    clear();
    println!("\n-=Welcome to Cafe order system=-\n");
    println!("help or ? to see commands\n");

    let mut orders = Vec::new();
    let mut order_ids = 0;

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error: failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot get user input");

        match input.as_str().to_lowercase().trim() {
            "help" | "?" => help_message(),
            "new_order" => {
                orders.push(new_order(order_ids));
                order_ids += 1;
            }
            "list" => list_orders(&orders),
            "remove" => remove_order(&mut orders),
            "clear" => clear(),
            "stat_nx" => ch_stat_nx(&mut orders),
            "stat_pr" => ch_stat_pr(&mut orders),
            "find" => find_coffee(&orders),
            "change" => change(&mut orders),
            "quit" | "exit" => {
                println!("Ok bye :)");
                exit(0)
            }
            _ => println!("Invalid command\n"),
        }
    }
}
