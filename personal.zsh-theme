# Prompt format:
#
# A series of info, each with a specfic color, and seperated by triangles
#
# Configured by $PERSONAL_PROMPT_STRING, formatted as
# '(color)key>'...
# Each section is seperated by a `>`, and has the color specified by (color).
# Color is passed (unmodified) to %K{} and %F{}
# `\n` ends the current line, and inserts a literal newline into the prompt
# `>` is expanded to be a trangle, using the approprate colors
# `<` is expanded similarly
# key is one of the following:
#   `$`...: a literal shell function or command
#   `%`...: a literal prompt expansion
#   username: the username of the current user
#   date`(format)`?: the date, followed by an optional format
#
#
# $psvar is used to add specfic parameters to the final prompt

# User > DIR > git >
# status <
SEPER=$'\ue0b0'
SEP() {
   echo -n "%K{$2}%F{$1}\ue0b0%f"
}

precmd() {
   # Executed before prompt. Gaurenteed to only execute once
   # Init prompt vars
   local git=$($HOME/bin/install/pretty-git-prompt/target/release/pretty-git-prompt -c $HOME/bin/config/pretty-git-prompt-zsh.yml)
   local venv=$(get_venv_name_prompt)
   export psvar=($git $venv)
}

PROMPT=''
PROMPT+="%K{237}%F{223} %n "
PROMPT+="%1(j.$(SEP 237 214)%F{237} %%%j "
PROMPT+="$(SEP 214 208).$(SEP 237 208))%F{237} !%! "
PROMPT+="$(SEP 208 109)%F{237} %4~ "
PROMPT+="$(SEP 109 142)%F{237}%1v"
PROMPT+="$(SEP 142 72)%2v"
PROMPT+="$(SEP 72 0)"
PROMPT+='
'
PROMPT+="%K{241} %(?.%F{250}√.%F{88}%?) %F{0}$(echo -n '\ue0b2')%k%f%(!.#.) "

#'(237;223)%n>(214)?1j;%%%j>(208;237)!%!>(109)%4~>(142)?$($HOME/bin/install/pretty-git-prompt/target/release/pretty-git-prompt -c $HOME/bin/config/pretty-git-prompt-zsh.yml)>(72)?$(get_venv_name_prompt)'
#'(0)\n|(241)??;%F{250}√;%F{88}%?<%k%f%(!.#.)'

# strategy: eval the result of a rust program

