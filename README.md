# kube-eye-frontend-server

kube-eye 前端服务器，提供静态文件服务和 kube-eye 报表 pdf。

## TODO:
- [x] 静态文件服务器
- [x] typst mvp 实现
- [ ] typst 集成，提供报表接口
   - [x] ks api-server 地址配置文件
   - [ ] typst 模版
   - [ ] 接口查询
   - [ ] typst 内容生产
   - [x] 输出 pdf 流 
- [ ] docker 构建
- [ ] github action 集成



#### 参考链接: 
 - [typst as lib](https://crates.io/crates/typst-as-lib)
 - [axum](https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs)
 - [构建](https://docker.github.net.cn/language/rust/)
