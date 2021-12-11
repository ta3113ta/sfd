use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use duct::cmd;
use std::fs;
use yaml_rust::{yaml::Yaml, YamlLoader};

pub fn collect_function_name(doc: &Yaml) -> Vec<String> {
    let mut functions = Vec::new();

    match &doc["functions"] {
        Yaml::Hash(hash) => {
            for (key, _) in hash {
                let function: String = key.as_str().unwrap().into();
                functions.push(function);
            }
        }
        _ => {}
    }

    functions
}

struct Config {
    function: String,
    stage: String,
}

fn exec(config: Config) -> Result<()> {
	println!("deploying {} to {}", config.function, config.stage);

    cmd!(
        "sls",
        "deploy",
        "function",
        "-f",
        &config.function,
        "-s",
        &config.stage
    )
    .run()?;

    Ok(())
}

pub fn run() -> Result<()> {
    let content = fs::read_to_string("serverless.yml").context("can not found serverless.yml")?;
    let docs = YamlLoader::load_from_str(&content).context("can not parse serverless.yml")?;
    let doc = &docs[0];

    let functions = collect_function_name(doc);
    let stages = &["dev", "prod"];

    let theme = ColorfulTheme::default();

    let function = Select::with_theme(&theme)
        .with_prompt("Select function")
        .items(&functions)
        .interact()
        .unwrap();

    let stage = Select::with_theme(&theme)
        .with_prompt("Select stage")
        .items(stages)
        .interact()
        .unwrap();

    let config = Config {
        function: functions[function].clone(),
        stage: stages[stage].to_string(),
    };

    exec(config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let contents = "
functions:
  paymentValidate:
    handler: src/controllers/paymentControllers.paymentValidate
  paymentUpdate:
    handler: src/controllers/paymentControllers.paymentUpdate
";

        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let doc = &docs[0];
        let expected = vec!["paymentValidate", "paymentUpdate"];
        let result = collect_function_name(doc);

        assert_eq!(result, expected);
    }
}
