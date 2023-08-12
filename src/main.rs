fn main() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let items: [&str; 12] = [
        "A patridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!("The Twelve Days of Christmas");

    for verse_number in 1..13 {
        println!("\n{}", print_verse(verse_number));
        println!("{}", print_day_message(days[verse_number - 1]));

        for day in (1..verse_number + 1).rev() {
            println!("{}", items[day - 1]);
        }
    }
}

fn print_verse(verse_number: usize) -> String {
    return format!("[VERSE {verse_number}]");
}

fn print_day_message(day: &str) -> String {
    return format!("On the {day} day of Christmas, my true love sent to me");
}
