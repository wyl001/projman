mod my_err;

use std::{
    fs,
    process::{Command, Stdio},
};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use crate::my_err::AppError;

#[derive( Deserialize)]
pub struct Config {
    pub projects: Vec<Project>,
    pub git: Option< Git>
}

#[derive( Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub start: String,
}
#[derive( Deserialize)]
pub struct Git {
    before: Before
}
#[derive( Deserialize)]
pub struct Before {
    is_pull: bool,
    commands: Vec<String>
}

#[derive(Parser)]
#[command(name = "projman", about="简单的项目管理cli工具",long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // 配置文件路劲
    List {
        #[arg(long, short, default_value = "projman.yml")]
        config: String,
    },
    //启动项目
    Start {
        #[arg(long, short)]
        name: String,
        #[arg(long, short, default_value = "projman.yml")]
        config: String,
    },
}


// 读取配置文件中的 项目
pub fn scan_projects(path: String) -> Result<(), AppError> {
    if path.is_empty() {
        return Err(AppError::ConfigFileError("请指定配置文件路径".to_string()));
    }
    let cfg = load_config(path)?;
    for p in cfg.projects {
        println!("项目名称：{}， 项目地址：{}", p.name, p.path)
    }
    Ok(())
}

//启动项目
pub fn start_project_by_yml(name: &String, path: &String) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::ConfigFileError("请指定项目名".to_string()));
    }
    if path.is_empty() {
        return Err(AppError::ConfigFileError("请指定配置文件路径".to_string()));
    }
    let cfg = load_config(path.clone())?;

    let project = cfg
        .projects
        .into_iter()
        .find(|p| p.name.eq(name))
        .ok_or_else(|| AppError::ProjectNotFound(name.clone()))?;

    start_project(&project)?;
    Ok(())
}

fn start_project(project: &Project) -> Result<(), AppError> {
    println!(
        "启动项目: {} | 路径: {} | 命令: {}",
        project.name, project.path, project.start
    );
    // 判断操作系统
    let mut command = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&project.start);
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c").arg(&project.start);
        c
    };

    let mut child = command
        .current_dir(&project.path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| AppError::CommandExecutionError(format!("启动进程失败: {}", e)))?;

    child.wait()
        .map_err(|e| AppError::CommandExecutionError(format!("等待进程完成失败: {}", e)))?;

    Ok(())
}

fn load_config(path: String) -> Result<Config, AppError> {
    let contents = fs::read_to_string(&path)
        .map_err(|e| AppError::ConfigFileError(format!("无法读取配置文件 {}: {}", path, e)))?;

    serde_yaml::from_str(&contents)
        .map_err(|e| AppError::InvalidConfig(format!("无法解析配置文件 {}: {}", path, e)))
}