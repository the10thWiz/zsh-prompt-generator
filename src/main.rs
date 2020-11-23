#![feature(split_inclusive)]

// Specification:
// '(color)key>'
// Color: ([color][;foreground])? the color, for %K and %F
// Seperator: [<>|], the seperator between parts
// Key: key?, the specified key,
//  $*: a shell variable
//  $(*): a shell function, the output is used
//  %*: a zsh escape
//  ?*;*;*?: a zsh escape, with condition
//  *: a special key
//
//
// Part expansion: ?(cond.%F{new_fore}$(key)%F{old_back}.)%K{new_back}$(sep}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndType {
    RightTriangle,
    LeftTriangle,
    None,
}

impl std::fmt::Display for EndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::RightTriangle => write!(f, "\u{e0b0}"),
            Self::LeftTriangle => write!(f, "\u{e0b2}"),
        }
    }
}

struct Part {
    background: String,
    foreground: String,
    value: String,
    end: EndType,
}

impl Part {
    fn new() -> Self {
        Self {
            background: String::new(),
            foreground: String::new(),
            value: String::new(),
            end: EndType::None,
        }
    }
    fn from_string_int(mut s: &str) -> Self {
        let (background, foreground) = if s.starts_with('(') {
            let idx = s.find(')').expect("No matching parens");
            //let colors = &s[1..idx];
            let tmp = if let Some(sep) = s[0..idx].find(';') {
                (s[1..sep].to_string(), s[sep + 1..idx].to_string())
            } else {
                (s[1..idx].to_string(), String::new())
            };
            s = &s[idx + 1..];
            tmp
        } else {
            (String::new(), String::new())
        };
        let value = if "<>|".contains(s.chars().last().unwrap()) {
            s[..s.len() - 1].to_string()
        } else {
            s.to_string()
        };
        let end = match s.chars().last().expect("String is empty") {
            '<' => EndType::LeftTriangle,
            '>' => EndType::RightTriangle,
            _ => EndType::None,
        };
        Self {
            background,
            foreground,
            value,
            end,
        }
    }
    fn from_string(s: String) -> Vec<Part> {
        s.split_inclusive(|c| "<>|".contains(c))
            .map(|s| Self::from_string_int(s))
            .collect()
    }
    // Part expansion: ?(cond.%F{new_fore}$(key)%F{old_back}.)%K{new_back}$(sep}
    fn to_prompt_format(self, s: &mut Prompt) -> String {
        let (value, condition, value_false) = Self::get_value(&self.value, s);
        let value = Self::expand(value);
        if value == "\n" {
            format!(
                "{end}{fore}{val}{back}",
                end = s.get_last_end(self.end, self.background.clone()),
                fore = s.get_foreground_string(self.foreground),
                val = value,
                back = s.get_foreground_string(self.background),
            )
        } else if condition == "" {
            format!(
                "{end}{fore}{sep}{val}{sep}{back}",
                end = s.get_last_end(self.end, self.background.clone()),
                fore = s.get_foreground_string(self.foreground),
                val = value,
                sep = s.get_seperator(),
                back = s.get_foreground_string(self.background),
            )
        } else if value_false == "" {
            format!(
                "%{cond}.{end}{fore}{sep}{val}{sep}{back}.)",
                cond = condition,
                end = s.get_last_end(self.end, self.background.clone()),
                fore = s.get_foreground_string(self.foreground),
                val = value,
                sep = s.get_seperator(),
                back = s.get_foreground_string(self.background),
            )
        } else {
            format!(
                "{end}{fore}{sep}%{cond}.{val}.{val_false}){sep}{back}",
                cond = condition,
                end = s.get_last_end(self.end, self.background.clone()),
                fore = s.get_foreground_string(self.foreground),
                val = value,
                val_false = value_false,
                sep = s.get_seperator(),
                back = s.get_foreground_string(self.background),
            )
        }
    }
    fn get_value(value: &str, p: &mut Prompt) -> (String, String, String) {
        if value.starts_with("$(") {
            // Need to handle proper expansion
            let num = p.add_psvar(value.to_string());
            (format!("%{}v", num), "".to_string(), "".to_string())
        } else if value.starts_with("?") && value.contains("$(") {
            // Shell variables can be used as is
            let num = p.add_psvar(value[value.find('$').unwrap()..].to_string());
            (
                format!("%{}v", num),
                format!("{}(v", num),
                value[1..value.find('$').unwrap()].to_string(),
            )
        } else if value.starts_with("?") {
            // ? idicates to only show if relevant
            let mut iter = value.split(';');
            let mut cond = iter.next().expect("Value is incomplete")[1..].to_string();
            cond.insert(
                cond.find(|c| !char::is_digit(c, 10))
                    .expect("Malformed condition"),
                '(',
            );
            let val = Self::get_value(iter.next().expect("Value is incomplete"), p).0;
            let false_val = if let Some(val) = iter.next() {
                Self::get_value(val, p).0
            } else {
                "".to_string()
            };
            (val, cond, false_val)
        } else {
            (value.to_string(), "".to_string(), "".to_string())
        }
    }
    fn expand(s: String) -> String {
        s.replace("\\n", "\n")
    }
}

