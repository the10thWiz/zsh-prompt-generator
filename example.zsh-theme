#!/bin/zsh

# Optional automatic build
#
# Assumes that the BUILD_DIR exists,
# and the cargo+rust are installed and available
# 
# Define the build directory
BUILD_DIR=$HOME/bin/install/
# only executes build if the theme doesn't exist yet
if [[ ! -x $BUILD_DIR/zsh-theme/target/debug/zsh-theme ]]
then
   if [[ ! -d $BUILD_DIR/zsh-theme ]]
   then
      cd $BUILD_DIR
      git clone https://github.com/the10thWiz/zsh-theme.git
   fi
   cd $BUILD_DIR/zsh-theme
   cargo +nightly build
fi

# Prompt configuration format:
#
# A series of info, each with a specfic color, and seperated by triangles
#
# Configured by $PERSONAL_PROMPT_STRING, formatted as
# '(background;foregound)key>'...
# Each section is seperated by a `>`, and has the color specified by (color).
# Color is passed (unmodified) to %K{} and %F{}, and the foregound is optional
# `\n` ends the current line, and inserts a literal newline into the prompt
# `>` is expanded to be a trangle, using the approprate colors
# `<` is expanded similarly
# `|` is expanded to nothing, but sperates sections (i.e. it creates a square break. it is used below in combination with a newline)
# key is one of the following:
#   `$`...: a literal shell function or command
#   `%`...: a literal prompt expansion
#
#
# $psvar can be used to add specfic parameters to the final prompt

# Username `%n`, white text on black background
TEST_PROMPT='(237;223)%n>'
# Number of jobs `%j`, black on yellow
# `?1j` indicates that this section should only be present when there is atleast
# one running job. `%%` is a literal percent
TEST_PROMPT+='(214)?1j;%%%j>'
# History number `%!`, black on orange
TEST_PROMPT+='(208;237)!%!>'
# Current directory, relative to $HOME, truncated to 4 dirs `%4~`
TEST_PROMPT+='(109;237)%4~>'
# Git: this uses an external tool to generate the one line status of git
#TEST_PROMPT+='(142;237)?$($HOME/bin/install/pretty-git-prompt/target/release/pretty-git-prompt -c $HOME/bin/config/pretty-git-prompt-zsh.yml)>'
# Venv: this uses an external tool to print the directory of the current python venv
#TEST_PROMPT+='(72;237)?$(get_venv_name_prompt)>'
# Date `%D %T`, black on orange
TEST_PROMPT+='(208;237)%D %T>'
# `(~)\n|` ends the current line and resets the background color
# On the next line, there is a setup to print the last status code, or a checkmark
# if the command exited sucessfully
TEST_PROMPT+='(~)\n|(237)??;%F{250}  √;%F{88}%?<'
# reset the prompt to normal, and print a `#` if in an elvated prompt
TEST_PROMPT+='(0)%k%f%(!.#.)'



TMP=$($BUILD_DIR/zsh-theme/target/debug/zsh-theme $TEST_PROMPT )
for L in $TMP; do
   eval $L
done


