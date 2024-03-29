use super::*;

impl RGB8 {
    /// <span style="color:#F0F8FF">aliceblue: #F0F8FF</span>
    pub const ALICEBLUE: Self = Self { r: 240, g: 248, b: 255, a: () };
    /// <span style="color:#FAEBD7">antiquewhite: #FAEBD7</span>
    pub const ANTIQUEWHITE: Self = Self { r: 250, g: 235, b: 215, a: () };
    /// <span style="color:#00FFFF">aqua: #00FFFF</span>
    pub const AQUA: Self = Self { r: 0, g: 255, b: 255, a: () };
    /// <span style="color:#7FFFD4">aquamarine: #7FFFD4</span>
    pub const AQUAMARINE: Self = Self { r: 127, g: 255, b: 212, a: () };
    /// <span style="color:#F0FFFF">azure: #F0FFFF</span>
    pub const AZURE: Self = Self { r: 240, g: 255, b: 255, a: () };
    /// <span style="color:#F5F5DC">beige: #F5F5DC</span>
    pub const BEIGE: Self = Self { r: 245, g: 245, b: 220, a: () };
    /// <span style="color:#FFE4C4">bisque: #FFE4C4</span>
    pub const BISQUE: Self = Self { r: 255, g: 228, b: 196, a: () };
    /// <span style="color:#000000">black: #000000</span>
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: () };
    /// <span style="color:#FFEBCD">blanchedalmond: #FFEBCD</span>
    pub const BLANCHEDALMOND: Self = Self { r: 255, g: 235, b: 205, a: () };
    /// <span style="color:#0000FF">blue: #0000FF</span>
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255, a: () };
    /// <span style="color:#8A2BE2">blueviolet: #8A2BE2</span>
    pub const BLUEVIOLET: Self = Self { r: 138, g: 43, b: 226, a: () };
    /// <span style="color:#A52A2A">brown: #A52A2A</span>
    pub const BROWN: Self = Self { r: 165, g: 42, b: 42, a: () };
    /// <span style="color:#DEB887">burlywood: #DEB887</span>
    pub const BURLYWOOD: Self = Self { r: 222, g: 184, b: 135, a: () };
    /// <span style="color:#5F9EA0">cadetblue: #5F9EA0</span>
    pub const CADETBLUE: Self = Self { r: 95, g: 158, b: 160, a: () };
    /// <span style="color:#7FFF00">chartreuse: #7FFF00</span>
    pub const CHARTREUSE: Self = Self { r: 127, g: 255, b: 0, a: () };
    /// <span style="color:#D2691E">chocolate: #D2691E</span>
    pub const CHOCOLATE: Self = Self { r: 210, g: 105, b: 30, a: () };
    /// <span style="color:#FF7F50">coral: #FF7F50</span>
    pub const CORAL: Self = Self { r: 255, g: 127, b: 80, a: () };
    /// <span style="color:#6495ED">cornflowerblue: #6495ED</span>
    pub const CORNFLOWERBLUE: Self = Self { r: 100, g: 149, b: 237, a: () };
    /// <span style="color:#FFF8DC">cornsilk: #FFF8DC</span>
    pub const CORNSILK: Self = Self { r: 255, g: 248, b: 220, a: () };
    /// <span style="color:#DC143C">crimson: #DC143C</span>
    pub const CRIMSON: Self = Self { r: 220, g: 20, b: 60, a: () };
    /// <span style="color:#00FFFF">cyan: #00FFFF</span>
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255, a: () };
    /// <span style="color:#00008B">darkblue: #00008B</span>
    pub const DARKBLUE: Self = Self { r: 0, g: 0, b: 139, a: () };
    /// <span style="color:#008B8B">darkcyan: #008B8B</span>
    pub const DARKCYAN: Self = Self { r: 0, g: 139, b: 139, a: () };
    /// <span style="color:#B8860B">darkgoldenrod: #B8860B</span>
    pub const DARKGOLDENROD: Self = Self { r: 184, g: 134, b: 11, a: () };
    /// <span style="color:#A9A9A9">darkgray: #A9A9A9</span>
    pub const DARKGRAY: Self = Self { r: 169, g: 169, b: 169, a: () };
    /// <span style="color:#006400">darkgreen: #006400</span>
    pub const DARKGREEN: Self = Self { r: 0, g: 100, b: 0, a: () };
    /// <span style="color:#A9A9A9">darkgrey: #A9A9A9</span>
    pub const DARKGREY: Self = Self { r: 169, g: 169, b: 169, a: () };
    /// <span style="color:#BDB76B">darkkhaki: #BDB76B</span>
    pub const DARKKHAKI: Self = Self { r: 189, g: 183, b: 107, a: () };
    /// <span style="color:#8B008B">darkmagenta: #8B008B</span>
    pub const DARKMAGENTA: Self = Self { r: 139, g: 0, b: 139, a: () };
    /// <span style="color:#556B2F">darkolivegreen: #556B2F</span>
    pub const DARKOLIVEGREEN: Self = Self { r: 85, g: 107, b: 47, a: () };
    /// <span style="color:#FF8C00">darkorange: #FF8C00</span>
    pub const DARKORANGE: Self = Self { r: 255, g: 140, b: 0, a: () };
    /// <span style="color:#9932CC">darkorchid: #9932CC</span>
    pub const DARKORCHID: Self = Self { r: 153, g: 50, b: 204, a: () };
    /// <span style="color:#8B0000">darkred: #8B0000</span>
    pub const DARKRED: Self = Self { r: 139, g: 0, b: 0, a: () };
    /// <span style="color:#E9967A">darksalmon: #E9967A</span>
    pub const DARKSALMON: Self = Self { r: 233, g: 150, b: 122, a: () };
    /// <span style="color:#8FBC8F">darkseagreen: #8FBC8F</span>
    pub const DARKSEAGREEN: Self = Self { r: 143, g: 188, b: 143, a: () };
    /// <span style="color:#483D8B">darkslateblue: #483D8B</span>
    pub const DARKSLATEBLUE: Self = Self { r: 72, g: 61, b: 139, a: () };
    /// <span style="color:#2F4F4F">darkslategray: #2F4F4F</span>
    pub const DARKSLATEGRAY: Self = Self { r: 47, g: 79, b: 79, a: () };
    /// <span style="color:#2F4F4F">darkslategrey: #2F4F4F</span>
    pub const DARKSLATEGREY: Self = Self { r: 47, g: 79, b: 79, a: () };
    /// <span style="color:#00CED1">darkturquoise: #00CED1</span>
    pub const DARKTURQUOISE: Self = Self { r: 0, g: 206, b: 209, a: () };
    /// <span style="color:#9400D3">darkviolet: #9400D3</span>
    pub const DARKVIOLET: Self = Self { r: 148, g: 0, b: 211, a: () };
    /// <span style="color:#FF1493">deeppink: #FF1493</span>
    pub const DEEPPINK: Self = Self { r: 255, g: 20, b: 147, a: () };
    /// <span style="color:#00BFFF">deepskyblue: #00BFFF</span>
    pub const DEEPSKYBLUE: Self = Self { r: 0, g: 191, b: 255, a: () };
    /// <span style="color:#696969">dimgray: #696969</span>
    pub const DIMGRAY: Self = Self { r: 105, g: 105, b: 105, a: () };
    /// <span style="color:#696969">dimgrey: #696969</span>
    pub const DIMGREY: Self = Self { r: 105, g: 105, b: 105, a: () };
    /// <span style="color:#1E90FF">dodgerblue: #1E90FF</span>
    pub const DODGERBLUE: Self = Self { r: 30, g: 144, b: 255, a: () };
    /// <span style="color:#B22222">firebrick: #B22222</span>
    pub const FIREBRICK: Self = Self { r: 178, g: 34, b: 34, a: () };
    /// <span style="color:#FFFAF0">floralwhite: #FFFAF0</span>
    pub const FLORALWHITE: Self = Self { r: 255, g: 250, b: 240, a: () };
    /// <span style="color:#228B22">forestgreen: #228B22</span>
    pub const FORESTGREEN: Self = Self { r: 34, g: 139, b: 34, a: () };
    /// <span style="color:#FF00FF">fuchsia: #FF00FF</span>
    pub const FUCHSIA: Self = Self { r: 255, g: 0, b: 255, a: () };
    /// <span style="color:#DCDCDC">gainsboro: #DCDCDC</span>
    pub const GAINSBORO: Self = Self { r: 220, g: 220, b: 220, a: () };
    /// <span style="color:#F8F8FF">ghostwhite: #F8F8FF</span>
    pub const GHOSTWHITE: Self = Self { r: 248, g: 248, b: 255, a: () };
    /// <span style="color:#DAA520">goldenrod: #DAA520</span>
    pub const GOLDENROD: Self = Self { r: 218, g: 165, b: 32, a: () };
    /// <span style="color:#FFD700">gold: #FFD700</span>
    pub const GOLD: Self = Self { r: 255, g: 215, b: 0, a: () };
    /// <span style="color:#808080">gray: #808080</span>
    pub const GRAY: Self = Self { r: 128, g: 128, b: 128, a: () };
    /// <span style="color:#008000">green: #008000</span>
    pub const GREEN: Self = Self { r: 0, g: 128, b: 0, a: () };
    /// <span style="color:#ADFF2F">greenyellow: #ADFF2F</span>
    pub const GREENYELLOW: Self = Self { r: 173, g: 255, b: 47, a: () };
    /// <span style="color:#808080">grey: #808080</span>
    pub const GREY: Self = Self { r: 128, g: 128, b: 128, a: () };
    /// <span style="color:#F0FFF0">honeydew: #F0FFF0</span>
    pub const HONEYDEW: Self = Self { r: 240, g: 255, b: 240, a: () };
    /// <span style="color:#FF69B4">hotpink: #FF69B4</span>
    pub const HOTPINK: Self = Self { r: 255, g: 105, b: 180, a: () };
    /// <span style="color:#CD5C5C">indianred: #CD5C5C</span>
    pub const INDIANRED: Self = Self { r: 205, g: 92, b: 92, a: () };
    /// <span style="color:#4B0082">indigo: #4B0082</span>
    pub const INDIGO: Self = Self { r: 75, g: 0, b: 130, a: () };
    /// <span style="color:#FFFFF0">ivory: #FFFFF0</span>
    pub const IVORY: Self = Self { r: 255, g: 255, b: 240, a: () };
    /// <span style="color:#F0E68C">khaki: #F0E68C</span>
    pub const KHAKI: Self = Self { r: 240, g: 230, b: 140, a: () };
    /// <span style="color:#FFF0F5">lavenderblush: #FFF0F5</span>
    pub const LAVENDERBLUSH: Self = Self { r: 255, g: 240, b: 245, a: () };
    /// <span style="color:#E6E6FA">lavender: #E6E6FA</span>
    pub const LAVENDER: Self = Self { r: 230, g: 230, b: 250, a: () };
    /// <span style="color:#7CFC00">lawngreen: #7CFC00</span>
    pub const LAWNGREEN: Self = Self { r: 124, g: 252, b: 0, a: () };
    /// <span style="color:#FFFACD">lemonchiffon: #FFFACD</span>
    pub const LEMONCHIFFON: Self = Self { r: 255, g: 250, b: 205, a: () };
    /// <span style="color:#ADD8E6">lightblue: #ADD8E6</span>
    pub const LIGHTBLUE: Self = Self { r: 173, g: 216, b: 230, a: () };
    /// <span style="color:#F08080">lightcoral: #F08080</span>
    pub const LIGHTCORAL: Self = Self { r: 240, g: 128, b: 128, a: () };
    /// <span style="color:#E0FFFF">lightcyan: #E0FFFF</span>
    pub const LIGHTCYAN: Self = Self { r: 224, g: 255, b: 255, a: () };
    /// <span style="color:#FAFAD2">lightgoldenrodyellow: #FAFAD2</span>
    pub const LIGHTGOLDENRODYELLOW: Self = Self { r: 250, g: 250, b: 210, a: () };
    /// <span style="color:#D3D3D3">lightgray: #D3D3D3</span>
    pub const LIGHTGRAY: Self = Self { r: 211, g: 211, b: 211, a: () };
    /// <span style="color:#90EE90">lightgreen: #90EE90</span>
    pub const LIGHTGREEN: Self = Self { r: 144, g: 238, b: 144, a: () };
    /// <span style="color:#D3D3D3">lightgrey: #D3D3D3</span>
    pub const LIGHTGREY: Self = Self { r: 211, g: 211, b: 211, a: () };
    /// <span style="color:#FFB6C1">lightpink: #FFB6C1</span>
    pub const LIGHTPINK: Self = Self { r: 255, g: 182, b: 193, a: () };
    /// <span style="color:#FFA07A">lightsalmon: #FFA07A</span>
    pub const LIGHTSALMON: Self = Self { r: 255, g: 160, b: 122, a: () };
    /// <span style="color:#20B2AA">lightseagreen: #20B2AA</span>
    pub const LIGHTSEAGREEN: Self = Self { r: 32, g: 178, b: 170, a: () };
    /// <span style="color:#87CEFA">lightskyblue: #87CEFA</span>
    pub const LIGHTSKYBLUE: Self = Self { r: 135, g: 206, b: 250, a: () };
    /// <span style="color:#778899">lightslategray: #778899</span>
    pub const LIGHTSLATEGRAY: Self = Self { r: 119, g: 136, b: 153, a: () };
    /// <span style="color:#778899">lightslategrey: #778899</span>
    pub const LIGHTSLATEGREY: Self = Self { r: 119, g: 136, b: 153, a: () };
    /// <span style="color:#B0C4DE">lightsteelblue: #B0C4DE</span>
    pub const LIGHTSTEELBLUE: Self = Self { r: 176, g: 196, b: 222, a: () };
    /// <span style="color:#FFFFE0">lightyellow: #FFFFE0</span>
    pub const LIGHTYELLOW: Self = Self { r: 255, g: 255, b: 224, a: () };
    /// <span style="color:#00FF00">lime: #00FF00</span>
    pub const LIME: Self = Self { r: 0, g: 255, b: 0, a: () };
    /// <span style="color:#32CD32">limegreen: #32CD32</span>
    pub const LIMEGREEN: Self = Self { r: 50, g: 205, b: 50, a: () };
    /// <span style="color:#FAF0E6">linen: #FAF0E6</span>
    pub const LINEN: Self = Self { r: 250, g: 240, b: 230, a: () };
    /// <span style="color:#FF00FF">magenta: #FF00FF</span>
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255, a: () };
    /// <span style="color:#800000">maroon: #800000</span>
    pub const MAROON: Self = Self { r: 128, g: 0, b: 0, a: () };
    /// <span style="color:#66CDAA">mediumaquamarine: #66CDAA</span>
    pub const MEDIUMAQUAMARINE: Self = Self { r: 102, g: 205, b: 170, a: () };
    /// <span style="color:#0000CD">mediumblue: #0000CD</span>
    pub const MEDIUMBLUE: Self = Self { r: 0, g: 0, b: 205, a: () };
    /// <span style="color:#BA55D3">mediumorchid: #BA55D3</span>
    pub const MEDIUMORCHID: Self = Self { r: 186, g: 85, b: 211, a: () };
    /// <span style="color:#9370DB">mediumpurple: #9370DB</span>
    pub const MEDIUMPURPLE: Self = Self { r: 147, g: 112, b: 219, a: () };
    /// <span style="color:#3CB371">mediumseagreen: #3CB371</span>
    pub const MEDIUMSEAGREEN: Self = Self { r: 60, g: 179, b: 113, a: () };
    /// <span style="color:#7B68EE">mediumslateblue: #7B68EE</span>
    pub const MEDIUMSLATEBLUE: Self = Self { r: 123, g: 104, b: 238, a: () };
    /// <span style="color:#00FA9A">mediumspringgreen: #00FA9A</span>
    pub const MEDIUMSPRINGGREEN: Self = Self { r: 0, g: 250, b: 154, a: () };
    /// <span style="color:#48D1CC">mediumturquoise: #48D1CC</span>
    pub const MEDIUMTURQUOISE: Self = Self { r: 72, g: 209, b: 204, a: () };
    /// <span style="color:#C71585">mediumvioletred: #C71585</span>
    pub const MEDIUMVIOLETRED: Self = Self { r: 199, g: 21, b: 133, a: () };
    /// <span style="color:#191970">midnightblue: #191970</span>
    pub const MIDNIGHTBLUE: Self = Self { r: 25, g: 25, b: 112, a: () };
    /// <span style="color:#F5FFFA">mintcream: #F5FFFA</span>
    pub const MINTCREAM: Self = Self { r: 245, g: 255, b: 250, a: () };
    /// <span style="color:#FFE4E1">mistyrose: #FFE4E1</span>
    pub const MISTYROSE: Self = Self { r: 255, g: 228, b: 225, a: () };
    /// <span style="color:#FFE4B5">moccasin: #FFE4B5</span>
    pub const MOCCASIN: Self = Self { r: 255, g: 228, b: 181, a: () };
    /// <span style="color:#FFDEAD">navajowhite: #FFDEAD</span>
    pub const NAVAJOWHITE: Self = Self { r: 255, g: 222, b: 173, a: () };
    /// <span style="color:#000080">navy: #000080</span>
    pub const NAVY: Self = Self { r: 0, g: 0, b: 128, a: () };
    /// <span style="color:#FDF5E6">oldlace: #FDF5E6</span>
    pub const OLDLACE: Self = Self { r: 253, g: 245, b: 230, a: () };
    /// <span style="color:#808000">olive: #808000</span>
    pub const OLIVE: Self = Self { r: 128, g: 128, b: 0, a: () };
    /// <span style="color:#6B8E23">olivedrab: #6B8E23</span>
    pub const OLIVEDRAB: Self = Self { r: 107, g: 142, b: 35, a: () };
    /// <span style="color:#FFA500">orange: #FFA500</span>
    pub const ORANGE: Self = Self { r: 255, g: 165, b: 0, a: () };
    /// <span style="color:#FF4500">orangered: #FF4500</span>
    pub const ORANGERED: Self = Self { r: 255, g: 69, b: 0, a: () };
    /// <span style="color:#DA70D6">orchid: #DA70D6</span>
    pub const ORCHID: Self = Self { r: 218, g: 112, b: 214, a: () };
    /// <span style="color:#EEE8AA">palegoldenrod: #EEE8AA</span>
    pub const PALEGOLDENROD: Self = Self { r: 238, g: 232, b: 170, a: () };
    /// <span style="color:#98FB98">palegreen: #98FB98</span>
    pub const PALEGREEN: Self = Self { r: 152, g: 251, b: 152, a: () };
    /// <span style="color:#AFEEEE">paleturquoise: #AFEEEE</span>
    pub const PALETURQUOISE: Self = Self { r: 175, g: 238, b: 238, a: () };
    /// <span style="color:#DB7093">palevioletred: #DB7093</span>
    pub const PALEVIOLETRED: Self = Self { r: 219, g: 112, b: 147, a: () };
    /// <span style="color:#FFEFD5">papayawhip: #FFEFD5</span>
    pub const PAPAYAWHIP: Self = Self { r: 255, g: 239, b: 213, a: () };
    /// <span style="color:#FFDAB9">peachpuff: #FFDAB9</span>
    pub const PEACHPUFF: Self = Self { r: 255, g: 218, b: 185, a: () };
    /// <span style="color:#CD853F">peru: #CD853F</span>
    pub const PERU: Self = Self { r: 205, g: 133, b: 63, a: () };
    /// <span style="color:#FFC0CB">pink: #FFC0CB</span>
    pub const PINK: Self = Self { r: 255, g: 192, b: 203, a: () };
    /// <span style="color:#DDA0DD">plum: #DDA0DD</span>
    pub const PLUM: Self = Self { r: 221, g: 160, b: 221, a: () };
    /// <span style="color:#B0E0E6">powderblue: #B0E0E6</span>
    pub const POWDERBLUE: Self = Self { r: 176, g: 224, b: 230, a: () };
    /// <span style="color:#800080">purple: #800080</span>
    pub const PURPLE: Self = Self { r: 128, g: 0, b: 128, a: () };
    /// <span style="color:#663399">rebeccapurple: #663399</span>
    pub const REBECCAPURPLE: Self = Self { r: 102, g: 51, b: 153, a: () };
    /// <span style="color:#FF0000">red: #FF0000</span>
    pub const RED: Self = Self { r: 255, g: 0, b: 0, a: () };
    /// <span style="color:#BC8F8F">rosybrown: #BC8F8F</span>
    pub const ROSYBROWN: Self = Self { r: 188, g: 143, b: 143, a: () };
    /// <span style="color:#4169E1">royalblue: #4169E1</span>
    pub const ROYALBLUE: Self = Self { r: 65, g: 105, b: 225, a: () };
    /// <span style="color:#8B4513">saddlebrown: #8B4513</span>
    pub const SADDLEBROWN: Self = Self { r: 139, g: 69, b: 19, a: () };
    /// <span style="color:#FA8072">salmon: #FA8072</span>
    pub const SALMON: Self = Self { r: 250, g: 128, b: 114, a: () };
    /// <span style="color:#F4A460">sandybrown: #F4A460</span>
    pub const SANDYBROWN: Self = Self { r: 244, g: 164, b: 96, a: () };
    /// <span style="color:#2E8B57">seagreen: #2E8B57</span>
    pub const SEAGREEN: Self = Self { r: 46, g: 139, b: 87, a: () };
    /// <span style="color:#FFF5EE">seashell: #FFF5EE</span>
    pub const SEASHELL: Self = Self { r: 255, g: 245, b: 238, a: () };
    /// <span style="color:#A0522D">sienna: #A0522D</span>
    pub const SIENNA: Self = Self { r: 160, g: 82, b: 45, a: () };
    /// <span style="color:#C0C0C0">silver: #C0C0C0</span>
    pub const SILVER: Self = Self { r: 192, g: 192, b: 192, a: () };
    /// <span style="color:#87CEEB">skyblue: #87CEEB</span>
    pub const SKYBLUE: Self = Self { r: 135, g: 206, b: 235, a: () };
    /// <span style="color:#6A5ACD">slateblue: #6A5ACD</span>
    pub const SLATEBLUE: Self = Self { r: 106, g: 90, b: 205, a: () };
    /// <span style="color:#708090">slategray: #708090</span>
    pub const SLATEGRAY: Self = Self { r: 112, g: 128, b: 144, a: () };
    /// <span style="color:#708090">slategrey: #708090</span>
    pub const SLATEGREY: Self = Self { r: 112, g: 128, b: 144, a: () };
    /// <span style="color:#FFFAFA">snow: #FFFAFA</span>
    pub const SNOW: Self = Self { r: 255, g: 250, b: 250, a: () };
    /// <span style="color:#00FF7F">springgreen: #00FF7F</span>
    pub const SPRINGGREEN: Self = Self { r: 0, g: 255, b: 127, a: () };
    /// <span style="color:#4682B4">steelblue: #4682B4</span>
    pub const STEELBLUE: Self = Self { r: 70, g: 130, b: 180, a: () };
    /// <span style="color:#D2B48C">tan: #D2B48C</span>
    pub const TAN: Self = Self { r: 210, g: 180, b: 140, a: () };
    /// <span style="color:#008080">teal: #008080</span>
    pub const TEAL: Self = Self { r: 0, g: 128, b: 128, a: () };
    /// <span style="color:#D8BFD8">thistle: #D8BFD8</span>
    pub const THISTLE: Self = Self { r: 216, g: 191, b: 216, a: () };
    /// <span style="color:#FF6347">tomato: #FF6347</span>
    pub const TOMATO: Self = Self { r: 255, g: 99, b: 71, a: () };
    /// <span style="color:#40E0D0">turquoise: #40E0D0</span>
    pub const TURQUOISE: Self = Self { r: 64, g: 224, b: 208, a: () };
    /// <span style="color:#EE82EE">violet: #EE82EE</span>
    pub const VIOLET: Self = Self { r: 238, g: 130, b: 238, a: () };
    /// <span style="color:#F5DEB3">wheat: #F5DEB3</span>
    pub const WHEAT: Self = Self { r: 245, g: 222, b: 179, a: () };
    /// <span style="color:#FFFFFF">white: #FFFFFF</span>
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: () };
    /// <span style="color:#F5F5F5">whitesmoke: #F5F5F5</span>
    pub const WHITESMOKE: Self = Self { r: 245, g: 245, b: 245, a: () };
    /// <span style="color:#FFFF00">yellow: #FFFF00</span>
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0, a: () };
    /// <span style="color:#9ACD32">yellowgreen: #9ACD32</span>
    pub const YELLOWGREEN: Self = Self { r: 154, g: 205, b: 50, a: () };
}
