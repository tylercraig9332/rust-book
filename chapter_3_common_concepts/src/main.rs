use rand::Rng;
const MY_CONST: u32 = 7;

fn main() {
    let mut my_mutable: u32 = 29;
    println!("My constant value is: {MY_CONST}");
    println!("My mutable value is: {my_mutable}");
    my_mutable += 13;
    println!("my mutable value now is {my_mutable}");
    println!("--- calling my shadowing example ---");
    shadowing();
    println!("--- calling my tuple example ---");
    destructure_tuple();
    println!("--- calling my array example ---");
    array_compound_type();
    println!("--- calling my expression example ---");
    let y = expression();
    println!("my function returned {y}");
    rand_escape();
}

fn shadowing() {
    let x = 2;
    let x = x + 1;

    {
        let x = x * 2;
        println!("the value of x in inner scope is {x}");
    }

    println!("the value of x in outer scope is {x}");
}

fn destructure_tuple() {
    let tup = (500, 2, 7);
    let (x, y, z) = tup;
    println!("the value of my tuple at y is {y}");
    let five_hundred = tup.0;
    println!("I can also call my tuple value by index {five_hundred}");
}

fn array_compound_type() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let five_items: [i32; 5] = [1, 2, 3, 4, 5];
    let three_five_times = [3; 5];
    let count = five_items.len();
    println!("{count} items in the following array:");
    // TODO: learn how to iterate so I can print these values
    for element in five_items {
        print!("{element} ");
    }
    println!("");
    for i in three_five_times {
        print!("{i} ")
    }
    println!("");
}

// Special Note: If you want to return the final value do not include a semicolon
fn expression() -> i32 {
    let y = {
        let x = 3;
        x + 2
    };

    println!("The value of y is: {y}");
    y
}

fn conditional_assignment() -> i32 {
    let condition = true;
    let number = if condition { 1 } else { 5 };
    number
}

fn rand_escape() {
    let mut counter = 0;
    let escape_num = rand::thread_rng().gen_range(0..20);
    println!("Will it escape before the counter hits 10!");
    // like I could just return if we are above or below, but this is fun
    let escaped_at = loop {
        if counter == escape_num || counter == 10 {
            break counter;
        };
        counter += 1;
    };
    if escaped_at == 10 {
        return println!("Womp womp! We didn't make it! {escape_num}");
    }
    println!("we escaped at {escaped_at}");
}
