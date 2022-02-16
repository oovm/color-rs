(* ::Package:: *)

SetDirectory@NotebookDirectory[];


getConst[name_ -> color_] := Block[
    {r, g, b},
    {r, g, b} = Round[255 * List @@ RGBColor[color]];
    TemplateApply["\
    /// <span style=\"color:`name1`\">`name1`: `c`</span>
    pub const `name2`: Self = Self { r: `r`, g: `g`, b: `b`, a: () };",
        <|
            "name1" -> name,
            "name2" -> ToUpperCase@name,
            "c" -> ToUpperCase@color,
            "r" -> r,
            "g" -> g,
            "b" -> b
        |>
    ]
];


rs = TemplateApply["\
use super::*;

impl RGB {
`fs`
}
",
    <|
        "fs" -> StringRiffle[getConst /@ Import["named_colors.json"], ""]
    |>
];
Export["named_colors.rs", rs, "Text"]
