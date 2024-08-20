fn main() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for day in 1..=12 {
        println!("On the {} day of Christmas, my true love gave to me:", ordinal(day));
        for i in (0..day).rev() {
            if day == 1 && i == 0 {
                println!("{}.", gifts[i]);
            } else {
                println!("{},", gifts[i]);
            }
        }
        println!(); // Add a blank line between days
    }
}

fn ordinal(day: usize) -> String {
    match day {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eighth".to_string(),
        9 => "ninth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelfth".to_string(),
        _ => unreachable!(),
    }
}
