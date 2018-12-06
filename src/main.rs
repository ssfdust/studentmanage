mod help;
mod handler;

use crate::handler::{
    insert,
    delete,
    update,
    query
};
use crate::help::{Target, showmenu};

fn main() {
    println!("Welcome to student management platform.");
    println!("Please enter the choice number:");
    showmenu();
    loop {
        let action: help::Target = help::helper();
        match action {
            Target::Exit => {
                println!("GoodBye~");
                break;
            },
            Target::ShowMenu => showmenu(),
            Target::Insert => insert(),
            Target::Query => query(),
            Target::Delete => delete(),
            Target::Update => update(),
        };
    }
}
