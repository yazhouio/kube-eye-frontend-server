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

#let h1(title) = {
  set text(font: 字体.黑体, tracking: 0.1em, size: 字号.小二)
  set block(spacing: 1.5em, )
  v(1em)
  title
  v(1em)
}


#let template(doc, lang) = {
    set page(
        margin:(x: 1.3em, y: 2em),
        paper: "a4",
    )

     set text(lang: lang, font: 字体.宋体, size: 字号.小五, fill: rgb("#242e42"))
    show figure: set block(breakable: true)
    let linespacing =  1em;
    show strong: it => text(font: 字体.黑体, weight: "semibold", it.body);
    show emph: it => text(font: 字体.楷体, style: "italic", it.body);
    show par: set par(spacing: 0.6em);
    show raw: set text(font: 字体.代码);
    set heading(numbering: (..args) => {
      let nums = args.pos()
      if nums.len() == 1 {
        return numbering("1.", ..nums)
      } else {
        return numbering("1.1", ..nums)
      }
    })
    show heading: set text(font: 字体.黑体)
    show heading.where(level: 1): set text(font: 字体.黑体, size: 字号.三号)
    show heading.where(level: 2): set text(font: 字体.黑体, size: 字号.四号)
    show heading: set block(above: 1.5em)

    doc
}

#let custom-table(headers, data, empty, rest) = {
    set table.header(repeat: true)
    let empty-cell = if data.len() == 0 {
        table.cell(colspan: headers.len(), align: center, stroke: (top: none, bottom: 1pt, left: none, right: none), [#empty])
    }
    if data.len() == 0 {
        data.push(empty-cell)
    }

    show table.cell: set text(size: 字号.小五)
    show table.cell.where(y: 0): set text(weight: "bold", size: 字号.小五)
    show table.cell: set align(left + horizon)
    show table.cell.where(y: 0): set align(left + top)

    table(
        columns: headers.map(h => 1fr),
        inset:(x: 8pt, y: 8pt),
        stroke: (x, y) => {
            let top = if y == 0 { 1pt + rgb("#cccccc60")} else { none }
            let bottom = if y == 0 or y == data.len() { 1pt + rgb("#cccccc60") } else {1pt + rgb("#cccccc60")}
            let left = if x == 0 { none } else { none }
            let right = if x == headers.len() - 1 { none } else { none }
            return (top: top, bottom: bottom, left: left, right: right)
          },
        table.header(
            ..headers.map(h => [*#h*]),
        ),
        ..data.flatten(),
        ..rest,
      
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
    show table.cell.where(x: 0): set text(weight: "bold", size: 字号.小五)
  
    let empty-cell = table.cell(rowspan: headers.len(), align: 
        center + horizon, stroke: (top: 1pt + rgb("#cccccc60"), bottom: 1pt + rgb("#cccccc60"), left: 1pt + rgb("#cccccc60"), right: none), [#empty])
     let rows = transpose(headers, data, empty-cell)
   
    table(
        columns: (100pt, ..range(rows.first().len() -1).map(_ => 1fr)),
        inset:(x: 16pt, y: 8pt),
        align: left,
        stroke: (x, y) => {
            let top = if y == 0 { 1pt + rgb("#cccccc60") } else { none }
            let bottom = if y == headers.len() -1 { 1pt + rgb("#cccccc60") } else {1pt + rgb("#cccccc60")}
            let left = if x == 0 { none } else { none }
            let right = if x==0 or x == rows.first().len() - 1 { none } else { none }
            return (top: top, bottom: bottom, left: left, right: right)
        },
        ..rows.flatten(),
        
    )
}

#let indent(content) = {
  h(2em)
  text(content)
  v(0.2em)
}

#let errorFn(error_text) = {
  box(baseline: 15%, inset: 2pt + 0pt, fill: rgb("#f5a623"), radius: 2pt, [#error_text])
}
