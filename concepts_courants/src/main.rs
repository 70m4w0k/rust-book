fn main() {
    // Convertir des températures entre les degrés Fahrenheit et Celsius.

    let fahrenheit1 = 212.0;

    let celsius1 = from_fahrenheit_to_celsius(fahrenheit1);

    println!("{} °F = {} °C", fahrenheit1, celsius1);

    let celsius2 = 100.0;

    let fahrenheit2 = from_celsius_to_fahrenheit(celsius2);

    println!("{} °C = {} °F", celsius2, fahrenheit2);

    // Générer le n-ième nombre de Fibonacci.
    let target = 5;
    let nieme = fibonacci(target);
    println!("{}-ieme nombre de fibonacci : {}", target, nieme);

    // Afficher les paroles de la chanson de Noël The Twelve Days of Christmas en profitant de l'aspect répétitif de la chanson.
    twelve_days_of_christmas();
}

fn from_fahrenheit_to_celsius(temperature: f32) -> f32 {
    (temperature - 32.0) / 1.8000
}

fn from_celsius_to_fahrenheit(temperature: f32) -> f32 {
    (temperature * 1.8000 + 32.0)
}

fn fibonacci(target: i32) -> i32 {
    let mut previous = 0;
    let mut current = 1;
    let mut result = 0;
    for _i in 0..target {
        result = current + previous;
        previous = current;
        current = result;
    }
    result
}

fn twelve_days_of_christmas() {
    let first_verse_phrase = String::from("On the {} day of Christmas, my true love sent to me");
    let last_verse_phrase = "A partridge in a pear tree";

    let second_verse_phrase = String::from("");

    let mut song = String::from("").to_owned();

    for i in 1..13 {
        let mut number = String::from("");
        match i {
            1 => {
                number = String::from("first");
            }
            2 => {
                number = String::from("second");
            }
            3 => {
                number = String::from("third");
            }
            4 => {
                number = String::from("fourth");
            }
            5 => {
                number = String::from("fifth");
            }
            6 => {
                number = String::from("sixth");
            }
            7 => {
                number = String::from("seventh");
            }
            8 => {
                number = String::from("eighth");
            }
            9 => {
                number = String::from("ninth");
            }
            10 => {
                number = String::from("tenth");
            }
            11 => {
                number = String::from("eleventh");
            }
            12 => {
                number = String::from("twelfth");
            }
            _ => {}
        }
        song.push_str("\nOn the ");
        song.push_str(&number.to_string());
        song.push_str(" day of Christmas, my true love sent to me");

        if i > 11 {
            song.push_str("\nTwelve drummers drumming");
        }
        if i > 10 {
            song.push_str("\nEleven pipers piping");
        }
        if i > 9 {
            song.push_str("\nTen lords a-leaping");
        }
        if i > 8 {
            song.push_str("\nNine ladies dancing");
        }
        if i > 7 {
            song.push_str("\nEight maids a-milking");
        }
        if i > 6 {
            song.push_str("\nSeven swans a-swimming");
        }
        if i > 5 {
            song.push_str("\nSix geese a-laying");
        }
        if i > 4 {
            song.push_str("\nFive golden rings");
        }
        if i > 3 {
            song.push_str("\nFour calling birds");
        }
        if i > 2 {
            song.push_str("\nThree french hens");
        }
        if i > 1 {
            song.push_str("\nTwo turtle doves, and");
        }
        song.push_str("\nA partridge in a pear tree\n");
    }

    println!("{}", song);

    // println!(
    //     "
    //     On the first day of Christmas, my true love sent to me
    //     A partridge in a pear tree

    //     On the second day of Christmas, my true love sent to me
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the third day of Christmas, my true love sent to me
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the fourth day of Christmas, my true love sent to me
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the fifth day of Christmas, my true love sent to me
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the sixth day of Christmas, my true love sent to me
    //     Six geese a-laying
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the seventh day of Christmas, my true love sent to me
    //     Seven swans a-swimming
    //     Six geese a-laying
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree
    //     You might also like
    //     Did you know that there’s a tunnel under Ocean Blvd
    //     Lana Del Rey
    //     All I Want for Christmas Is You
    //     Mariah Carey
    //     This Is Why
    //     Paramore

    //     On the eighth day of Christmas, my true love sent to me
    //     Eight maids a-milking
    //     Seven swans a-swimming
    //     Six geese a-laying
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the ninth day of Christmas, my true love sent to me
    //     Nine ladies dancing
    //     Eight maids a-milking
    //     Seven swans a-swimming
    //     Six geese a-laying
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the tenth day of Christmas, my true love sent to me
    //     Ten lords a-leaping
    //     Nine ladies dancing
    //     Eight maids a-milking
    //     Seven swans a-swimming
    //     Six geese a-laying
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the eleventh day of Christmas, my true love sent to me
    //     Eleven pipers piping
    //     Ten lords a-leaping
    //     Nine ladies dancing
    //     Eight maids a-milking
    //     Seven swans a-swimming
    //     Six geese a-laying
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree

    //     On the twelfth day of Christmas, my true love sent to me
    //     Twelve drummers drumming
    //     Eleven pipers piping
    //     Ten lords a-leaping
    //     Nine ladies dancing
    //     Eight maids a-milking
    //     Seven swans a-swimming
    //     Six geese a-laying
    //     Five golden rings
    //     Four calling birds
    //     Three french hens
    //     Two turtle doves, and
    //     A partridge in a pear tree"
    // );
}