struct Prompt {
    background: String,
    foreground: String,
    space_char: String,
    last_end: EndType,
    cond_end: Option<(String, String, EndType)>,
    psvars: Vec<String>,
}

impl Prompt {
    fn default() -> Self {
        Self {
            background: String::new(),
            foreground: String::new(),
            space_char: " ".to_string(),
            last_end: EndType::None,
            cond_end: None,
            psvars: vec![],
        }
    }
    fn get_seperator(&self) -> String {
        self.space_char.clone()
    }
    fn get_foreground_string(&mut self, color: String) -> String {
        if self.foreground == color || color == "" {
            String::new()
        } else {
            self.foreground = color;
            format!("%F{{{}}}", self.foreground)
        }
    }
    fn get_background_string(&mut self, color: String) -> String {
        if self.background == color || color == "" {
            String::new()
        } else {
            self.background = color;
            format!("%K{{{}}}", self.background)
        }
    }
    fn get_last_end(&mut self, next_end: EndType, background: String) -> String {
        let ret = format!(
            "{new_back}{sep}{new_back2}",
            new_back = if self.last_end == EndType::LeftTriangle {
                self.get_foreground_string(background.clone())
            } else {
                self.get_background_string(background.clone())
            },
            new_back2 = if self.last_end == EndType::LeftTriangle {
                self.get_background_string(background)
            } else {
                String::new()
            },
            sep = self.last_end,
        );
        self.last_end = next_end;
        ret
    }
    fn add_psvar(&mut self, psvar: String) -> usize {
        self.psvars.push(psvar);
        self.psvars.len()
    }
    fn write_precmd(&self) {
        println!("precmd() {{");
        println!(
            "export psvar=({});",
            self.psvars
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    println!("local a{}={};", i, s);
                    format!("$a{} ", i)
                })
                .collect::<String>()
                .trim()
        );
        println!("}}");
    }
}

struct Params {
    parts: Vec<Part>,
    seperator: String,
}

impl Params {
    fn from_args(args: impl Iterator<Item = String>) -> Self {
        let mut cur_param = "";
        args.fold(
            Self {
                parts: vec![],
                seperator: " ".to_string(),
            },
            |mut s, part| {
                if cur_param != "" {
                    match cur_param {
                        "seperator" => s.seperator = part,
                        _ => (),
                    }
                    cur_param = "";
                } else if part.starts_with("-") {
                    match &part[..] {
                        "--seperator" => cur_param = "seperator",
                        _ => (),
                    }
                } else {
                    s.parts.append(&mut Part::from_string(part));
                }
                s
            },
        )
    }
    fn write_prompt(self) {
        // Write prompt info using println
        let mut prompt = Prompt::default();
        println!("PROMPT='';");
        for part in self.parts {
            println!("PROMPT+=$'{}';", part.to_prompt_format(&mut prompt));
        }
        prompt.write_precmd();
    }
}

fn main() {
    let mut args = std::env::args();
    println!(
        "# Envoking name: {}",
        args.next().expect("Arguments is empty");
    );
    Params::from_args(args).write_prompt();
}
