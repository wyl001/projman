use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ConfigFileError(String),
    ProjectNotFound(String),
    InvalidConfig(String),
    CommandExecutionError(String),
    GITREPO( String),
    UserCancel( String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigFileError(msg) => write!(f, "配置文件错误: {}", msg),
            AppError::ProjectNotFound(name) => write!(f, "项目不存在: {}", name),
            AppError::InvalidConfig(msg) => write!(f, "配置格式错误: {}", msg),
            AppError::CommandExecutionError(msg) => write!(f, "命令执行失败: {}", msg),
            AppError::GITREPO( msg) => write!(f, "Git操作失败: {}", msg),
            AppError::UserCancel( msg) => write!(f, "用户操作错误: {}", msg),
        }
    }
}

impl Error for AppError {}
