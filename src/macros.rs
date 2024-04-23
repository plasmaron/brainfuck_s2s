use std::io;
use std::io::{Read, Write};

// Brainfuck machine model: 1-dimensional memory of byte cells, movable data pointer
pub struct Data {
    pub memory: Vec<u8>,
    pub pointer: usize
}

// Parses individual Brainfuck commands.
macro_rules! brainfuck_parse {
    () => {};
    // >  Increment the data pointer by one. (++ptr;)
    (>; $data:expr) => {
        $data.pointer += 1;
        // Bump allocated memory :3
        if $data.pointer >= $data.memory.len() {
            $data.memory.resize($data.memory.len()*2, 0);
        }
    };
    // <  Decrement the data pointer by one. (--ptr;)
    (<; $data:expr) => {
        // Check if out of bounds
        if ($data.pointer == 0) {
            panic!("Data pointer out of bounds!");
        }
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
        <_ as ::std::io::Write>::flush(&mut ::std::io::stdout()).unwrap();
    };
    // ,  Accept one byte of input, storing its value in the byte at the data pointer. (ptr[0] = getchar();)
    (,; $data: expr) => {
        let mut buffer: [u8; 1] = [0];
        <_ as ::std::io::Read>::read_exact(&mut ::std::io::stdin(), &mut buffer)?;
        // dbg!(buffer[0]);
        $data.memory[$data.pointer] = buffer[0];
    };
    // [] Loop everything in brackets until the byte at the data pointer is zero. (while (*ptr) {})
    ([$($x:tt)*]; $data:expr) => {
        while $data.memory[$data.pointer] != 0 {
            $(
                $crate::macros::brainfuck_parse!($x; $data);
            )*
        }
    };

    // We need to split certain punctuation tokens that Rust recognizes as unique.
    // << (Shl)
    (<<; $data:expr) => {
        $crate::macros::brainfuck_parse!(<; $data);
        $crate::macros::brainfuck_parse!(<; $data);
    };

    // >> (Shr)
    (>>; $data:expr) => {
        $crate::macros::brainfuck_parse!(>; $data);
        $crate::macros::brainfuck_parse!(>; $data);
    };

    // .. (DotDot)
    (..; $data:expr) => {
        $crate::macros::brainfuck_parse!(.; $data);
        $crate::macros::brainfuck_parse!(.; $data);
    };

    // ... (DotDotDot)
    (...; $data:expr) => {
        $crate::macros::brainfuck_parse!(.; $data);
        $crate::macros::brainfuck_parse!(.; $data);
        $crate::macros::brainfuck_parse!(.; $data);
    };

    // <- (LArrow)
    (<-; $data:expr) => {
        $crate::macros::brainfuck_parse!(<; $data);
        $crate::macros::brainfuck_parse!(-; $data);
    };

    // -> (RArrow)
    (->; $data:expr) => {
        $crate::macros::brainfuck_parse!(-; $data);
        $crate::macros::brainfuck_parse!(>; $data);
    };
}

// Compiles a full Brainfuck program into Rust source code.
macro_rules! brainfuck {
    // Reads in program as individual tokens.
    // Due to the nature of Rust's :tt fragment specifier, everything within [] delimiters will be
    // read as a single command.
    ($($x:tt)*) => {{
            // Create an instance of the program's data storage struct.
            let mut program_data = $crate::macros::Data {
                memory: vec![0; 30000],
                pointer: 0,
            };
            // For each token in the program, parse as Rust code, referencing the program's data and
            // data pointer.
            $(
                $crate::macros::brainfuck_parse!($x; program_data);
            )*
    }};
}
pub(crate) use brainfuck;
pub(crate) use brainfuck_parse;


