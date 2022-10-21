#[derive(Debug)]
pub enum Command {
    Inc,
    Dec,
    Left,
    Right,
    OpeningBracket(u32),
    ClosingBracket(u32),
    Write,
    Read,
    Comment
}

pub fn lex(x: &str) -> Vec<Command> {
    let mut ret: Vec<Command> = Vec::new();
    let mut o: u32 = 0;
    let mut e: u32 = 0;

    for i in x.chars() {
        ret.push(match i {
            '+' => Command::Inc,
            '-' => Command::Dec,
            '<' => Command::Left,
            '>' => Command::Right,
            '[' => Command::OpeningBracket({let temp = o; o += 1; temp}),
            ']' => Command::ClosingBracket({let temp = e; e += 1; temp}),
            '.' => Command::Write,
            ',' => Command::Read,
            _   => Command::Comment
        });
    }
    
    if o > e {
        println!("SyntaxError: Opening bracket not closed off");
        std::process::exit(1);
    } else if o < e {
        println!("SyntaxError: Unexpected closing bracket");
        std::process::exit(1);    
    }
    ret
}