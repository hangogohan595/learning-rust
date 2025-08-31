fn main() {
    const GIFTS: [&str; 12] = [
        "A Partridge in a Pear Tree",
        "Turtle Doves",
        "French Hens",
        "Calling Birds",
        "Gold Rings",
        "Geese a-Laying",
        "Swans a-Swimming",
        "Maids a-Milking",
        "Ladies Dancing",
        "Lords a-Leaping",
        "Pipers Piping",
        "Drummers Drumming",
    ];

    for (i, gift) in GIFTS.iter().enumerate() {
        println!(
            "On the {}{} day of Christmas, my true love sent to me\n{}{gift}",
            i + 1,
            {
                if i == 0 {
                    "st"
                } else if i == 1 {
                    "nd"
                } else if i == 2 {
                    "rd"
                } else {
                    "th"
                }
            },
            {
                match i {
                    0 => String::new(),
                    n => (n + 1).to_string() + " ",
                }
            }
        )
    }
}
