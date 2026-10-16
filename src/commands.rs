use crate::{Coffee, Discount, Milk, Size, Status};
use std::{
    io::{self, Write}
};

// ----------------------helper functions----------------------//
fn get_id() -> u16 {
    loop {
        match user_input("Order ID").trim().parse::<u16>() {
            Ok(i) => return i,
            Err(e) => {
                eprintln!("Error: failed to parse id to u16: {e:?}");
                continue;
            }
        }
    }
}

fn user_input(answer: &str) -> String {
    loop {
        println!("{answer}");
        print!(": ");
        io::stdout().flush().expect("failed to flush stdout");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => return input,
            Err(e) => {
                println!("invalid input: {e:?}");
                continue;
            }
        }
    }
}

fn get_discount() -> Discount {
    loop {
        match user_input("Have discount? (yes/no)").to_lowercase().trim() {
            "no" | "none" => return Discount::None,
            "yes" => loop {
                match user_input("Which type? (percent/fixed)").to_lowercase().trim() {
                    "percent" => loop {
                        match user_input("How many percent (NUMBER)").trim().parse::<u32>() {
                            Ok(n) => return Discount::Percent(n),
                            Err(_) => {
                                eprintln!("Please type a valid value.");
                                continue;
                            }
                        }
                    },
                    "fixed"   => loop {
                        match user_input("How much (NUMBER)").trim().parse::<u32>() {
                            Ok(n) => return Discount::Fixed(n),
                            Err(_) => {
                                eprintln!("Please type a valid value.");
                                continue;
                            }
                        }
                    },
                    _ => {
                        eprintln!("Please type 'percent' or 'fixed'.");
                        continue;
                    }
                }
            },
            _  => {
                eprintln!("Please type 'no' or 'yes'.");
                continue;
            }
        }
    }
}

fn get_size() -> Size {
    loop {
        match user_input("Size of the coffee (small/medium/large)")
            .to_lowercase()
            .trim()
        {
            "small" => break Size::Small,
            "medium" => break Size::Medium,
            "large" => break Size::Large,
            _ => {
                eprintln!("Please type 'small', 'medium' or 'large'.");
                continue;
            }
        }
    }
} 

fn get_milk() -> Milk {
    loop {
        match user_input("Milk (none/whole/oat)")
            .to_lowercase()
            .trim()
        {
            "none" => break Milk::None,
            "whole" => break Milk::Whole,
            "oat" => break Milk::Oat,
            _ => {
                eprintln!("Please type 'none', 'whole' or 'oat'.");
                continue;
            }
        }
    }
}

fn get_status() -> Status {
    loop {
        match user_input("New status (pending/preparing/ready/delivered)")
            .trim()
            .to_lowercase()
            .as_str()
        {
            "pending" => return Status::Pending,
            "preparing" => return Status::Preparing,
            "ready" => return Status::Ready,
            "delivered" => return Status::Delivered,
            _ => {
                eprintln!("Please type 'pending', 'preparing', 'ready' or 'delivered'.");
                continue;
            }
        }
    }
}

// ----------------------finding coffees in a list----------------------//
pub fn find_coffee(orders: &[Coffee]) {
    let id = get_id();

    for item in orders {
        if item.id == id {
            println!(
                "{} : {} ({:?}, {:?} milk) - {} cents ({:?} discount) --> {:?}",
                item.id,
                item.name,
                item.size,
                item.milk,
                item.price_calculate(),
                item.discount,
                item.status
            );
            return;
        }
    }
}

// ----------------------clear terminal----------------------//
pub fn clear() {
    print!("\x1b[2J\x1b[3J\x1b[H");
    io::stdout().flush().expect("failed to flush stdout");
}

// ----------------------help message----------------------//
pub fn help_message() {
    println!(
        "
new_order   : create a new order
help | ?    : shows this message
list        : lists all orders
stat_nx  : changes a status to next level
stat_pr  : changes a status to prev level
remove      : removes an order with order ID
clear       : clear the terminal
find        : find orders with id
change      :
quit | exit : exit with ExitCode 0
 "
    )
}

// ----------------------remove order from orders----------------------//
pub fn remove_order(orders: &mut Vec<Coffee>) {
    let target_id = get_id();

    if let Some(i) = orders.iter().position(|item| item.id == target_id) {
        orders.remove(i);
        println!("Removed: {target_id}\n");
    }
}

// ----------------------new order----------------------//
pub fn new_order(id: u16) -> Coffee {
    println!("\nCreating new order\n");

    let name = user_input("Coffee name").trim().to_string();
    let size = get_size();
    let milk = get_milk();
    let discount = get_discount();

    Coffee {
        id,
        name,
        size,
        milk,
        status: Status::Pending,
        discount,
    }
}

// ----------------------list all orders----------------------//
pub fn list_orders(orders: &[Coffee]) {
    for item in orders {
        println!(
            "{} : {} ({:?}, {:?} milk) - {} cents ({:?} discount) --> {:?}",
            item.id,
            item.name,
            item.size,
            item.milk,
            item.price_calculate(),
            item.discount,
            item.status
        )
    }
}

// ----------------------next or prev order status----------------------//
pub fn ch_stat_nx(orders: &mut Vec<Coffee>) {
    let id = get_id();

    for item in orders {
        if item.id == id {
            print!("The status changed from {:?} ", item.status);
            item.advance_status();
            println!("To {:?}\n", item.status);
            io::stdout().flush().expect("Error: failed to flush stdout");
            return;
        }
    }
    println!("No order was found with this ID: {id}");
}

pub fn ch_stat_pr(orders: &mut Vec<Coffee>) {
    let id = get_id();

    for item in orders {
        if item.id == id {
            print!("The status changed from {:?} ", item.status);
            item.retreat_status();
            println!("To {:?}\n", item.status);
            io::stdout().flush().expect("Error: failed to flush stdout");
            return;
        }
    }
    println!("No order was found with this ID: {id}");
}

// ----------------------next or prev order status----------------------//
pub fn change(orders: &mut Vec<Coffee>) {
    let id = get_id();

    for item in orders {
        if item.id == id {
            let target_field = user_input("Which info? (name/size/milk/status/discount)");
            match target_field.trim().to_lowercase().as_str() {
                "name" => item.name = user_input("New name").trim().to_string(),
                "size" => item.size = get_size(),
                "milk" => item.milk = get_milk(),
                "status" => item.status = get_status(),
                "discount" => item.discount = get_discount(),
                _ => eprintln!("Unknown field: {target_field}"),
            }
            return;
        }
    }
    println!("No order was found with this ID: {id}");
}
