//! You know what this web development project needs?
//! It needs a minimal turing-complete language and interpreter embedded within it to obfuscate
//! what would otherwise be trivially readable code.
//! (Brainfuck)[https://en.wikipedia.org/wiki/Brainfuck] is just such a language.
extern crate nom;

use nom::*;

use nom::types::CompleteByteSlice as Input;
use std::{ops::Deref, str::Chars};

pub struct BfProgram(Vec<Token>);

impl Deref for BfProgram {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Runs a brainfuck program.
/// This does not allow input and output via , and .
/// Instead, it allows interop with Rust via setting the tape to a pre-defined state beforehand and allowing
/// examination of it afterwords.
pub fn run_brainfuck(program: &[Token], tape: &mut [u8]) {
    consume_tokens(program, tape, &mut 0, &mut "".chars());
}

/// Parses a brainfuck program.
pub fn parse_brainfuck(input: &str) -> Option<BfProgram> {
    let input = Input::from(input.as_bytes());
    let (_, b) = brainfuck_parser(input).ok()?;
    Some(BfProgram(b))
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    ShiftRight,
    ShiftLeft,
    Output,
    InputT,
    Loop { expr: Vec<Token> },
    Comment,
}

fn consume_tokens(
    tokens: &[Token],
    tape: &mut [u8],
    tape_pointer: &mut usize,
    input: &mut Chars,
) -> String {
    let mut output_string: String = String::new();

    for token in tokens {
        match *token {
            Token::Plus => {
                tape[*tape_pointer] = tape[*tape_pointer].wrapping_add(1);
            }
            Token::Minus => {
                tape[*tape_pointer] = tape[*tape_pointer].wrapping_sub(1);
            }
            Token::ShiftRight => {
                *tape_pointer += 1;
            }
            Token::ShiftLeft => {
                *tape_pointer -= 1;
            }
            Token::Output => {
                print!("{}", tape[*tape_pointer] as char);
                output_string.push(tape[*tape_pointer] as char);
            }
            Token::InputT => {
                match input.next() {
                    Some(c) => {
                        if c.is_ascii() {
                            let value: u8 = c as u8;
                            tape[*tape_pointer] = value; // set the value at the ptr to be the value of the input character.
                        } else {
                            panic!("character {} is not ascii", c);
                        }
                    }
                    None => {
                        panic!("Ran out of input"); // todo, look for some spec on BF to find out what to do here. Should I loop back around the input?
                    }
                };
            }
            Token::Loop { ref expr } => {
                while tape[*tape_pointer] > 0 {
                    consume_tokens(&expr, tape, tape_pointer, input);
                }
            }
            _ => {}
        }
    }

    output_string
}

named!(plus_parser<Input, Token>,
    do_parse!(
        tag!("+") >>
        (Token::Plus)
    )
);

named!(minus_parser<Input, Token>,
    do_parse!(
        tag!("-") >>
        (Token::Minus)
    )
);

named!(shiftr_parser<Input, Token>,
    do_parse!(
        tag!(">") >>
        (Token::ShiftRight)
    )
);

named!(shiftl_parser<Input, Token>,
    do_parse!(
        tag!("<") >>
        (Token::ShiftLeft)
    )
);

named!(output_parser<Input, Token>,
    do_parse!(
        tag!(".") >>
        (Token::Output)
    )
);

named!(input_parser<Input, Token>,
    do_parse!(
        tag!(",") >>
        (Token::InputT)
    )
);

named!(comment_parser<Input, Token>,
    do_parse!(
        tag!("//") >>
        many_till!(anychar, any_end)  >>
        (Token::Comment)
    )
);

named!(any_end<Input, Input>,
    complete!(alt!(line_ending | eof!()))
);

named!(loop_parser<Input, Token>,
    do_parse!(
        expression: ws!(delimited!(tag!("["), many0!(syntax), tag!("]")))>>
        (Token::Loop {expr: expression})
    )
);

named!(syntax<Input, Token>,
    alt!( plus_parser | minus_parser | shiftr_parser | shiftl_parser | output_parser | input_parser | loop_parser | comment_parser )
);

named!(brainfuck_parser<Input, Vec<Token> >,
    do_parse!(
        tokens: many0!(ws!(syntax)) >>
        (tokens)
    )
);

#[test]
fn plus_parser_test() {
    let plus = Input(&b"+"[..]);
    let res = plus_parser(plus);
    let remainder = Input(&b""[..]);
    assert_eq!(res, Ok((remainder, Token::Plus)));
}

#[test]
fn syntax_test() {
    let syn = Input(&b"-"[..]);
    let remainder = Input(&b""[..]);
    let res = syntax(syn);
    assert_eq!(res, Ok((remainder, Token::Minus)));
}

#[test]
fn loop_test() {
    let looop = Input(&b"[++-]"[..]);
    let remainder = Input(&b""[..]);
    let res = loop_parser(looop);
    use Token::*;
    assert_eq!(
        res,
        Ok((
            remainder,
            Token::Loop {
                expr: vec!(Plus, Plus, Minus)
            }
        ))
    );
}

#[test]
fn nested_loop_test() {
    let looop = Input(&b"[+[++]-]"[..]);
    let remainder = Input(&b""[..]);
    let res = loop_parser(looop);

    use Token::*;
    assert_eq!(
        res,
        Ok((
            remainder,
            Token::Loop {
                expr: vec!(
                    Plus,
                    Loop {
                        expr: vec!(Plus, Plus)
                    },
                    Minus
                )
            }
        ))
    );
}

#[test]
fn ignore_whitespace_test() {
    let bf = Input(
        &b"+-+>  <  -

    +"[..],
    );
    let remainder = Input(&b""[..]);
    let res = brainfuck_parser(bf);

    use Token::*;
    assert_eq!(
        res,
        Ok((
            remainder,
            vec!(Plus, Minus, Plus, ShiftRight, ShiftLeft, Minus, Plus)
        ))
    );
}

// tests for end of line for the comment
#[test]
fn ignore_comment_eol_test() {
    let bf = Input(
        &b"+ //+
    +"[..],
    );
    let remainder = Input(&b""[..]);
    let res = brainfuck_parser(bf);

    use Token::*;
    assert_eq!(res, Ok((remainder, vec!(Plus, Comment, Plus))));
}

//tests for end of file
#[test]
fn ignore_comment_eof_test() {
    let bf = Input(&b"+ //"[..]);
    let remainder = Input(&b""[..]);
    let res = brainfuck_parser(bf);

    assert_eq!(res, Ok((remainder, vec!(Token::Plus, Token::Comment))));
}

#[test]
fn hello_world_integration_test() {
    let bf = "++++++++               //Set Cell #0 to 8
[
    >++++               //Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   //as the cell will be cleared by the loop
        >++             //Add 2 to Cell #2
        >+++            //Add 3 to Cell #3
        >+++            //Add 3 to Cell #4
        >+              //Add 1 to Cell #5
        <<<<-           //Decrement the loop counter in Cell #1
    ]                   //Loop till Cell #1 is zero; number of iterations is 4
    >+                  //Add 1 to Cell #2
    >+                  //Add 1 to Cell #3
    >-                  //Subtract 1 from Cell #4
    >>+                 //Add 1 to Cell #6
    [<]                 //Move back to the first zero cell you find; this will
                        //be Cell #1 which was cleared by the previous loop
    <-                  //Decrement the loop Counter in Cell #0
]

>>.                     //Cell #2 has value 72 which is 'H'
>---.                   //Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           //Likewise for 'llo' from Cell #3
>>.                     //Cell #5 is 32 for the space
<-.                     //Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      //Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    //Cell #3 for 'rl' and 'd'
>>+.                    //Add 1 to Cell #5 gives us an exclamation point
>++.                    //And finally a newline from Cell #6
"
    .to_string();

