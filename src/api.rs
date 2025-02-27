pub fn query_report() -> String {
    r#"
    #import "@preview/polylux:0.4.0": *
    
    #set page(paper: "presentation-16-9")
    
    #slide[
      #set page(footer: none)
      #set align(horizon + center)
    
    = Hello, World!
    A document (+ `polylux` library) rendered with `Typst`!
    ]"#
    .to_owned()
}
