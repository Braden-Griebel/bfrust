use std;
use std::io;
use std::io::Read;
use std::path::Path;
use std::io::Write;


fn read_program<P: AsRef<Path>>(path: P)->Vec<char>{
    // Read the file into a string
    let program: String = match std::fs::read_to_string(&path){
        Err(why) => panic!("Couldn't read file: {}", why),
        Ok(program_string) => program_string,
    };
    // Hashset of Acceptable characters
    let accepted_chars:std::collections::HashSet<char> = "<>[]+-.,".chars().collect();

    // Filter the string so it only contains accepted characters
    program.chars().filter(|c| accepted_chars.contains(c)).collect()
}

fn main() {
    // Read the path from the command line args
    let path = std::env::args().nth(1).expect("No Path Given!");

    // Read the program in from the path
    let program: Vec<char> = read_program(&path);


    // Initialize the data array and data pointer
    let mut data_array: [u8; 30_000] = [0; 30_000];
    let mut data_pointer:usize = 0;

    // Initialize the program pointer
    let mut prog_pointer: usize = 0;

    // Implement the actual program
    while prog_pointer < program.len(){
        // Perform command
        match program[prog_pointer]{
            // Increment data array at the pointer
            '+'=>{
                data_array[data_pointer] = data_array[data_pointer].wrapping_add(1);
            },
            // Decrement data array at the pointer
            '-'=>{
                data_array[data_pointer]=data_array[data_pointer].wrapping_sub(1);
            },
            // Increment the data pointer
            '>'=>{
                data_pointer+=1;
            },
            // Decrement the data pointer
            '<'=>{
                data_pointer-=1;
            },
            /*
             If the value of the array at the data pointer is 0, jump to matching close bracket,
             otherwise just increment the program pointer by one
             */
            '['=>{
                let mut paren_count:u32 = 0;
                if data_array[data_pointer]==0{
                    prog_pointer+=1;
                    while paren_count > 0 || program[prog_pointer]!=']'{
                        if program[prog_pointer]=='['{
                            paren_count+=1;
                        } else if program[prog_pointer]==']' && paren_count!=0{
                            paren_count-=1;
                        }
                        prog_pointer+=1;
                    }
                }
            },
            /*
            If the value of the array at the data pointer is nonzero, then jump back to the
            command after the matching [ command
             */
            ']'=>{
                let mut paren_count:u32 = 0;
                if data_array[data_pointer]!=0{
                    prog_pointer-=1;
                    while paren_count > 0 || program[prog_pointer]!='['{
                        if program[prog_pointer]==']'{
                            paren_count+=1;
                        } else if program[prog_pointer]=='[' && paren_count!=0{
                            paren_count-=1;
                        }
                        prog_pointer-=1;
                    }
                }
            },
            '.'=>{
                print!("{}", data_array[data_pointer] as char);
                io::stdout().flush().unwrap();
            },
            ','=>{
                let mut buffer = [0;1]; // Buffer holding one byte

                // Read one byte from stdin
                match io::stdin().read(&mut buffer){
                    Ok(1)=>{
                        let byte = buffer[0];
                        if byte.is_ascii(){
                            data_array[data_pointer] = byte;
                        }
                    },
                    Ok(_)=>{
                        panic!("Byte Not Read!");
                    },
                    Err(err) => {
                        panic!("Error reading byte: {}", err)
                    }

                }
            },
            c=>{panic!("Found impossible character after program parsing: {}", c)},
        }
        // Increment program pointer
        prog_pointer+=1;
    }
    println!("\nThe state of the first 10 cells in the array is:");
    for val in &data_array[..=10]{
        print!("{}, ", val)
    }
}
