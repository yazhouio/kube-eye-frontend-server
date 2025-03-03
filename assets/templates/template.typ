#let 字号 = (
  初号: 42pt,
  小初: 36pt,
  一号: 26pt,
  小一: 24pt,
  二号: 22pt,
  小二: 18pt,
  三号: 16pt,
  小三: 15pt,
  四号: 14pt,
  中四: 13pt,
  小四: 12pt,
  五号: 10.5pt,
  小五: 9pt,
  六号: 7.5pt,
  小六: 6.5pt,
  七号: 5.5pt,
  小七: 5pt,
)

#let 字体 = (
  宋体: ("Times New Roman", "Noto Serif CJK SC", "Noto Serif SC"),
  黑体: ("Times New Roman", "SimHei", "Noto Sans CJK SC", "Noto Sans SC"),
  楷体: ("Times New Roman", "KaiTi"),
  代码: ("New Computer Modern Mono", "Times New Roman", "SimSun"),
)

#let template(doc, lang) = {
    set page(
        margin:(x: 0.9em, y: 1.3em),
        paper: "a4",
    )

    set text(lang: lang, font: 字体.宋体, size: 字号.小四)
    show figure: set block(breakable: true)
    let linespacing =  1em;
    show strong: it => text(font: 字体.黑体, weight: "semibold", it.body);
    show emph: it => text(font: 字体.楷体, style: "italic", it.body);
    show par: set par(spacing: 0.6em);
    show raw: set text(font: 字体.代码);
       // 自定义一级标题样式
    show heading.where(level: 1): it => {
        set text(font: 字体.黑体, size: 字号.小二)
        set block(spacing: 1.5em)
        align(left, it)
    }

    doc
}

#let custom-table(headers, data, empty) = {
    set table.header(repeat: true)
    let empty-cell = if data.len() == 0 {
        table.cell(colspan: headers.len(), align: center, stroke: (top: none, bottom: 1pt, left: 1pt, right: 1pt), [#empty])
    }

    if data.len() == 0 {
        data.push(empty-cell)
    }
     show table.cell.where(y: 0): set text(weight: "bold", size: 字号.小四)
    table(
        columns: headers.map(h => 1fr),
        inset:(x: 16pt, y: 8pt),
        align: left,
        stroke: (x, y) => {
            let top = if y == 0 { 1pt } else { none }
            let bottom = if y == 0 or y == data.len() { 1pt } else {1pt + rgb("#cccccc60")}
            let left = if x == 0 { 1pt } else { none }
            let right = if x == headers.len() - 1 { 1pt } else { none }
            return (top: top, bottom: bottom, left: left, right: right)
        },
        table.header(
            ..headers.map(h => [*#h*]),
        ),
        ..data.flatten(),
      
    )
}


#let transpose(headers, data, empty-cell) = {
    let result = ()
     if data.len() == 0 {
        for i in range(headers.len()) {
            let row = (headers.at(i),)
            if i == 0 {
                row.push(empty-cell)
            }
            result.push(row)
        }
    } else {
        for i in range(headers.len()) {
             let row = (headers.at(i), ..data.map(d => d.at(i))) // 第一列是属性名
            result.push(row)
        }
    }
    result
}

#let vertical-table(headers, data, empty) = {
    set table.header(repeat: false)
    show table.cell.where(x: 0): set text(weight: "bold", size: 字号.小四)
        let empty-cell = table.cell(rowspan: headers.len(), align: 
        center + horizon, stroke: (top: 1pt, bottom: 1pt + black, left: none, right: 1pt), [#empty])
     let rows = transpose(headers, data, empty-cell)
   
    table(
        columns: (100pt, ..range(rows.first().len() -1).map(_ => 1fr)),
        inset:(x: 16pt, y: 8pt),
        align: left,
         stroke: (x, y) => {
            let top = if y == 0 { 1pt } else { none }
            let bottom = if y == headers.len() -1 { 1pt } else {1pt + rgb("#cccccc60")}
            let left = if x == 0 { 1pt } else { none }
            let right = if x==0 or x == rows.first().len() - 1 { 1pt } else { none }
            return (top: top, bottom: bottom, left: left, right: right)
        },
        ..rows.flatten(),
        
    )
}

#let indent = h(2em)

#show: doc => template(doc, "zh")

= 一级标题1

巡检日期：*2025-03-02*

#indent 集群 host： 正常

#indent 集群 member： 正常

#indent 集群 CPU 最高使用率：1.32%（host， master3） 

#figure(
custom-table(
  ([姓名1],[年龄1],[职业1]),
 (),
  "暂无异常数据",
 
)
,
 caption: figure.caption(position: top, [asdfsa Probe results for design A]),
)

== 集群1
#figure(
custom-table(
  ([姓名],[年龄],[职业]),
 ( 
  ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
    ([张三], [25], [工程师]),
  ([李四], [30], [设计师]),
  ([王五], [28], [教师]),
 )
 ,()
),
 caption: figure.caption(position: top, [asdfsa Probe results for design A]),
)

= 一级标题2
#figure(
vertical-table(
  ([姓名2],[年龄],[职业]),
  (
      ([张三], [25], [工程师]),
      ([李四], [30], [设计师]),
      ([王五], [28], [教师]),
    )
 ,()
),
 caption: figure.caption(position: top, [asdfsa Probe results for design A]),
)

== 集群2
#figure(
vertical-table(
  ([姓名],[年龄],[职业]),
  (
      ([张三], [25], [工程师]),
    )
 ,()
),
 caption: figure.caption(position: top, [asdfsa Probe results for design A]),
)

== 集群3
#figure(
vertical-table(
  ([姓名],[年龄],[职业]),
 (),
  "暂无异常数据",
 
)
,
 caption: figure.caption(position: top, [asdfsa Probe results for design A]),
)
