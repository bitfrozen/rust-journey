fn main() {
    for i in 1..=12 {
        print_carol_day(i);
        println!();
    }
}

fn print_carol_day(n: u8) {
    println!("On the {} day of Christmas, my true love sent to me", get_ordinal_word_from_number(n));

    for day in (1..=n).rev() {
        let mut prefix = "";
        if n > 1 && day == 1 {
            prefix = "and ";
        }
        let day_text = format!("{}{}", prefix, get_carol_stuff_for_day(day));
        println!("{}", capitalize_first_letter(&day_text));
    }
}

fn capitalize_first_letter(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn get_ordinal_word_from_number(n: u8) -> &'static str {
    match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "unknown",
    }
}

fn get_carol_stuff_for_day(n: u8) -> &'static str {
    match n {
        1 => "a partridge in a pear tree.",
        2 => "two turtle doves,",
        3 => "three French hens,",
        4 => "four calling birds,",
        5 => "five golden rings,",
        6 => "six geese a-laying,",
        7 => "seven swans a-swimming,",
        8 => "eight maids a-milking,",
        9 => "nine drummers drumming,",
        10 => "ten pipers piping,",
        11 => "eleven ladies dancing,",
        12 => "twelve lords a-leaping,",
        _ => "unknown",
    }
}