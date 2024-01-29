use clap::{Command, Arg};

pub fn parse_commands() -> String {
    let matches = Command::new("abook")
        .author("Thejana Rajapakshe, thejanarajapakshe1@gmail.com")
        .version("1.0")
        .about("a programm that let you download almost any book for free!")

        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Name on the book you want to download")
                .required(true)
        )

        .arg(
            Arg::new("amount")
                .short('a',)
                .long("amount")
                .help("specify the number of books you need to fetch NOTE: this could only be 25, 50, 75 or 100")
        )

        .arg(
            Arg::new("author")
                .long("author")
                .help("specify the author")
        )

        .get_matches();

    if let Some(c) = matches.get_one::<String>("name") {
        return c.to_string();
    } else {
        return "".to_string();
    }
}