use anyhow::anyhow;
use clap::Parser;
use clap::Subcommand;
use similar::DiffableStr;

/// Diff two http requests and compare the differences of the two requests.
#[derive(Parser, Debug, Clone)]
#[clap(versioon, author, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct RunArgs {
    /// Profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    #[clap(short, long, value_parser = parse_key_value, number_of_values = 1)]
    extra_params: Vec<KeyValue>,
}

#[derive(Debug, Clone)]
pub(crate) struct KeyValue {
    pub key: String,
    pub value: String,
}

fn parse_key_value(s: &str) -> Result<KeyValue> {
    let mut parts = s.splitn(2, '=');
    let retrieve = move || {
        parts
            .next()
            .ok_or_else(|| anyhow!("invalid key-value pair"))
    };

    let key = retrieve()?.trim();
    let value = retrieve()?.trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyType::Header, &key[1..]),
        Some('@') => (KeyType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyType::Query, key),
        _ => return Err(anyhow!("invalid key type")),
    };

    Ok(KeyValue {
        key: key.to_string(),
        value: value.to_string(),
    })
}

enum KeyType {
    Header,
    Body,
    Query,
}
