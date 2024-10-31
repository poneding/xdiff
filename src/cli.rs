use anyhow::anyhow;
use anyhow::Error;
use clap::Parser;
use clap::Subcommand;

use crate::ExtraArgs;

/// Diff two http requests and compare the differences of the two requests.
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// Profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    #[clap(short, long, value_parser = parse_key_value, number_of_values = 1)]
    pub extra_params: Vec<KeyValue>,

    /// Configuration to use.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
    pub key_type: KeyValueType,
}

fn parse_key_value(s: &str) -> Result<KeyValue, Error> {
    let mut parts: std::str::SplitN<'_, char> = s.splitn(2, '=');
    let mut retrieve = move || {
        parts
            .next()
            .ok_or_else(|| anyhow!("invalid key-value pair"))
    };

    let key = retrieve()?.trim();
    let value = retrieve()?.trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValueType::Header, &key[1..]),
        Some('@') => (KeyValueType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValueType::Query, key),
        _ => return Err(anyhow!("invalid key type")),
    };

    Ok(KeyValue {
        key: key.to_string(),
        key_type,
        value: value.to_string(),
    })
}

#[derive(Debug, Clone)]
pub enum KeyValueType {
    /// if key starts with '#', it will be treated as header
    Header,
    /// if key starts with '@', it wiil be treated as query
    Query,
    /// if key has no any prefix, it will be treated as body
    Body,
}

impl From<Vec<KeyValue>> for ExtraArgs {
    fn from(args: Vec<KeyValue>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];
        for arg in args {
            match arg.key_type {
                KeyValueType::Header => headers.push((arg.key, arg.value)),
                KeyValueType::Query => query.push((arg.key, arg.value)),
                KeyValueType::Body => body.push((arg.key, arg.value)),
            }
        }

        ExtraArgs {
            headers,
            query,
            body,
        }
    }
}
