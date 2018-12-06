use std::io;

pub enum Target {
    Insert,
    Query,
    Delete,
    Update,
    ShowMenu,
    Exit
}

pub fn helper() -> Target {
    loop {
        let mut action = String::new();
        io::stdin().read_line(&mut action)
            .expect("Failed to read line");
        let action: u8 = match action.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Please Enter a number between 0 to 4 ");
                continue
            },
        };
        let action: Target = match action {
            0 => Target::Exit,
            1 => Target::Insert,
            2 => Target::Query,
            3 => Target::Delete,
            4 => Target::Update,
            _ => Target::ShowMenu,
        };
        break action
    }
}

pub fn showmenu() -> bool {
    println!("1. insert a student's information.");
    println!("2. query the students' information.");
    println!("3. delete a student's information by id.");
    println!("4. update a student's informatjion by id.");
    println!("0. exit the paltform.");
    true
}
