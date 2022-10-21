use crate::lexer::Command;

pub fn compile(x: Vec<Command>) -> String {
    let mut comp: String = String::from("format ELF64 executable\nentry start\nsegment readable executable\nstart:\nmov rsi, mem\n");
    // borrow checker trouble
    let mut t;
    for c in x {
        comp.push_str(match c {
            Command::Inc                    => "; add\ninc byte [rsi]\n",
            Command::Dec                    => "; sub\ndec byte [rsi]\n",
            Command::Left                   => "; left\ndec rsi\n",
            Command::Right                  => "; right\ninc rsi\n",
            Command::OpeningBracket(i) => {t = format!("; opb\nb{i}:\nmov al, byte [rsi]\ntest al, al\njz e{i}\n"); t.as_str()},
            Command::ClosingBracket(i) => {t = format!("; endb\nmov al, byte [rsi]\ntest al, al\njnz b{i}\ne{i}:\n"); t.as_str()},
            Command::Write                  => "; write\nmov rax, 1\nmov rdi, 1\nmov rdx, 1\nsyscall\n",
            Command::Read                   => "",
            _                               => ""
        });
    }
    comp.push_str("mov rax, 60\nmov rdi, 0\nsyscall\nsegment readable writable\nmem: times 30000 db 0");
    comp
}