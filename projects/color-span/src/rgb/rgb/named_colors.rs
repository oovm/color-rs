use super::*;

impl RGB {
    /// <span style="color:aliceblue">aliceblue: #F0F8FF</span>
    pub const ALICEBLUE: Self = Self { r: 240, g: 248, b: 255 };

    /// <span style="color:antiquewhite">antiquewhite: #FAEBD7</span>
    pub const ANTIQUEWHITE: Self = Self { r: 250, g: 235, b: 215 };

    /// <span style="color:aqua">aqua: #00FFFF</span>
    pub const AQUA: Self = Self { r: 0, g: 255, b: 255 };

    /// <span style="color:aquamarine">aquamarine: #7FFFD4</span>
    pub const AQUAMARINE: Self = Self { r: 127, g: 255, b: 212 };

    /// <span style="color:azure">azure: #F0FFFF</span>
    pub const AZURE: Self = Self { r: 240, g: 255, b: 255 };

    /// <span style="color:beige">beige: #F5F5DC</span>
    pub const BEIGE: Self = Self { r: 245, g: 245, b: 220 };

    /// <span style="color:bisque">bisque: #FFE4C4</span>
    pub const BISQUE: Self = Self { r: 255, g: 228, b: 196 };

    /// <span style="color:black">black: #000000</span>
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };

    /// <span style="color:blanchedalmond">blanchedalmond: #FFEBCD</span>
    pub const BLANCHEDALMOND: Self = Self { r: 255, g: 235, b: 205 };

    /// <span style="color:blue">blue: #0000FF</span>
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };

    /// <span style="color:blueviolet">blueviolet: #8A2BE2</span>
    pub const BLUEVIOLET: Self = Self { r: 138, g: 43, b: 226 };

    /// <span style="color:brown">brown: #A52A2A</span>
    pub const BROWN: Self = Self { r: 165, g: 42, b: 42 };

    /// <span style="color:burlywood">burlywood: #DEB887</span>
    pub const BURLYWOOD: Self = Self { r: 222, g: 184, b: 135 };

    /// <span style="color:cadetblue">cadetblue: #5F9EA0</span>
    pub const CADETBLUE: Self = Self { r: 95, g: 158, b: 160 };

    /// <span style="color:chartreuse">chartreuse: #7FFF00</span>
    pub const CHARTREUSE: Self = Self { r: 127, g: 255, b: 0 };

    /// <span style="color:chocolate">chocolate: #D2691E</span>
    pub const CHOCOLATE: Self = Self { r: 210, g: 105, b: 30 };

    /// <span style="color:coral">coral: #FF7F50</span>
    pub const CORAL: Self = Self { r: 255, g: 127, b: 80 };

    /// <span style="color:cornflowerblue">cornflowerblue: #6495ED</span>
    pub const CORNFLOWERBLUE: Self = Self { r: 100, g: 149, b: 237 };

    /// <span style="color:cornsilk">cornsilk: #FFF8DC</span>
    pub const CORNSILK: Self = Self { r: 255, g: 248, b: 220 };

    /// <span style="color:crimson">crimson: #DC143C</span>
    pub const CRIMSON: Self = Self { r: 220, g: 20, b: 60 };

    /// <span style="color:cyan">cyan: #00FFFF</span>
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255 };

    /// <span style="color:darkblue">darkblue: #00008B</span>
    pub const DARKBLUE: Self = Self { r: 0, g: 0, b: 139 };

    /// <span style="color:darkcyan">darkcyan: #008B8B</span>
    pub const DARKCYAN: Self = Self { r: 0, g: 139, b: 139 };

    /// <span style="color:darkgoldenrod">darkgoldenrod: #B8860B</span>
    pub const DARKGOLDENROD: Self = Self { r: 184, g: 134, b: 11 };

    /// <span style="color:darkgray">darkgray: #A9A9A9</span>
    pub const DARKGRAY: Self = Self { r: 169, g: 169, b: 169 };

    /// <span style="color:darkgreen">darkgreen: #006400</span>
    pub const DARKGREEN: Self = Self { r: 0, g: 100, b: 0 };

    /// <span style="color:darkgrey">darkgrey: #A9A9A9</span>
    pub const DARKGREY: Self = Self { r: 169, g: 169, b: 169 };

    /// <span style="color:darkkhaki">darkkhaki: #BDB76B</span>
    pub const DARKKHAKI: Self = Self { r: 189, g: 183, b: 107 };

    /// <span style="color:darkmagenta">darkmagenta: #8B008B</span>
    pub const DARKMAGENTA: Self = Self { r: 139, g: 0, b: 139 };

    /// <span style="color:darkolivegreen">darkolivegreen: #556B2F</span>
    pub const DARKOLIVEGREEN: Self = Self { r: 85, g: 107, b: 47 };

    /// <span style="color:darkorange">darkorange: #FF8C00</span>
    pub const DARKORANGE: Self = Self { r: 255, g: 140, b: 0 };

    /// <span style="color:darkorchid">darkorchid: #9932CC</span>
    pub const DARKORCHID: Self = Self { r: 153, g: 50, b: 204 };

    /// <span style="color:darkred">darkred: #8B0000</span>
    pub const DARKRED: Self = Self { r: 139, g: 0, b: 0 };

    /// <span style="color:darksalmon">darksalmon: #E9967A</span>
    pub const DARKSALMON: Self = Self { r: 233, g: 150, b: 122 };

    /// <span style="color:darkseagreen">darkseagreen: #8FBC8F</span>
    pub const DARKSEAGREEN: Self = Self { r: 143, g: 188, b: 143 };

    /// <span style="color:darkslateblue">darkslateblue: #483D8B</span>
    pub const DARKSLATEBLUE: Self = Self { r: 72, g: 61, b: 139 };

    /// <span style="color:darkslategray">darkslategray: #2F4F4F</span>
    pub const DARKSLATEGRAY: Self = Self { r: 47, g: 79, b: 79 };

    /// <span style="color:darkslategrey">darkslategrey: #2F4F4F</span>
    pub const DARKSLATEGREY: Self = Self { r: 47, g: 79, b: 79 };

    /// <span style="color:darkturquoise">darkturquoise: #00CED1</span>
    pub const DARKTURQUOISE: Self = Self { r: 0, g: 206, b: 209 };

    /// <span style="color:darkviolet">darkviolet: #9400D3</span>
    pub const DARKVIOLET: Self = Self { r: 148, g: 0, b: 211 };

    /// <span style="color:deeppink">deeppink: #FF1493</span>
    pub const DEEPPINK: Self = Self { r: 255, g: 20, b: 147 };

    /// <span style="color:deepskyblue">deepskyblue: #00BFFF</span>
    pub const DEEPSKYBLUE: Self = Self { r: 0, g: 191, b: 255 };

    /// <span style="color:dimgray">dimgray: #696969</span>
    pub const DIMGRAY: Self = Self { r: 105, g: 105, b: 105 };

    /// <span style="color:dimgrey">dimgrey: #696969</span>
    pub const DIMGREY: Self = Self { r: 105, g: 105, b: 105 };

    /// <span style="color:dodgerblue">dodgerblue: #1E90FF</span>
    pub const DODGERBLUE: Self = Self { r: 30, g: 144, b: 255 };

    /// <span style="color:firebrick">firebrick: #B22222</span>
    pub const FIREBRICK: Self = Self { r: 178, g: 34, b: 34 };

    /// <span style="color:floralwhite">floralwhite: #FFFAF0</span>
    pub const FLORALWHITE: Self = Self { r: 255, g: 250, b: 240 };

    /// <span style="color:forestgreen">forestgreen: #228B22</span>
    pub const FORESTGREEN: Self = Self { r: 34, g: 139, b: 34 };

    /// <span style="color:fuchsia">fuchsia: #FF00FF</span>
    pub const FUCHSIA: Self = Self { r: 255, g: 0, b: 255 };

    /// <span style="color:gainsboro">gainsboro: #DCDCDC</span>
    pub const GAINSBORO: Self = Self { r: 220, g: 220, b: 220 };

    /// <span style="color:ghostwhite">ghostwhite: #F8F8FF</span>
    pub const GHOSTWHITE: Self = Self { r: 248, g: 248, b: 255 };

    /// <span style="color:goldenrod">goldenrod: #DAA520</span>
    pub const GOLDENROD: Self = Self { r: 218, g: 165, b: 32 };

    /// <span style="color:gold">gold: #FFD700</span>
    pub const GOLD: Self = Self { r: 255, g: 215, b: 0 };

    /// <span style="color:gray">gray: #808080</span>
    pub const GRAY: Self = Self { r: 128, g: 128, b: 128 };

    /// <span style="color:green">green: #008000</span>
    pub const GREEN: Self = Self { r: 0, g: 128, b: 0 };

    /// <span style="color:greenyellow">greenyellow: #ADFF2F</span>
    pub const GREENYELLOW: Self = Self { r: 173, g: 255, b: 47 };

    /// <span style="color:grey">grey: #808080</span>
    pub const GREY: Self = Self { r: 128, g: 128, b: 128 };

    /// <span style="color:honeydew">honeydew: #F0FFF0</span>
    pub const HONEYDEW: Self = Self { r: 240, g: 255, b: 240 };

    /// <span style="color:hotpink">hotpink: #FF69B4</span>
    pub const HOTPINK: Self = Self { r: 255, g: 105, b: 180 };

    /// <span style="color:indianred">indianred: #CD5C5C</span>
    pub const INDIANRED: Self = Self { r: 205, g: 92, b: 92 };

    /// <span style="color:indigo">indigo: #4B0082</span>
    pub const INDIGO: Self = Self { r: 75, g: 0, b: 130 };

    /// <span style="color:ivory">ivory: #FFFFF0</span>
    pub const IVORY: Self = Self { r: 255, g: 255, b: 240 };

    /// <span style="color:khaki">khaki: #F0E68C</span>
    pub const KHAKI: Self = Self { r: 240, g: 230, b: 140 };

    /// <span style="color:lavenderblush">lavenderblush: #FFF0F5</span>
    pub const LAVENDERBLUSH: Self = Self { r: 255, g: 240, b: 245 };

    /// <span style="color:lavender">lavender: #E6E6FA</span>
    pub const LAVENDER: Self = Self { r: 230, g: 230, b: 250 };

    /// <span style="color:lawngreen">lawngreen: #7CFC00</span>
    pub const LAWNGREEN: Self = Self { r: 124, g: 252, b: 0 };

    /// <span style="color:lemonchiffon">lemonchiffon: #FFFACD</span>
    pub const LEMONCHIFFON: Self = Self { r: 255, g: 250, b: 205 };

    /// <span style="color:lightblue">lightblue: #ADD8E6</span>
    pub const LIGHTBLUE: Self = Self { r: 173, g: 216, b: 230 };

    /// <span style="color:lightcoral">lightcoral: #F08080</span>
    pub const LIGHTCORAL: Self = Self { r: 240, g: 128, b: 128 };

    /// <span style="color:lightcyan">lightcyan: #E0FFFF</span>
    pub const LIGHTCYAN: Self = Self { r: 224, g: 255, b: 255 };

    /// <span style="color:lightgoldenrodyellow">lightgoldenrodyellow: #FAFAD2</span>
    pub const LIGHTGOLDENRODYELLOW: Self = Self { r: 250, g: 250, b: 210 };

    /// <span style="color:lightgray">lightgray: #D3D3D3</span>
    pub const LIGHTGRAY: Self = Self { r: 211, g: 211, b: 211 };

    /// <span style="color:lightgreen">lightgreen: #90EE90</span>
    pub const LIGHTGREEN: Self = Self { r: 144, g: 238, b: 144 };

    /// <span style="color:lightgrey">lightgrey: #D3D3D3</span>
    pub const LIGHTGREY: Self = Self { r: 211, g: 211, b: 211 };

    /// <span style="color:lightpink">lightpink: #FFB6C1</span>
    pub const LIGHTPINK: Self = Self { r: 255, g: 182, b: 193 };

    /// <span style="color:lightsalmon">lightsalmon: #FFA07A</span>
    pub const LIGHTSALMON: Self = Self { r: 255, g: 160, b: 122 };

    /// <span style="color:lightseagreen">lightseagreen: #20B2AA</span>
    pub const LIGHTSEAGREEN: Self = Self { r: 32, g: 178, b: 170 };

    /// <span style="color:lightskyblue">lightskyblue: #87CEFA</span>
    pub const LIGHTSKYBLUE: Self = Self { r: 135, g: 206, b: 250 };

    /// <span style="color:lightslategray">lightslategray: #778899</span>
    pub const LIGHTSLATEGRAY: Self = Self { r: 119, g: 136, b: 153 };

    /// <span style="color:lightslategrey">lightslategrey: #778899</span>
    pub const LIGHTSLATEGREY: Self = Self { r: 119, g: 136, b: 153 };

    /// <span style="color:lightsteelblue">lightsteelblue: #B0C4DE</span>
    pub const LIGHTSTEELBLUE: Self = Self { r: 176, g: 196, b: 222 };

    /// <span style="color:lightyellow">lightyellow: #FFFFE0</span>
    pub const LIGHTYELLOW: Self = Self { r: 255, g: 255, b: 224 };

    /// <span style="color:lime">lime: #00FF00</span>
    pub const LIME: Self = Self { r: 0, g: 255, b: 0 };

    /// <span style="color:limegreen">limegreen: #32CD32</span>
    pub const LIMEGREEN: Self = Self { r: 50, g: 205, b: 50 };

    /// <span style="color:linen">linen: #FAF0E6</span>
    pub const LINEN: Self = Self { r: 250, g: 240, b: 230 };

    /// <span style="color:magenta">magenta: #FF00FF</span>
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255 };

    /// <span style="color:maroon">maroon: #800000</span>
    pub const MAROON: Self = Self { r: 128, g: 0, b: 0 };

    /// <span style="color:mediumaquamarine">mediumaquamarine: #66CDAA</span>
    pub const MEDIUMAQUAMARINE: Self = Self { r: 102, g: 205, b: 170 };

    /// <span style="color:mediumblue">mediumblue: #0000CD</span>
    pub const MEDIUMBLUE: Self = Self { r: 0, g: 0, b: 205 };

    /// <span style="color:mediumorchid">mediumorchid: #BA55D3</span>
    pub const MEDIUMORCHID: Self = Self { r: 186, g: 85, b: 211 };

    /// <span style="color:mediumpurple">mediumpurple: #9370DB</span>
    pub const MEDIUMPURPLE: Self = Self { r: 147, g: 112, b: 219 };

    /// <span style="color:mediumseagreen">mediumseagreen: #3CB371</span>
    pub const MEDIUMSEAGREEN: Self = Self { r: 60, g: 179, b: 113 };

    /// <span style="color:mediumslateblue">mediumslateblue: #7B68EE</span>
    pub const MEDIUMSLATEBLUE: Self = Self { r: 123, g: 104, b: 238 };

    /// <span style="color:mediumspringgreen">mediumspringgreen: #00FA9A</span>
    pub const MEDIUMSPRINGGREEN: Self = Self { r: 0, g: 250, b: 154 };

    /// <span style="color:mediumturquoise">mediumturquoise: #48D1CC</span>
    pub const MEDIUMTURQUOISE: Self = Self { r: 72, g: 209, b: 204 };

    /// <span style="color:mediumvioletred">mediumvioletred: #C71585</span>
    pub const MEDIUMVIOLETRED: Self = Self { r: 199, g: 21, b: 133 };

    /// <span style="color:midnightblue">midnightblue: #191970</span>
    pub const MIDNIGHTBLUE: Self = Self { r: 25, g: 25, b: 112 };

    /// <span style="color:mintcream">mintcream: #F5FFFA</span>
    pub const MINTCREAM: Self = Self { r: 245, g: 255, b: 250 };

    /// <span style="color:mistyrose">mistyrose: #FFE4E1</span>
    pub const MISTYROSE: Self = Self { r: 255, g: 228, b: 225 };

    /// <span style="color:moccasin">moccasin: #FFE4B5</span>
    pub const MOCCASIN: Self = Self { r: 255, g: 228, b: 181 };

    /// <span style="color:navajowhite">navajowhite: #FFDEAD</span>
    pub const NAVAJOWHITE: Self = Self { r: 255, g: 222, b: 173 };

    /// <span style="color:navy">navy: #000080</span>
    pub const NAVY: Self = Self { r: 0, g: 0, b: 128 };

    /// <span style="color:oldlace">oldlace: #FDF5E6</span>
    pub const OLDLACE: Self = Self { r: 253, g: 245, b: 230 };

    /// <span style="color:olive">olive: #808000</span>
    pub const OLIVE: Self = Self { r: 128, g: 128, b: 0 };

    /// <span style="color:olivedrab">olivedrab: #6B8E23</span>
    pub const OLIVEDRAB: Self = Self { r: 107, g: 142, b: 35 };

    /// <span style="color:orange">orange: #FFA500</span>
    pub const ORANGE: Self = Self { r: 255, g: 165, b: 0 };

    /// <span style="color:orangered">orangered: #FF4500</span>
    pub const ORANGERED: Self = Self { r: 255, g: 69, b: 0 };

    /// <span style="color:orchid">orchid: #DA70D6</span>
    pub const ORCHID: Self = Self { r: 218, g: 112, b: 214 };

    /// <span style="color:palegoldenrod">palegoldenrod: #EEE8AA</span>
    pub const PALEGOLDENROD: Self = Self { r: 238, g: 232, b: 170 };

    /// <span style="color:palegreen">palegreen: #98FB98</span>
    pub const PALEGREEN: Self = Self { r: 152, g: 251, b: 152 };

    /// <span style="color:paleturquoise">paleturquoise: #AFEEEE</span>
    pub const PALETURQUOISE: Self = Self { r: 175, g: 238, b: 238 };

    /// <span style="color:palevioletred">palevioletred: #DB7093</span>
    pub const PALEVIOLETRED: Self = Self { r: 219, g: 112, b: 147 };

    /// <span style="color:papayawhip">papayawhip: #FFEFD5</span>
    pub const PAPAYAWHIP: Self = Self { r: 255, g: 239, b: 213 };

    /// <span style="color:peachpuff">peachpuff: #FFDAB9</span>
    pub const PEACHPUFF: Self = Self { r: 255, g: 218, b: 185 };

    /// <span style="color:peru">peru: #CD853F</span>
    pub const PERU: Self = Self { r: 205, g: 133, b: 63 };

    /// <span style="color:pink">pink: #FFC0CB</span>
    pub const PINK: Self = Self { r: 255, g: 192, b: 203 };

    /// <span style="color:plum">plum: #DDA0DD</span>
    pub const PLUM: Self = Self { r: 221, g: 160, b: 221 };

    /// <span style="color:powderblue">powderblue: #B0E0E6</span>
    pub const POWDERBLUE: Self = Self { r: 176, g: 224, b: 230 };

    /// <span style="color:purple">purple: #800080</span>
    pub const PURPLE: Self = Self { r: 128, g: 0, b: 128 };

    /// <span style="color:rebeccapurple">rebeccapurple: #663399</span>
    pub const REBECCAPURPLE: Self = Self { r: 102, g: 51, b: 153 };

    /// <span style="color:red">red: #FF0000</span>
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };

    /// <span style="color:rosybrown">rosybrown: #BC8F8F</span>
    pub const ROSYBROWN: Self = Self { r: 188, g: 143, b: 143 };

    /// <span style="color:royalblue">royalblue: #4169E1</span>
    pub const ROYALBLUE: Self = Self { r: 65, g: 105, b: 225 };

    /// <span style="color:saddlebrown">saddlebrown: #8B4513</span>
    pub const SADDLEBROWN: Self = Self { r: 139, g: 69, b: 19 };

    /// <span style="color:salmon">salmon: #FA8072</span>
    pub const SALMON: Self = Self { r: 250, g: 128, b: 114 };

    /// <span style="color:sandybrown">sandybrown: #F4A460</span>
    pub const SANDYBROWN: Self = Self { r: 244, g: 164, b: 96 };

    /// <span style="color:seagreen">seagreen: #2E8B57</span>
    pub const SEAGREEN: Self = Self { r: 46, g: 139, b: 87 };

    /// <span style="color:seashell">seashell: #FFF5EE</span>
    pub const SEASHELL: Self = Self { r: 255, g: 245, b: 238 };

    /// <span style="color:sienna">sienna: #A0522D</span>
    pub const SIENNA: Self = Self { r: 160, g: 82, b: 45 };

    /// <span style="color:silver">silver: #C0C0C0</span>
    pub const SILVER: Self = Self { r: 192, g: 192, b: 192 };

    /// <span style="color:skyblue">skyblue: #87CEEB</span>
    pub const SKYBLUE: Self = Self { r: 135, g: 206, b: 235 };

    /// <span style="color:slateblue">slateblue: #6A5ACD</span>
    pub const SLATEBLUE: Self = Self { r: 106, g: 90, b: 205 };

    /// <span style="color:slategray">slategray: #708090</span>
    pub const SLATEGRAY: Self = Self { r: 112, g: 128, b: 144 };

    /// <span style="color:slategrey">slategrey: #708090</span>
    pub const SLATEGREY: Self = Self { r: 112, g: 128, b: 144 };

    /// <span style="color:snow">snow: #FFFAFA</span>
    pub const SNOW: Self = Self { r: 255, g: 250, b: 250 };

    /// <span style="color:springgreen">springgreen: #00FF7F</span>
    pub const SPRINGGREEN: Self = Self { r: 0, g: 255, b: 127 };

    /// <span style="color:steelblue">steelblue: #4682B4</span>
    pub const STEELBLUE: Self = Self { r: 70, g: 130, b: 180 };

    /// <span style="color:tan">tan: #D2B48C</span>
    pub const TAN: Self = Self { r: 210, g: 180, b: 140 };

    /// <span style="color:teal">teal: #008080</span>
    pub const TEAL: Self = Self { r: 0, g: 128, b: 128 };

    /// <span style="color:thistle">thistle: #D8BFD8</span>
    pub const THISTLE: Self = Self { r: 216, g: 191, b: 216 };

    /// <span style="color:tomato">tomato: #FF6347</span>
    pub const TOMATO: Self = Self { r: 255, g: 99, b: 71 };

    /// <span style="color:turquoise">turquoise: #40E0D0</span>
    pub const TURQUOISE: Self = Self { r: 64, g: 224, b: 208 };

    /// <span style="color:violet">violet: #EE82EE</span>
    pub const VIOLET: Self = Self { r: 238, g: 130, b: 238 };

    /// <span style="color:wheat">wheat: #F5DEB3</span>
    pub const WHEAT: Self = Self { r: 245, g: 222, b: 179 };

    /// <span style="color:white">white: #FFFFFF</span>
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };

    /// <span style="color:whitesmoke">whitesmoke: #F5F5F5</span>
    pub const WHITESMOKE: Self = Self { r: 245, g: 245, b: 245 };

    /// <span style="color:yellow">yellow: #FFFF00</span>
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0 };

    /// <span style="color:yellowgreen">yellowgreen: #9ACD32</span>
    pub const YELLOWGREEN: Self = Self { r: 154, g: 205, b: 50 };
}
