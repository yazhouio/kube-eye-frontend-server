# kube-eye-frontend
~~需求变更，由页面实现 pdf 导出功能。页面渲染 pdf，需要下载字体，等待时间可能较长。~~

# kube-eye-frontend-server

kube-eye 前端服务器，提供静态文件服务和 kube-eye 报表 pdf。
## 流程
~~1. 其他页面点击下载按钮，调用接口，生成 typst 文本，监听 postmessage 事件，打开此页面。~~
~~2. 页面获取 url 参数，通过 postmessage 传递参数，获取 typst 文本，渲染或下载。~~
1. 浏览器页面发起 post 接口。接口会返回 pdf。
2. 页面获取接口数据后，js 实现下载功能。

> 注：所有 typst 内容均在前端页面内组装完成，接口拿到 content 参数，渲染对应的 pdf。




## TODO:
- [x] 静态文件服务器
- [x] typst mvp 实现
- [ ] typst 集成，提供报表接口
   - [x] ~~ks api-server 地址配置文件~~
   - [ ] ~~typst 模版~~
   - [ ] ~~接口查询~~
   - [ ] ~~typst 内容生产~~
   - [x] 输出 pdf 流 
- [x] docker 构建
- [x] github action 集成
- [x] ~~使用 typst.ts 在前端界面实现 pdf 导出~~



#### 参考链接: 
 - [typst as lib](https://crates.io/crates/typst-as-lib)
 - [axum](https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs)
 - [构建](https://docker.github.net.cn/language/rust/)
 - [typst.ts](myriad-dreamin.github.io/typst.ts/)
