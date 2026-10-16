## rustpresso
A CLI tool written in Rust to manage cafe cashier orders.
(This project has a fan aspect and is mostly for sample work)

## Build
```sh
cargo build --release
```
The binary is placed at target/release/rustpresso

## How to use
This tool is in the form of a REPL and all events happen in the program itself. The program does not receive arguments (currently)
```sh
rustpresso
```
### commands:
|command|discription|
|---------|-----------|
|help, ?|shows all the commands|
|new_order|creates an order|
|list|lists all the orders|
|stat_nx|changes order status to next level of status|
|stat_pr|changes order status to previous level of status|
|remove|removes an order with its ID|
|clear|clears the terminal|
|find|finds an order with its ID|
|change|changes the info of an order|
|quit, exit|exit with ExitCode 0|

## Sample 
```
-=Welcome to Cafe order system=-

help or ? to see commands

>> help

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

>> new_order

Creating new order

Coffee name
: Latte
Size of the coffee (small/medium/large)
: medium
Milk (none/whole/oat)
: whole
Have discount? (yes/no)
: yes
Which type? (percent/fixed)
: percent
How many percent (NUMBER)
: 25
>> list
0 : Latte (Medium, Whole milk) - 57 cents (Percent(25) discount) --> Pending
>> stat_nx
Order ID
: 0
The status changed from Pending To Preparing

>> list
0 : Latte (Medium, Whole milk) - 57 cents (Percent(25) discount) --> Preparing
>>
```
## Project structure
```
src/
├── commands.rs
├── lib.rs
└── main.rs
```

## Tests
```sh
cargo test
```
Should pass 21 tests


("without using AI B)")