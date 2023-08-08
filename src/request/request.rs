use reqwest::{Client, Response}; 

pub struct Request<'a> {
    pub url: &'a String
}
impl<'a> Request<'a> {
    pub fn new(url: &'a String) -> Self {
        Self {
            url
        }
    }
    pub async fn get(&self) -> Result<Response, reqwest::Error> {
        Client::new()
            .get(self.url)
            .send()
            .await
    }
    pub async fn get_text(&self) -> Result<String, reqwest::Error>{
        self.get()
            .await?
            .text()
            .await
    }
    // There was something wrong with the code.
    //
    // pub async fn get_json<T>(&'a self) -> Result<T, Box<dyn Error>>
    // where 
    //     T: Deserialize<'a>
    // {
    //     let str = self.get_text().await?;
    //     let p_str = str.as_str();
    //     Ok(serde_json::from_str(p_str)?)
    // }
}
