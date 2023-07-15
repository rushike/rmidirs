## Import Hierarchy

In Import Hierarchy top module don't import anything from module, their children imports only there parent.

- primitive.rs
  |    |
  |    |> utils.functions
  |    |
  |--> |>  models 
  |        |> core
  |        |> note_seq
  |--> |> parser
  |--> |> reader