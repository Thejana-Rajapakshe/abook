use scraper::{self};
use crate::book::Book;

pub struct Parser {
    document: scraper::Html,
    books: Vec<Book>
}


impl Parser{
    pub fn new(html: String) -> Self{
        let document = scraper::Html::parse_document(&html);
        Parser { document, books: vec![] }
    }

    pub fn get_books(mut self){
        let tr_selector = scraper::Selector::parse("tr[valign]").unwrap();
        let td_selector = scraper::Selector::parse("td").unwrap();
        
        for (line_number, tr) in self.document.select(&tr_selector).enumerate(){
            if line_number == 0 {
                continue;
            }
            
            print!("{}", line_number);

            let mut properties = Vec::new();
            
            for property in tr.select(&td_selector){
                print!("{}", '\t');
                if let Some(text) = property.text().next() {
                    properties.push(text.trim());   
                }else{
                    properties.push("NONE");
                }
            }

            self.books.push(
                Book::new(
                    String::from(properties[2]), 
                    String::from(properties[1]), 
                    String::from(properties[5]), 
                    String::from(properties[8]), 
                    String::from(properties[3]), 
                    String::from(properties[4]), 
                    String::from(properties[6])
                )
            );

            properties.clear();

            println!();
        }

        for (line_number, book) in self.books.iter().enumerate(){
            println!("----------BOOK {}----------", (line_number+1));
            book.print();
            println!();
        } 
    }

}