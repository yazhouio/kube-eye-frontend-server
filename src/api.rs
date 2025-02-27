pub fn query_report() -> String {
    r#"
#set text(
  font: "New Computer Modern",
  size: 10pt
)
#set page(
  paper: "a6",
  margin: (x: 1.8cm, y: 1.5cm),
)
#set par(
  justify: true,
  leading: 0.52em,
)

= Introduction
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behavior of these natural structures.

#table(
  columns: (auto, auto, auto),
  inset: 10pt,
  align: center,
  [*姓名*], [*年龄*], [*职业*],
  [张三], [25], [工程师],
  [李四], [30], [设计师],
  [王五], [28], [教师],
)
"#
    .to_owned()
}
