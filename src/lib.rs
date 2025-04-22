use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    Free(String),
    Pair { key: String, val: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Frag {
    Raw(String),
    Tag {
        name: String,
        val: Option<String>,
        params: Vec<Param>,
        subfrags: Vec<Frag>,
    },
}

pub fn parse_frags(input: &str) -> Vec<Frag> {
    let mut chars: VecDeque<char> = input.chars().collect();
    parse_until(&mut chars, None)
}

fn parse_until(chars: &mut VecDeque<char>, end_tag: Option<&str>) -> Vec<Frag> {
    let mut frags = Vec::new();
    let mut buf = String::new();

    while let Some(&c) = chars.front() {
        if c == '[' {
            if !buf.is_empty() {
                frags.push(Frag::Raw(buf.clone()));
                buf.clear();
            }

            chars.pop_front();

            if chars.front() == Some(&'/') {
                chars.pop_front();
                let name = take_name(chars);
                chars.pop_front();
                if let Some(et) = end_tag {
                    if name.eq_ignore_ascii_case(et) {
                        return frags;
                    }
                }
                frags.push(Frag::Raw(format!("[/{name}]")));
                continue;
            }

            let name = take_name(chars);
            let mut val = None;
            let mut params = Vec::new();
            skip_spaces(chars);
            if chars.front() == Some(&'=') {
                chars.pop_front();
                let v = take_while(chars, |ch| ch != ']' && !ch.is_whitespace());
                val = Some(v);
                skip_spaces(chars);
            }

            while chars.front().map_or(false, |&ch| ch != ']') {
                let token = take_while(chars, |ch| ch != ']' && !ch.is_whitespace());
                if token.contains('=') {
                    let mut parts = token.splitn(2, '=');
                    let k = parts.next().unwrap().to_string();
                    let v = parts.next().unwrap().to_string();
                    params.push(Param::Pair { key: k, val: v });
                } else {
                    params.push(Param::Free(token));
                }
                skip_spaces(chars);
            }

            chars.pop_front();

            let sub = parse_until(chars, Some(&name));
            frags.push(Frag::Tag {
                name,
                val,
                params,
                subfrags: sub,
            });
        } else {
            buf.push(c);
            chars.pop_front();
        }
    }

    if !buf.is_empty() {
        frags.push(Frag::Raw(buf));
    }
    frags
}

fn take_name(chars: &mut VecDeque<char>) -> String {
    take_while(chars, |ch| ch.is_alphanumeric())
}

fn skip_spaces(chars: &mut VecDeque<char>) {
    while chars.front().map_or(false, |&ch| ch.is_whitespace()) {
        chars.pop_front();
    }
}

fn take_while<F>(chars: &mut VecDeque<char>, pred: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut s = String::new();
    while chars.front().map_or(false, |&ch| pred(ch)) {
        s.push(chars.pop_front().unwrap());
    }
    s
}
