use super::*;

impl RGB {
    /// <view style="color:aliceblue">aliceblue: #F0F8FF</view>
    pub const ALICEBLUE: Self = Self { r: 240, g: 248, b: 255 };

    /// <view style="color:antiquewhite">antiquewhite: #FAEBD7</view>
    pub const ANTIQUEWHITE: Self = Self { r: 250, g: 235, b: 215 };

    /// <view style="color:aqua">aqua: #00FFFF</view>
    pub const AQUA: Self = Self { r: 0, g: 255, b: 255 };

    /// <view style="color:aquamarine">aquamarine: #7FFFD4</view>
    pub const AQUAMARINE: Self = Self { r: 127, g: 255, b: 212 };

    /// <view style="color:azure">azure: #F0FFFF</view>
    pub const AZURE: Self = Self { r: 240, g: 255, b: 255 };

    /// <view style="color:beige">beige: #F5F5DC</view>
    pub const BEIGE: Self = Self { r: 245, g: 245, b: 220 };

    /// <view style="color:bisque">bisque: #FFE4C4</view>
    pub const BISQUE: Self = Self { r: 255, g: 228, b: 196 };

    /// <view style="color:black">black: #000000</view>
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };

    /// <view style="color:blanchedalmond">blanchedalmond: #FFEBCD</view>
    pub const BLANCHEDALMOND: Self = Self { r: 255, g: 235, b: 205 };

    /// <view style="color:blue">blue: #0000FF</view>
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };

    /// <view style="color:blueviolet">blueviolet: #8A2BE2</view>
    pub const BLUEVIOLET: Self = Self { r: 138, g: 43, b: 226 };

    /// <view style="color:brown">brown: #A52A2A</view>
    pub const BROWN: Self = Self { r: 165, g: 42, b: 42 };

    /// <view style="color:burlywood">burlywood: #DEB887</view>
    pub const BURLYWOOD: Self = Self { r: 222, g: 184, b: 135 };

    /// <view style="color:cadetblue">cadetblue: #5F9EA0</view>
    pub const CADETBLUE: Self = Self { r: 95, g: 158, b: 160 };

    /// <view style="color:chartreuse">chartreuse: #7FFF00</view>
    pub const CHARTREUSE: Self = Self { r: 127, g: 255, b: 0 };

    /// <view style="color:chocolate">chocolate: #D2691E</view>
    pub const CHOCOLATE: Self = Self { r: 210, g: 105, b: 30 };

    /// <view style="color:coral">coral: #FF7F50</view>
    pub const CORAL: Self = Self { r: 255, g: 127, b: 80 };

    /// <view style="color:cornflowerblue">cornflowerblue: #6495ED</view>
    pub const CORNFLOWERBLUE: Self = Self { r: 100, g: 149, b: 237 };

    /// <view style="color:cornsilk">cornsilk: #FFF8DC</view>
    pub const CORNSILK: Self = Self { r: 255, g: 248, b: 220 };

    /// <view style="color:crimson">crimson: #DC143C</view>
    pub const CRIMSON: Self = Self { r: 220, g: 20, b: 60 };

    /// <view style="color:cyan">cyan: #00FFFF</view>
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255 };

    /// <view style="color:darkblue">darkblue: #00008B</view>
    pub const DARKBLUE: Self = Self { r: 0, g: 0, b: 139 };

    /// <view style="color:darkcyan">darkcyan: #008B8B</view>
    pub const DARKCYAN: Self = Self { r: 0, g: 139, b: 139 };

    /// <view style="color:darkgoldenrod">darkgoldenrod: #B8860B</view>
    pub const DARKGOLDENROD: Self = Self { r: 184, g: 134, b: 11 };

    /// <view style="color:darkgray">darkgray: #A9A9A9</view>
    pub const DARKGRAY: Self = Self { r: 169, g: 169, b: 169 };

    /// <view style="color:darkgreen">darkgreen: #006400</view>
    pub const DARKGREEN: Self = Self { r: 0, g: 100, b: 0 };

    /// <view style="color:darkgrey">darkgrey: #A9A9A9</view>
    pub const DARKGREY: Self = Self { r: 169, g: 169, b: 169 };

    /// <view style="color:darkkhaki">darkkhaki: #BDB76B</view>
    pub const DARKKHAKI: Self = Self { r: 189, g: 183, b: 107 };

    /// <view style="color:darkmagenta">darkmagenta: #8B008B</view>
    pub const DARKMAGENTA: Self = Self { r: 139, g: 0, b: 139 };

    /// <view style="color:darkolivegreen">darkolivegreen: #556B2F</view>
    pub const DARKOLIVEGREEN: Self = Self { r: 85, g: 107, b: 47 };

    /// <view style="color:darkorange">darkorange: #FF8C00</view>
    pub const DARKORANGE: Self = Self { r: 255, g: 140, b: 0 };

    /// <view style="color:darkorchid">darkorchid: #9932CC</view>
    pub const DARKORCHID: Self = Self { r: 153, g: 50, b: 204 };

    /// <view style="color:darkred">darkred: #8B0000</view>
    pub const DARKRED: Self = Self { r: 139, g: 0, b: 0 };

    /// <view style="color:darksalmon">darksalmon: #E9967A</view>
    pub const DARKSALMON: Self = Self { r: 233, g: 150, b: 122 };

    /// <view style="color:darkseagreen">darkseagreen: #8FBC8F</view>
    pub const DARKSEAGREEN: Self = Self { r: 143, g: 188, b: 143 };

    /// <view style="color:darkslateblue">darkslateblue: #483D8B</view>
    pub const DARKSLATEBLUE: Self = Self { r: 72, g: 61, b: 139 };

    /// <view style="color:darkslategray">darkslategray: #2F4F4F</view>
    pub const DARKSLATEGRAY: Self = Self { r: 47, g: 79, b: 79 };

    /// <view style="color:darkslategrey">darkslategrey: #2F4F4F</view>
    pub const DARKSLATEGREY: Self = Self { r: 47, g: 79, b: 79 };

    /// <view style="color:darkturquoise">darkturquoise: #00CED1</view>
    pub const DARKTURQUOISE: Self = Self { r: 0, g: 206, b: 209 };

    /// <view style="color:darkviolet">darkviolet: #9400D3</view>
    pub const DARKVIOLET: Self = Self { r: 148, g: 0, b: 211 };

    /// <view style="color:deeppink">deeppink: #FF1493</view>
    pub const DEEPPINK: Self = Self { r: 255, g: 20, b: 147 };

    /// <view style="color:deepskyblue">deepskyblue: #00BFFF</view>
    pub const DEEPSKYBLUE: Self = Self { r: 0, g: 191, b: 255 };

    /// <view style="color:dimgray">dimgray: #696969</view>
    pub const DIMGRAY: Self = Self { r: 105, g: 105, b: 105 };

    /// <view style="color:dimgrey">dimgrey: #696969</view>
    pub const DIMGREY: Self = Self { r: 105, g: 105, b: 105 };

    /// <view style="color:dodgerblue">dodgerblue: #1E90FF</view>
    pub const DODGERBLUE: Self = Self { r: 30, g: 144, b: 255 };

    /// <view style="color:firebrick">firebrick: #B22222</view>
    pub const FIREBRICK: Self = Self { r: 178, g: 34, b: 34 };

    /// <view style="color:floralwhite">floralwhite: #FFFAF0</view>
    pub const FLORALWHITE: Self = Self { r: 255, g: 250, b: 240 };

    /// <view style="color:forestgreen">forestgreen: #228B22</view>
    pub const FORESTGREEN: Self = Self { r: 34, g: 139, b: 34 };

    /// <view style="color:fuchsia">fuchsia: #FF00FF</view>
    pub const FUCHSIA: Self = Self { r: 255, g: 0, b: 255 };

    /// <view style="color:gainsboro">gainsboro: #DCDCDC</view>
    pub const GAINSBORO: Self = Self { r: 220, g: 220, b: 220 };

    /// <view style="color:ghostwhite">ghostwhite: #F8F8FF</view>
    pub const GHOSTWHITE: Self = Self { r: 248, g: 248, b: 255 };

    /// <view style="color:goldenrod">goldenrod: #DAA520</view>
    pub const GOLDENROD: Self = Self { r: 218, g: 165, b: 32 };

    /// <view style="color:gold">gold: #FFD700</view>
    pub const GOLD: Self = Self { r: 255, g: 215, b: 0 };

    /// <view style="color:gray">gray: #808080</view>
    pub const GRAY: Self = Self { r: 128, g: 128, b: 128 };

    /// <view style="color:green">green: #008000</view>
    pub const GREEN: Self = Self { r: 0, g: 128, b: 0 };

    /// <view style="color:greenyellow">greenyellow: #ADFF2F</view>
    pub const GREENYELLOW: Self = Self { r: 173, g: 255, b: 47 };

    /// <view style="color:grey">grey: #808080</view>
    pub const GREY: Self = Self { r: 128, g: 128, b: 128 };

    /// <view style="color:honeydew">honeydew: #F0FFF0</view>
    pub const HONEYDEW: Self = Self { r: 240, g: 255, b: 240 };

    /// <view style="color:hotpink">hotpink: #FF69B4</view>
    pub const HOTPINK: Self = Self { r: 255, g: 105, b: 180 };

    /// <view style="color:indianred">indianred: #CD5C5C</view>
    pub const INDIANRED: Self = Self { r: 205, g: 92, b: 92 };

    /// <view style="color:indigo">indigo: #4B0082</view>
    pub const INDIGO: Self = Self { r: 75, g: 0, b: 130 };

    /// <view style="color:ivory">ivory: #FFFFF0</view>
    pub const IVORY: Self = Self { r: 255, g: 255, b: 240 };

    /// <view style="color:khaki">khaki: #F0E68C</view>
    pub const KHAKI: Self = Self { r: 240, g: 230, b: 140 };

    /// <view style="color:lavenderblush">lavenderblush: #FFF0F5</view>
    pub const LAVENDERBLUSH: Self = Self { r: 255, g: 240, b: 245 };

    /// <view style="color:lavender">lavender: #E6E6FA</view>
    pub const LAVENDER: Self = Self { r: 230, g: 230, b: 250 };

    /// <view style="color:lawngreen">lawngreen: #7CFC00</view>
    pub const LAWNGREEN: Self = Self { r: 124, g: 252, b: 0 };

    /// <view style="color:lemonchiffon">lemonchiffon: #FFFACD</view>
    pub const LEMONCHIFFON: Self = Self { r: 255, g: 250, b: 205 };

    /// <view style="color:lightblue">lightblue: #ADD8E6</view>
    pub const LIGHTBLUE: Self = Self { r: 173, g: 216, b: 230 };

    /// <view style="color:lightcoral">lightcoral: #F08080</view>
    pub const LIGHTCORAL: Self = Self { r: 240, g: 128, b: 128 };

    /// <view style="color:lightcyan">lightcyan: #E0FFFF</view>
    pub const LIGHTCYAN: Self = Self { r: 224, g: 255, b: 255 };

    /// <view style="color:lightgoldenrodyellow">lightgoldenrodyellow: #FAFAD2</view>
    pub const LIGHTGOLDENRODYELLOW: Self = Self { r: 250, g: 250, b: 210 };

    /// <view style="color:lightgray">lightgray: #D3D3D3</view>
    pub const LIGHTGRAY: Self = Self { r: 211, g: 211, b: 211 };

    /// <view style="color:lightgreen">lightgreen: #90EE90</view>
    pub const LIGHTGREEN: Self = Self { r: 144, g: 238, b: 144 };

    /// <view style="color:lightgrey">lightgrey: #D3D3D3</view>
    pub const LIGHTGREY: Self = Self { r: 211, g: 211, b: 211 };

    /// <view style="color:lightpink">lightpink: #FFB6C1</view>
    pub const LIGHTPINK: Self = Self { r: 255, g: 182, b: 193 };

    /// <view style="color:lightsalmon">lightsalmon: #FFA07A</view>
    pub const LIGHTSALMON: Self = Self { r: 255, g: 160, b: 122 };

    /// <view style="color:lightseagreen">lightseagreen: #20B2AA</view>
    pub const LIGHTSEAGREEN: Self = Self { r: 32, g: 178, b: 170 };

    /// <view style="color:lightskyblue">lightskyblue: #87CEFA</view>
    pub const LIGHTSKYBLUE: Self = Self { r: 135, g: 206, b: 250 };

    /// <view style="color:lightslategray">lightslategray: #778899</view>
    pub const LIGHTSLATEGRAY: Self = Self { r: 119, g: 136, b: 153 };

    /// <view style="color:lightslategrey">lightslategrey: #778899</view>
    pub const LIGHTSLATEGREY: Self = Self { r: 119, g: 136, b: 153 };

    /// <view style="color:lightsteelblue">lightsteelblue: #B0C4DE</view>
    pub const LIGHTSTEELBLUE: Self = Self { r: 176, g: 196, b: 222 };

    /// <view style="color:lightyellow">lightyellow: #FFFFE0</view>
    pub const LIGHTYELLOW: Self = Self { r: 255, g: 255, b: 224 };

    /// <view style="color:lime">lime: #00FF00</view>
    pub const LIME: Self = Self { r: 0, g: 255, b: 0 };

    /// <view style="color:limegreen">limegreen: #32CD32</view>
    pub const LIMEGREEN: Self = Self { r: 50, g: 205, b: 50 };

    /// <view style="color:linen">linen: #FAF0E6</view>
    pub const LINEN: Self = Self { r: 250, g: 240, b: 230 };

    /// <view style="color:magenta">magenta: #FF00FF</view>
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255 };

    /// <view style="color:maroon">maroon: #800000</view>
    pub const MAROON: Self = Self { r: 128, g: 0, b: 0 };

    /// <view style="color:mediumaquamarine">mediumaquamarine: #66CDAA</view>
    pub const MEDIUMAQUAMARINE: Self = Self { r: 102, g: 205, b: 170 };

    /// <view style="color:mediumblue">mediumblue: #0000CD</view>
    pub const MEDIUMBLUE: Self = Self { r: 0, g: 0, b: 205 };

    /// <view style="color:mediumorchid">mediumorchid: #BA55D3</view>
    pub const MEDIUMORCHID: Self = Self { r: 186, g: 85, b: 211 };

    /// <view style="color:mediumpurple">mediumpurple: #9370DB</view>
    pub const MEDIUMPURPLE: Self = Self { r: 147, g: 112, b: 219 };

    /// <view style="color:mediumseagreen">mediumseagreen: #3CB371</view>
    pub const MEDIUMSEAGREEN: Self = Self { r: 60, g: 179, b: 113 };

    /// <view style="color:mediumslateblue">mediumslateblue: #7B68EE</view>
    pub const MEDIUMSLATEBLUE: Self = Self { r: 123, g: 104, b: 238 };

    /// <view style="color:mediumspringgreen">mediumspringgreen: #00FA9A</view>
    pub const MEDIUMSPRINGGREEN: Self = Self { r: 0, g: 250, b: 154 };

    /// <view style="color:mediumturquoise">mediumturquoise: #48D1CC</view>
    pub const MEDIUMTURQUOISE: Self = Self { r: 72, g: 209, b: 204 };

    /// <view style="color:mediumvioletred">mediumvioletred: #C71585</view>
    pub const MEDIUMVIOLETRED: Self = Self { r: 199, g: 21, b: 133 };

    /// <view style="color:midnightblue">midnightblue: #191970</view>
    pub const MIDNIGHTBLUE: Self = Self { r: 25, g: 25, b: 112 };

    /// <view style="color:mintcream">mintcream: #F5FFFA</view>
    pub const MINTCREAM: Self = Self { r: 245, g: 255, b: 250 };

    /// <view style="color:mistyrose">mistyrose: #FFE4E1</view>
    pub const MISTYROSE: Self = Self { r: 255, g: 228, b: 225 };

    /// <view style="color:moccasin">moccasin: #FFE4B5</view>
    pub const MOCCASIN: Self = Self { r: 255, g: 228, b: 181 };

    /// <view style="color:navajowhite">navajowhite: #FFDEAD</view>
    pub const NAVAJOWHITE: Self = Self { r: 255, g: 222, b: 173 };

    /// <view style="color:navy">navy: #000080</view>
    pub const NAVY: Self = Self { r: 0, g: 0, b: 128 };

    /// <view style="color:oldlace">oldlace: #FDF5E6</view>
    pub const OLDLACE: Self = Self { r: 253, g: 245, b: 230 };

    /// <view style="color:olive">olive: #808000</view>
    pub const OLIVE: Self = Self { r: 128, g: 128, b: 0 };

    /// <view style="color:olivedrab">olivedrab: #6B8E23</view>
    pub const OLIVEDRAB: Self = Self { r: 107, g: 142, b: 35 };

    /// <view style="color:orange">orange: #FFA500</view>
    pub const ORANGE: Self = Self { r: 255, g: 165, b: 0 };

    /// <view style="color:orangered">orangered: #FF4500</view>
    pub const ORANGERED: Self = Self { r: 255, g: 69, b: 0 };

    /// <view style="color:orchid">orchid: #DA70D6</view>
    pub const ORCHID: Self = Self { r: 218, g: 112, b: 214 };

    /// <view style="color:palegoldenrod">palegoldenrod: #EEE8AA</view>
    pub const PALEGOLDENROD: Self = Self { r: 238, g: 232, b: 170 };

    /// <view style="color:palegreen">palegreen: #98FB98</view>
    pub const PALEGREEN: Self = Self { r: 152, g: 251, b: 152 };

    /// <view style="color:paleturquoise">paleturquoise: #AFEEEE</view>
    pub const PALETURQUOISE: Self = Self { r: 175, g: 238, b: 238 };

    /// <view style="color:palevioletred">palevioletred: #DB7093</view>
    pub const PALEVIOLETRED: Self = Self { r: 219, g: 112, b: 147 };

    /// <view style="color:papayawhip">papayawhip: #FFEFD5</view>
    pub const PAPAYAWHIP: Self = Self { r: 255, g: 239, b: 213 };

    /// <view style="color:peachpuff">peachpuff: #FFDAB9</view>
    pub const PEACHPUFF: Self = Self { r: 255, g: 218, b: 185 };

    /// <view style="color:peru">peru: #CD853F</view>
    pub const PERU: Self = Self { r: 205, g: 133, b: 63 };

    /// <view style="color:pink">pink: #FFC0CB</view>
    pub const PINK: Self = Self { r: 255, g: 192, b: 203 };

    /// <view style="color:plum">plum: #DDA0DD</view>
    pub const PLUM: Self = Self { r: 221, g: 160, b: 221 };

    /// <view style="color:powderblue">powderblue: #B0E0E6</view>
    pub const POWDERBLUE: Self = Self { r: 176, g: 224, b: 230 };

    /// <view style="color:purple">purple: #800080</view>
    pub const PURPLE: Self = Self { r: 128, g: 0, b: 128 };

    /// <view style="color:rebeccapurple">rebeccapurple: #663399</view>
    pub const REBECCAPURPLE: Self = Self { r: 102, g: 51, b: 153 };

    /// <view style="color:red">red: #FF0000</view>
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };

    /// <view style="color:rosybrown">rosybrown: #BC8F8F</view>
    pub const ROSYBROWN: Self = Self { r: 188, g: 143, b: 143 };

    /// <view style="color:royalblue">royalblue: #4169E1</view>
    pub const ROYALBLUE: Self = Self { r: 65, g: 105, b: 225 };

    /// <view style="color:saddlebrown">saddlebrown: #8B4513</view>
    pub const SADDLEBROWN: Self = Self { r: 139, g: 69, b: 19 };

    /// <view style="color:salmon">salmon: #FA8072</view>
    pub const SALMON: Self = Self { r: 250, g: 128, b: 114 };

    /// <view style="color:sandybrown">sandybrown: #F4A460</view>
    pub const SANDYBROWN: Self = Self { r: 244, g: 164, b: 96 };

    /// <view style="color:seagreen">seagreen: #2E8B57</view>
    pub const SEAGREEN: Self = Self { r: 46, g: 139, b: 87 };

    /// <view style="color:seashell">seashell: #FFF5EE</view>
    pub const SEASHELL: Self = Self { r: 255, g: 245, b: 238 };

    /// <view style="color:sienna">sienna: #A0522D</view>
    pub const SIENNA: Self = Self { r: 160, g: 82, b: 45 };

    /// <view style="color:silver">silver: #C0C0C0</view>
    pub const SILVER: Self = Self { r: 192, g: 192, b: 192 };

    /// <view style="color:skyblue">skyblue: #87CEEB</view>
    pub const SKYBLUE: Self = Self { r: 135, g: 206, b: 235 };

    /// <view style="color:slateblue">slateblue: #6A5ACD</view>
    pub const SLATEBLUE: Self = Self { r: 106, g: 90, b: 205 };

    /// <view style="color:slategray">slategray: #708090</view>
    pub const SLATEGRAY: Self = Self { r: 112, g: 128, b: 144 };

    /// <view style="color:slategrey">slategrey: #708090</view>
    pub const SLATEGREY: Self = Self { r: 112, g: 128, b: 144 };

    /// <view style="color:snow">snow: #FFFAFA</view>
    pub const SNOW: Self = Self { r: 255, g: 250, b: 250 };

    /// <view style="color:springgreen">springgreen: #00FF7F</view>
    pub const SPRINGGREEN: Self = Self { r: 0, g: 255, b: 127 };

    /// <view style="color:steelblue">steelblue: #4682B4</view>
    pub const STEELBLUE: Self = Self { r: 70, g: 130, b: 180 };

    /// <view style="color:tan">tan: #D2B48C</view>
    pub const TAN: Self = Self { r: 210, g: 180, b: 140 };

    /// <view style="color:teal">teal: #008080</view>
    pub const TEAL: Self = Self { r: 0, g: 128, b: 128 };

    /// <view style="color:thistle">thistle: #D8BFD8</view>
    pub const THISTLE: Self = Self { r: 216, g: 191, b: 216 };

    /// <view style="color:tomato">tomato: #FF6347</view>
    pub const TOMATO: Self = Self { r: 255, g: 99, b: 71 };

    /// <view style="color:turquoise">turquoise: #40E0D0</view>
    pub const TURQUOISE: Self = Self { r: 64, g: 224, b: 208 };

    /// <view style="color:violet">violet: #EE82EE</view>
    pub const VIOLET: Self = Self { r: 238, g: 130, b: 238 };

    /// <view style="color:wheat">wheat: #F5DEB3</view>
    pub const WHEAT: Self = Self { r: 245, g: 222, b: 179 };

    /// <view style="color:white">white: #FFFFFF</view>
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };

    /// <view style="color:whitesmoke">whitesmoke: #F5F5F5</view>
    pub const WHITESMOKE: Self = Self { r: 245, g: 245, b: 245 };

    /// <view style="color:yellow">yellow: #FFFF00</view>
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0 };

    /// <view style="color:yellowgreen">yellowgreen: #9ACD32</view>
    pub const YELLOWGREEN: Self = Self { r: 154, g: 205, b: 50 };
}
