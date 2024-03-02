//use fancy::printcol;
use reqwest::Client;
use std::error::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;
// use dialoguer::{theme::ColorfulTheme, Input};
use serde_json::json;
use spinners::{Spinner, Spinners};
use clearscreen;
use fancy::printcoln;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Risposta {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub index: i64,
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMaConfig {
    pub messages: Vec<Message>,
    pub temperature: f64,
    #[serde(rename = "max_tokens")]
    pub max_tokens: i64,
    pub stream: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    clearscreen::clear().expect("failed to clear screen");
    printcoln!("[green]LLMA-Demo v1.0");
    printcoln!("[red]Prompt : Sei un esperto di lingua italiana.");
    
    // let quest: String = Input::with_theme(&ColorfulTheme::default())
    //              .with_prompt("Domanda:")
    //              .interact_text()
    //              .unwrap();

 
    let body = json!({
        "messages": [ 
           { "role": "system", "content": "sei un esperto di lingua italiana" },
           { "role": "user", "content": "componi un tema come argomento l'unità d'italia" }
         ], 
         "temperature": 0.7, 
         "max_tokens": -1,
         "stream": false
       });
       
       
    
    println!("{}", body);

    let mut sp = Spinner::new(Spinners::Dots2, "Elaborazione...".into());   
    let request_url = "http://localhost:1234/v1/chat/completions";                             
    let response: Risposta = Client::new()
        .post(request_url)         
        .json(&body)                
        .send()
        .await?
        .json()
        .await?;       
    sp.stop();
    
    let con =response.choices[0].message.content.to_string().replace('\n',"").replace("\"", "");
    println!("");
    printcoln!("[cyan]{:#?}",con);
    println!("");
    println!("");

    
    
    Ok(())
}