fn main() {
    const GIFTS: [&str; 12] = [
        "A Partridge in a Pear Tree",
        "Turtle Doves and",
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

    for i in 0..12 {
        println!(
            "On the {}{} day of Christmas, my true love sent to me\n{}",
            { i + 1 },
            {
                match i {
                    0 => "st",
                    1 => "nd",
                    2 => "rd",
                    _ => "th",
                }
            },
            {
                let mut current_gifts = String::new();

                for j in (0..(i + 1)).rev() {
                    if j == 0 {
                        current_gifts = current_gifts + GIFTS[j] + "\n"
                    } else {
                        current_gifts = format!("{}{} {}\n", current_gifts, j + 1, GIFTS[j]);
                    }
                }

                current_gifts
            }
        )
    }
}
