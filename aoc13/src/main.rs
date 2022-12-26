use core::fmt;
use std::io;
use ouroboros::self_referencing;

#[derive (PartialEq, Clone, Copy, Debug)]
enum Token {
    EOF,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Int(isize),
}

#[self_referencing]
struct Lexer {
    s: String,
    peeked: Token,
    #[borrows(s)]
    r: &'this str,
}

impl Lexer {
    fn next(&mut self) -> Token {
        if *self.borrow_peeked() != Token::EOF {
            let p = *self.borrow_peeked();
            self.with_peeked_mut(|peeked| {*peeked = Token::EOF});
            return p;
        }
        if self.borrow_r().len() == 0 {
            return Token::EOF;
        }

        let r = self.with_r_mut(|user| -> Token {
            if let Some(r) = user.strip_prefix("[") {
                *user = r;
                return Token::OpeningBrace;
            }
            if let Some(r) = user.strip_prefix("]") {
                *user = r;
                return Token::ClosingBrace;
            }
            if let Some(r) = user.strip_prefix(",") {
                *user = r;
                return Token::Comma;
            }

            let i = user.find(|c| c == ',' || c == '[' || c == ']');
            match i {
                None => {
                    let ret = Token::Int(user.parse().expect("expecting int"));
                    *user = &user[user.len()..];
                    ret
                },
                Some(u) => {
                    let ret = Token::Int(user[..u].parse().expect("expecting int"));
                    *user = &user[u..];
                    ret
                },
            }
        });
        return r;
    }

    fn peek(&mut self) -> Token {
        let p = self.next();
        self.with_peeked_mut(|user| {*user = p});
        return p;
    }
}

enum Elem {
    Int(isize),
    List(Vec<Elem>),
}

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Elem::Int(i) => {write!(f, "{}", i)},
            Elem::List(elems) => {
                write!(f, "[")?;
                for (i, elem) in elems.iter().enumerate() {
                    elem.fmt(f)?;
                    if i != elems.len()-1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug)]
enum Ord {
    Determined(bool),
    Indetermined,
}

impl fmt::Display for Ord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ord::Determined(b) => write!(f, "{}", b),
            Ord::Indetermined => write!(f, "indetermined"),
        }
    }
}

fn print_indent(indent: isize) {
    for _ in 0..indent {
        print!("  ");
    }
}

impl Elem {
    fn new (l: &mut Lexer) -> Elem {
        let t = l.next();
        match t {
            Token::OpeningBrace => {
                let mut contents = Vec::new();
                // special case: empty list
                if l.peek() == Token::ClosingBrace {
                    l.next();
                    return Elem::List(contents);
                }
                loop {
                    let elem = Elem::new(l);
                    contents.push(elem);
                    let t2 = l.next();
                    match t2 {
                        Token::Comma => {},
                        Token::ClosingBrace => {break;}
                        _ => {panic!("unexpected token");}
                    }
                }
                return Elem::List(contents);
            },
            Token::Int(i) => {
                return Elem::Int(i);
            },
            _ => {
                panic!("unexpected token");
            }
        }
    }

    fn cmp(left: &Elem, right: &Elem, indent: isize) -> Ord {
        print_indent(indent);
        print!("{left} vs {right}: ");
        match left {
            Elem::Int(l) => match right {
                Elem::Int(r) => {
                    let mut res = Ord::Indetermined;
                    if l < r {
                        res =  Ord::Determined(true);
                    }
                    if l > r {
                        res =  Ord::Determined(false);
                    }
                    println!("{res}");
                    return res;
                }
                Elem::List(_) => {
                    let mut contents = Vec::new();
                    contents.push(Elem::Int(*l));
                    let tmp_l = Elem::List(contents);
                    println!("");
                    let res = Elem::cmp(&tmp_l, right, indent+1);
                    print_indent(indent);
                    println!("{res}");
                    return res;
                }
            },
            Elem::List(l) => match right {
                Elem::Int(r) => {
                    let mut contents = Vec::new();
                    contents.push(Elem::Int(*r));
                    let tmp_r = Elem::List(contents);
                    println!("");
                    let res = Elem::cmp(left, &tmp_r, indent+1);
                    print_indent(indent);
                    println!("{res}");
                    return res;
                },
                Elem::List(r) => {
                    for i in 0..l.len() {
                        if i >= r.len() {
                            let res = Ord::Determined(false);
                            println!("{res}");
                            return res;
                        }
                        println!("");
                        let c = Elem::cmp(&l[i], &r[i], indent+1);
                        print_indent(indent);
                        if let Ord::Determined(_) = c {
                            println!("{c}");
                            return c;
                        }
                    }
                    let mut res = Ord::Determined(true);
                    if l.len() == r.len() {
                        res = Ord::Indetermined;
                    }
                    println!("{res}");
                    return res;
                }
            }
        }
    }
}

fn main() {
    let mut index = 1;
    let mut sum = 0;
    loop {
        let mut line = String::new();

        let mut bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        if line.trim() == "" {
            continue;
        }

        let mut lexer1 = LexerBuilder{
            s: line,
            peeked: Token::EOF,
            r_builder: |s: &String| s,
        }.build();
        let l1 = Elem::new(&mut lexer1);

        let mut line2 = String::new();
        bytes = io::stdin()
            .read_line(&mut line2)
            .expect("Failed to read line");

        if bytes == 0 {
            panic!("expected second line to form pair");
        }
        let mut lexer2 = LexerBuilder{
            s: line2,
            peeked: Token::EOF,
            r_builder: |s: &String| s,
        }.build();
        let l2 = Elem::new(&mut lexer2);

        let res = Elem::cmp(&l1, &l2, 0);
        //println!("{} {:?}", index, res);
        match res {
            Ord::Indetermined => panic!(""),
            Ord::Determined(true) => {
                sum += index;
            },
            Ord::Determined(false) => {},
        }

        index += 1;
    }
    println!("{sum}");
}
