use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
}

const BASE_URL: &str = "https://api.ce-cotoha.com/api/dev";

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    // #[serde(deserialize_with = "from_str")]
    expires_in: String,
    scope: String,
    issued_at: String,
}

pub fn get_access_token() -> Result<String, reqwest::Error> {
    let access_token_publish_url = "https://api.ce-cotoha.com/v1/oauth/accesstokens";

    let response: TokenResponse = reqwest::blocking::Client::new()
        .post(access_token_publish_url)
        .json(&TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: env::var("COTOHA_CLIENT_ID").unwrap(),
            client_secret: env::var("COTOHA_CLIENT_SECRET").unwrap(),
        })
        .send()?
        .json()?;

    return Ok(response.access_token);
}

pub enum Modality {
    Declarative,
    Interrogative,
    Imperative,
}
#[derive(Serialize, Debug)]
struct SentenceTypeRequest {
    sentence: String,
    r#type: String,
}
#[derive(Deserialize, Debug)]
struct SentenceTypeResponse {
    result: SentenceType,
    status: i32,
    message: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct SentenceType {
    pub modality: String,
    pub dialog_act: Vec<String>,
}

pub fn get_sentence_type(token: &String, text: &String) -> Result<SentenceType, reqwest::Error> {
    let response: SentenceTypeResponse = reqwest::blocking::Client::new()
        .post(&format!("{}/nlp/v1/sentence_type", BASE_URL))
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&SentenceTypeRequest {
            sentence: text.to_string(),
            r#type: "kuzure".to_string(),
        })
        .send()?
        .json()?;

    return Ok(response.result);
}

#[derive(Serialize, Debug)]
struct ParseRequest {
    sentence: String,
    r#type: String,
}
#[derive(Deserialize, Debug)]
struct ParseResponse {
    result: Vec<ParseObject>,
    status: i32,
    message: String,
}
#[derive(Deserialize, Debug)]
pub struct ParseObject {
    pub chunk_info: Chunk,
    pub tokens: Vec<Token>,
}
#[derive(Deserialize, Debug)]
pub struct Chunk {
    pub id: i32,
    pub head: i32,
    pub dep: String,
    pub chunk_head: i32,
    pub chunk_func: i32,
    pub links: Vec<Link>,
    pub predicate: Option<Vec<String>>,
}
#[derive(Deserialize, Debug)]
pub struct Link {
    pub link: i32,
    pub label: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Token {
    pub id: i32,
    pub form: String,
    pub kana: String,
    pub lemma: String,
    pub pos: String,
    pub features: Option<Vec<String>>,
    pub dependency_labels: Option<Vec<Dependency>>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Dependency {
    pub token_id: i32,
    pub label: String,
}

pub fn parse(
    token: &String,
    text: &String,
) -> Result<(Vec<ParseObject>, Vec<Token>), reqwest::Error> {
    let response: ParseResponse = reqwest::blocking::Client::new()
        .post(&format!("{}/nlp/v1/parse", BASE_URL))
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&ParseRequest {
            sentence: text.to_string(),
            r#type: "kuzure".to_string(),
        })
        .send()?
        .json()?;

    let tokens = response
        .result
        .iter()
        .flat_map(|ch| ch.tokens.iter())
        .cloned()
        .collect::<Vec<Token>>();

    return Ok((response.result, tokens));
}

pub fn imiaru(pos: &str) -> bool {
    return match pos {
        "名詞" => true,
        "名詞接尾辞" => true,
        "冠名詞" => true,
        "英語接尾辞" => true,
        "動詞語幹" => true,
        "動詞活用語尾" => true,
        "動詞接尾辞" => true,
        "冠動詞" => true,
        "補助名詞" => true,
        "形容詞語幹" => true,
        "形容詞接尾辞" => true,
        "冠形容詞" => true,
        "連体詞" => true,
        "連用詞" => true,
        "独立詞" => true,
        "Number" => true,
        "助数詞" => true,
        "冠数詞" => true,

        "接続詞" => false,
        "接続接尾辞" => false,
        "判定詞" => false,
        "格助詞" => false,
        "引用助詞" => false,
        "連用助詞" => false,
        "終助詞" => false,
        "間投詞" => false,
        "括弧" => false,
        "句点" => false,
        "読点" => false,
        "空白" => false,
        "Symbol" => false,
        "助助数詞" => false,
        _ => false,
    };
}
