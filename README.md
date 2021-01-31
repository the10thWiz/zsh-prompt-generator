# Zsh bar theme tool

A rust tool to create a customized Zsh prompt

## Dependencies

- A working Rust (nightly) installation
- Some of the symbols used may require powerline [https://github.com/powerline/fonts]

## Usage

The tool can be used to generate a Zsh prompt (in a similar form to Oh-My-Zsh themes).

The example in `example.zsh-theme` runs the tool every time a prompt is needed,
but the output could also just be piped to a file to be used as-is

## Screenshots

My personal Zsh prompt
![My personal Zsh prompt](/screenshots/zsh_prompt.png)

## Installation

If you don't want to run this for every instance of Zsh, just clone the
repository and use `cargo run -- [prompt configuration]`.

## Prompt configuration format

A series of info, each with a specfic color, and seperated by triangles

Configured by the parameter, formatted as
`'(background;foregound)key>'...`

Each section is seperated by a `>`, `<` or `|`, and has the colors specified by
in the parenthesis

Color is passed (unmodified) to `%K{}` and `%F{}`, and the foregound is optional

- `\n` ends the current line, and inserts a literal newline into the prompt
- `>` is expanded to be a trangle, using the approprate colors
- `<` is expanded similarly
- `|` is expanded to nothing, but sperates sections (i.e. it creates a square
  break. it an be used in combination with a newline)
- key is one of the following:
  `$`...: a literal shell function or command
  `%`...: a literal prompt expansion

## Contributing

At this point in time, I do not have any real need to add features. If you think
something is missing, open an issue, or maybe just code it yourself.

If you would like to contribute, feel free to make a pull request. When doing
so, please use rustfmt to format.

