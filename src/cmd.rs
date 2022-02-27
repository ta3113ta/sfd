use anyhow::{anyhow, Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use duct::cmd;
use std::fs;
use yaml_rust::{yaml::Yaml, YamlLoader};

use crate::opt::Opt;

fn collect_function_name(doc: &Yaml) -> Vec<String> {
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

fn deploy_all_stage() -> Result<()> {
    println!("deploying stages dev and prod");
    cmd!("sls", "deploy", "--stage", "dev",).run()?;
    cmd!("sls", "deploy", "--stage", "prod",).run()?;
    Ok(())
}

fn deploy_funciton(config: Config) -> Result<()> {
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

pub fn run(opt: Opt) -> Result<()> {
	let content = fs::read_to_string("serverless.yml").context("can not found serverless.yml")?;

    if opt.all {
        deploy_all_stage()?;
        return Ok(());
    }

    let docs = YamlLoader::load_from_str(&content).context("can not parse serverless.yml")?;
    let doc = &docs[0];

    let functions = collect_function_name(doc);
    let stages = &["dev", "prod"];

    if functions.is_empty() {
        return Err(anyhow!("can not found functions"));
    }

    let function = display("Select function".to_string(), &functions);
    let stage = display("Select stage".to_string(), stages);
    let config = Config { function, stage };

    deploy_funciton(config)?;

    Ok(())
}

fn display<T>(display_prompt: String, list: &[T]) -> String
where
    T: ToString,
{
    let theme = ColorfulTheme::default();
    let selected = Select::with_theme(&theme)
        .with_prompt(display_prompt)
        .items(list)
        .interact()
        .unwrap();

    list[selected].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_found_functions() {
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

    #[test]
    fn not_found_functions() {
        let contents = "
paymentValidate:
  - a1
paymentUpdate:
  - b2
";
        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let doc = &docs[0];
        let expected: Vec<String> = vec![];
        let result = collect_function_name(doc);

        assert_eq!(result, expected);
    }
}
