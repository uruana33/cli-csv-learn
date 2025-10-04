use crate::OutputFormat;
use anyhow::Result;
use serde_json::Value;
use std::fs;

fn read_csv(path: &str) -> Vec<Value> {
    let mut result = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(path)
        .unwrap();

    let header = result.headers().unwrap().clone();

    let mut content = vec![];
    for result in result.records() {
        let record = result.unwrap();
        let ret = header.iter().zip(record.iter());
        let ret = ret.collect::<Value>();
        content.push(ret);
    }
    content
}

fn write_json(content: &Vec<Value>, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(content)?;
    fs::write(path, json)?;
    Ok(())
}

fn write_yaml(content: &Vec<Value>, path: &str) -> Result<()> {
    let yaml = serde_yaml::to_string(content)?;
    fs::write(path, yaml)?;
    Ok(())
}

fn write_toml(content: &Vec<Value>, path: &str) -> Result<()> {
    // 将 JSON Value 转换为 TOML 兼容的格式
    // 创建一个包含所有记录的表格结构
    let mut toml_content = std::collections::HashMap::new();
    toml_content.insert("records", content);

    let toml = toml::to_string(&toml_content)?;
    fs::write(path, toml)?;
    Ok(())
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let content = read_csv(input);
    match format {
        OutputFormat::Json => write_json(&content, output)?,
        OutputFormat::Yaml => write_yaml(&content, output)?,
        OutputFormat::Toml => write_toml(&content, output)?,
    }
    Ok(())
}
