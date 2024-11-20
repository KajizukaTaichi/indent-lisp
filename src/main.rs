fn main() {
    println!("Hello, world!");
    let code = r#"
define (fact n)
    if (= n 0) 1
    (* n (fact (- n 1))
"#
    .trim()
    .to_string();
    println!("{}", parse(code).trans_compile());
}

#[derive(Debug)]
enum Type {
    Atom(String),
    Expr(Vec<Type>),
}

impl Type {
    fn trans_compile(&self) -> String {
        match &self {
            Type::Atom(a) => a.to_string(),
            Type::Expr(e) => format!(
                "({})",
                e.iter()
                    .map(|i| i.trans_compile())
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
        }
    }
}

fn parse(source: String) -> Type {
    let mut expr = vec![];
    let mut temp = String::new();
    let mut nest = 0;

    dbg!(tokenize(source.clone()));

    for token in tokenize(source) {
        if token == "INDENT" {
            if nest != 0 {
                temp += &format!("{token}\n");
            }
            nest += 1;
        } else if token == "DEDENT" {
            nest -= 1;
            if nest == 0 {
                expr.push(parse(temp.clone()));
                temp.clear();
            } else {
                temp += &format!("{token}\n");
            }
        } else {
            if nest == 0 {
                expr.push(Type::Atom(token));
            } else {
                temp += &format!("{token}\n");
            }
        }
    }
    Type::Expr(expr)
}

fn tokenize(source: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut indentation_stack = Vec::new();
    let mut previous_indent_level = 0;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') {
            tokens.push("DEDENT".to_string());
            tokens.push("INDENT".to_string());
            continue;
        }

        let current_indent_level = line.chars().take_while(|c| c.is_whitespace()).count();

        while previous_indent_level > current_indent_level {
            tokens.push("DEDENT".to_string());
            previous_indent_level = indentation_stack.pop().unwrap_or(0);
        }

        if current_indent_level > previous_indent_level {
            tokens.push("INDENT".to_string());
            indentation_stack.push(previous_indent_level);
            previous_indent_level = current_indent_level;
        }

        tokens.push(trimmed.to_string());
    }

    // 残ったインデントを全て閉じる
    while !indentation_stack.is_empty() {
        tokens.push("DEDENT".to_string());
        indentation_stack.pop();
    }

    tokens
}
