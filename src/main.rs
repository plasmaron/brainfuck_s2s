use std::io;

pub struct Data {
    memory: Vec<u8>,
    pointer: usize
}

macro_rules! brainfuck_parse {
    () => {};
    // >  Increment the data pointer by one. (++ptr;)
    (>; $data:expr) => {
        $data.pointer += 1;
    };
    // <  Decrement the data pointer by one. (--ptr;)
    (<; $data:expr) => {
        $data.pointer -= 1;
    };
    // +  Increment the byte at the data pointer by one. (++*ptr;)
    (+; $data:expr) => {
        $data.memory[$data.pointer] += 1;
    };
    // -  Decrement the byte at the data pointer by one. (--*ptr;)
    (-; $data:expr) => {
        $data.memory[$data.pointer] -= 1;
    };
    // .  Output the byte at the data pointer. (putchar(*ptr);)
    (.; $data: expr) => {
        print!("{}", $data.memory[$data.pointer] as char);
    };
    // ,  Accept one byte of input, storing its value in the byte at the data pointer. (ptr[0] = getchar();)
    (,; $data: expr) => {
        let mut buffer: [u8, 1] = [0];
        io:stdin().read_exact(&mut buffer)?;
        $data.memory[$data.pointer] = buffer[0];
    };
    // [] Loop everything in brackets until the byte at the data pointer is zero. (while (*ptr) {})
    ([$($x:tt)*]; $data:expr) => {
        while $data.memory[$data.pointer] != 0 {
            $(
                brainfuck_parse!($x; $data);
            )*
        }
    };

    // We need to split certain punctuation tokens that Rust recognizes as unique.
    // << (Shl)
    (<<; $data:expr) => {
        brainfuck_parse!(<; $data);
        brainfuck_parse!(<; $data);
    };

    // >> (Shr)
    (>>; $data:expr) => {
        brainfuck_parse!(>; $data);
        brainfuck_parse!(>; $data);
    };

    // .. (DotDot)
    (..; $data:expr) => {
        brainfuck_parse!(.; $data);
        brainfuck_parse!(.; $data);
    };

    // ... (DotDotDot)
    (...; $data:expr) => {
        brainfuck_parse!(.; $data);
        brainfuck_parse!(.; $data);
        brainfuck_parse!(.; $data);
    };

    // <- (LArrow)
    (<-; $data:expr) => {
        brainfuck_parse!(<; $data);
        brainfuck_parse!(-; $data);
    };

    // -> (RArrow)
    (->; $data:expr) => {
        brainfuck_parse!(-; $data);
        brainfuck_parse!(>; $data);
    };
}

macro_rules! brainfuck {
    // Reads in a Brainfuck program as individual tokens.
    // Due to the nature of Rust's :tt fragment specifier, everything within [] delimiters will be
    // read as a single command.
    ($($x:tt)*) => {{
        // Create an instance of the program's data storage struct.
        let mut program_data = Data {
            memory: vec![0; 30000],
            pointer: 0,
        };
        // For each token in the program, parse as Rust code, referencing the program's data and
        // data pointer.
        $(
            brainfuck_parse!($x; program_data);
        )*
    }};
}

fn main() {
    brainfuck!(+++++[-]);

    //brainfuck!(+[>[<-[]>+[>+++>[+++++++++++>][>]-[<]>-]]++++++++++<]>>>>>>----.<<+++.<-..+++.<-.>>>.<<.+++.------.>-.<<+.<.);
    brainfuck!(++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.);
}
