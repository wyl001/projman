use std::process::Command;

pub struct GitCommand;

impl GitCommand {
    // 是否是git仓库
    pub fn is_git_repo(path: &String) -> bool{
        Command::new("git")
            .arg("rev-parse")
            .arg("--git-dir")
            .current_dir(path)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    //获取当前分支
    pub fn get_current_branch(path: &String) -> Result<String, String> {
        let output = Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .current_dir(path)
            .output()
            .map_err(|e| format!("执行 git 命令失败: {}", e))?;

        if !output.status.success() {
            return Err(format!("Error: {}", String::from_utf8_lossy(&output.stderr)))
        }

        let branch = String::from_utf8(output.stdout)
            .map_err(|e| format!("输出解析失败: {}", e))?
            .trim()
            .to_string();
        Ok(branch)
    }

    // 切换分支
    pub fn checkout_branch(path: &String, branch: &str) -> Result<(), String> {
        let output = Command::new("git")
            .arg("checkout")
            .arg(branch)
            .current_dir(path)
            .output()
            .map_err(|e| format!("执行 git 命令失败: {}", e))?;

        if !output.status.success() {
            return Err(format!("git checkout 失败: {}",
                               String::from_utf8_lossy(&output.stderr)));
        }
        Ok(())
    }
    //拉取远程分支
    pub fn pull_branch(path: &String, branch: &str) -> Result<(), String> {
        let output = Command::new("git")
            .arg("pull")
            .arg("origin")
            .arg(branch)
            .current_dir(path)
            .output()
            .map_err(|e| format!("执行 git 命令失败: {}", e))?;

        if !output.status.success() {
            return Err(format!("git pull 错误: {}",
                               String::from_utf8_lossy(&output.stderr)));
        }
        Ok( ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pull(){
        let branch = "master";
        let  path: &String = &"/Users/wyl/Documents/myProject/tauri-project/projman".to_string();
        let result = GitCommand::pull_branch(path, branch);

    }

    #[test]
    fn test_is_git_repo() {
        let is = GitCommand::is_git_repo(&r"/Users/wyl/Documents/myProject/tauri-project".to_string());

        assert_eq!(is, true)
    }
    #[test]
    fn test_get_current_branch() {
        let  path: &String = &"/Users/wyl/Documents/myProject/tauri-project/projman".to_string();

        let branch = GitCommand::get_current_branch(path);

        println!("{}", branch.unwrap())
    }
}