    const TAPE_SIZE: usize = 32000;
    let mut tape = [0; TAPE_SIZE];
    let mut tape_pointer: usize = 0;

    let tokens: BfProgram = parse_brainfuck(&bf).unwrap();

    let output = consume_tokens(&tokens, &mut tape, &mut tape_pointer, &mut "".chars());
    assert_eq!(output, "Hello World!\n");
}

#[test]
fn multiplication_integration_test() {
    let bf = "+++++++ [>+++<-]>".to_string(); // 7 * 3

    const TAPE_SIZE: usize = 32000;
    let mut tape = [0; TAPE_SIZE];
    let mut tape_pointer: usize = 0;

    let tokens: BfProgram = parse_brainfuck(&bf).unwrap();

    let _ = consume_tokens(&tokens, &mut tape, &mut tape_pointer, &mut "".chars());
    assert_eq!(tape_pointer, 1);
    assert_eq!(tape[tape_pointer], 21);
}

#[test]
fn seek_right() {
    let mut tape = [0; 30];
    let mut tape_pointer: usize = 0;

    let bf = "++++>+<[>]+".to_string();
    let tokens: BfProgram = parse_brainfuck(&bf).unwrap();
    let _output = consume_tokens(&tokens, &mut tape, &mut tape_pointer, &mut "HI".chars());
    assert_eq!(tape_pointer, 2);
    assert_eq!(tape[0], 4);
    assert_eq!(tape[1], 1);
    assert_eq!(tape[2], 1);
}

#[test]
fn compare() {
    let mut tape = [0; 30];
    let mut tape_pointer: usize = 0;

    let bf = ">++++>+++<[- > -[>]<<]".to_string();
    let tokens: BfProgram = parse_brainfuck(&bf).unwrap();
    let _output = consume_tokens(&tokens, &mut tape, &mut tape_pointer, &mut "HI".chars());
    std::dbg!(&tape);
    assert_eq!(tape_pointer, 0);
    assert_eq!(tape[0], 0);
    assert_eq!(tape[1], 1);
    assert_eq!(tape[2], 0);
}

#[test]
fn read_input_test() {
    let bf = ",.>,.".to_string();

    const TAPE_SIZE: usize = 32000;
    let mut tape = [0; TAPE_SIZE];
    let mut tape_pointer: usize = 0;

    let tokens: BfProgram = parse_brainfuck(&bf).unwrap();

    let output = consume_tokens(&tokens, &mut tape, &mut tape_pointer, &mut "HI".chars());
    assert_eq!(tape_pointer, 1);
    assert_eq!(tape[0], 72); // H
    assert_eq!(tape[1], 73); // I
    assert_eq!(output, "HI".to_string());
}
