use crate::web_fetcher::Info;
use crate::html_parser::Parser;


mod web_fetcher;
mod logger;
mod html_parser;
mod command_parser;
mod book;

fn main() {
    let book_name = command_parser::parse_commands();

    let fetcher = Info::new();
    let data = fetcher.fetch(&book_name).unwrap();
    let parser = Parser::new(data);
    parser.get_books();
}