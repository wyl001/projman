一、功能设计思路

一个项目管理 CLI 工具，本质是目录扫描 + 配置管理 + 快捷命令执行。可以考虑分几类功能：

项目发现和索引

扫描本地某个工作目录下的所有项目（比如 ~/workspace）。

按语言 / 框架分类，比如 java、rust、frontend-react。

自动读取项目的配置文件（如 package.json、Cargo.toml、pom.xml），提取关键信息。

项目启动与停止

对前端项目：执行 npm run dev / yarn start。

对后端项目：执行 cargo run、mvn spring-boot:run、java -jar 等。

支持并行启动多个服务，方便调试微服务。

快捷命令管理

给每个项目配置常用命令（例如 build、test、lint）。

一键执行，而不用每次 cd 再打长命令。

状态查看

查看哪些项目正在运行（可以在启动时记录 PID，写到 .pid 文件里）。

支持一键停止服务。

扩展功能

打开 IDE（比如 code .）。

Git 操作（git pull all、git status all）。

给项目打标签，分组管理。