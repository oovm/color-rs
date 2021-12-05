Rust Color
==========

**Compile time color parsing!**

## Projects

- **[Color](projects/color-core)**: Parser and syntax tree
- **[Color parser](projects/color-parser)**: Core library, interpreter and validator
  - Does not contain any IO and configuration parser
  - If you're going to do a framework, you should start here
- **[Color macro](projects/color-macro)**: `Tailwind CSS` based post processor, supports html
  - Ready to use with zero configuration
  - Can also be freely configured and combined with CI
