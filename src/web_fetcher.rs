use reqwest::blocking;

pub struct Info {
    client: blocking::Client,
    base_url: String,
    params: String
}

impl Info{
    pub fn new() -> Self{
        let client = blocking::Client::new();
        Info{
            client, 
            base_url: "http://libgen.is/search.php?req=".to_string(),
            params: "&open=0&res=10&view=simple&phrase=1&column=def".to_string()
        }
    }

    pub fn fetch(&self, book_name: &str) -> Result<String, reqwest::Error>{
        let book_name = book_name.chars()
            .map(|c| if c == ' ' {'+'} else {c})
            .collect::<String>();
        let url = format!("{}{}{}", self.base_url, book_name, self.params);
        let body = self.client.get(url).send()?.text()?;
        
        Ok(body)
    }
}
