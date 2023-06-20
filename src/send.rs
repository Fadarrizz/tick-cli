use reqwest::{blocking::Client, header::USER_AGENT};

const BASE_URL: &str = "https://secure.tickspot.com";
const ORG_ID: &str = "45669";
const TOKEN: &str = "d115b93153b4f9968214865e96ff7789";

 fn send_entry(http_client: &Client, entry: Entry) -> Result<Response, Error> {
    let http_client = Client::new();

     http_client
         .post(format!("{}/{}/api/v2/entries.json", BASE_URL, ORG_ID))
         .json(&entry.json())
         .bearer_auth(TOKEN)
         .header(USER_AGENT, "tick-cli (auke@ijsfontein.nl)")
         .send()
 }
