mod my_err;
mod git_commands;

use std::{
    fs,
    process::{Command, Stdio},
};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use crate::git_commands::GitCommand;
use crate::my_err::AppError;
use dialoguer::{ Select};

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
    pub branch: String
}
#[derive( Deserialize)]
pub struct Git {
    before: Before
}
#[derive( Deserialize)]
pub struct Before {
    is_pull: bool
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
    let cfg = load_config(path.clone())?;

    let projects = cfg.projects;

    let mut items = Vec::new();
    for p in &projects {
        // println!("项目名称：{}， 项目地址：{}", p.name, p.path);
        items.push(p.name.clone());
    }

    let selected = Select::new()
        .with_prompt("请选择要启动的项目")
        .items(&items)
        .default(0)
        .interact_opt()
        .map_err(|e| AppError::UserCancel(e.to_string()))?; // 这里需要添加 ? 来解包 Result

    // 修复这里的模式匹配
    if let Some(selected_index) = selected {
        let selected_name = &items[selected_index];
        println!("已选择：{}", selected_name);

        let project = projects
            .iter()
            .find(|p| p.name.eq(selected_name))
            .ok_or_else(|| AppError::ProjectNotFound(selected_name.clone()))?;

        start_project_by_yml(&project.name, &path)?;

    } else {
        println!("用户取消了选择");
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

    fetch_project(Option::from(&cfg.git),&project)?;
    start_project(&project)?;
    Ok(())
}

fn fetch_project(git: Option<&Git>, project: &Project) -> Result<(), AppError> {
    // 避免多次 unwrap，可以先解构
    let git_ref = git.ok_or_else(|| AppError::GITREPO("Git配置为空".to_string()))?;
    let is_pull = git_ref.before.is_pull;
    let path = &project.path;

    if is_pull {
        if !GitCommand::is_git_repo(path) {
            return Err(AppError::GITREPO("该项目未初始化Git仓库".to_string()));
        }

        let cfg_branch = &project.branch; // 不需要再次 unwrap

        let current_branch = GitCommand::get_current_branch(path)
            .map_err(|e| AppError::GITREPO(format!("获取当前分支失败: {}", e)))?;

        println!("当前分支: {}", current_branch);

        if *cfg_branch != current_branch {
            println!("正在切换至：{} 分支", cfg_branch);
            GitCommand::checkout_branch(path, cfg_branch)
                .map_err(|e| AppError::GITREPO(format!("切换分支失败: {}", e)))?;
        }

        // 拉取分支
        println!("正在拉取：{} 分支", cfg_branch);
        GitCommand::pull_branch(path, cfg_branch)
            .map_err(|e| AppError::GITREPO(format!("拉取分支失败: {}", e)))?;
    }

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