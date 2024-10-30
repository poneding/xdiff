use xdiff::DiffConfig;

fn main() -> anyhow::Result<()> {
    let content: &str = include_str!("../fixtures/config.yaml");
    let config = DiffConfig::from_yaml(content)?;

    println!("{:#?}", config);
    Ok(())
}
