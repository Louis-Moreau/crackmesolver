use clap::Parser;
use rand::prelude::*;

const LENGHT_KEY: usize = 19;

/// Simple program to generate keys
#[derive(Parser, Debug)]
#[command(author="Louis Moreau", version, about="A key generator for crackme05", long_about = None)]
struct Args {

    /// Display the number of possible keys
    #[arg(short, long)]
    display_count: bool,

    /// Number of key to generate
    #[arg(short, long, default_value_t = 1)]
    number: u32,
}


fn main() {
    let not_rock : Vec<u8>  = vec![0,1,2,3,4,5,8,9,10,13,14,15,16,17,18];
    let mut rng = rand::thread_rng();

    let args = Args::parse();

    let letters = get_rock_letters();
    
    for _i in 0..args.number {
        let mut combinations : u64 = 1;
        let seed : u64 = rng.next_u64();
        let mut key : [char; LENGHT_KEY] = ['h';LENGHT_KEY];
        combinations *= solve_rock(&not_rock,&letters,&mut key,seed);
        combinations *= solve_paper(&letters,&mut key,seed / combinations);
        combinations *= solve_scissors(&letters,&mut key,seed / combinations);
        solve_cracker(&letters,&mut key,seed / combinations);
        let key_string: String =  key.iter().collect();
        println!("{}",key_string);
    }

    if args.display_count {
        println!("Possible combinations : {}",combinations_rock(&not_rock,&letters) * combinations_paper(&letters) * combinations_paper(&letters) * combinations_scissors(&letters) * combinations_cracker(&letters));
    }
}


fn get_rock_letters() -> Vec<char> {
    let mut out : Vec<char>  = vec![];
    for c in 32..127_u8 {
        //println!("{}",c);
        if c < ('-' as u8) || (('-' as u8) < c && c < ('0' as u8)) {
            //println!("ROCK1");
        }
        else if c < (':' as u8) || ('@' as u8) < c{
            if (('Z' as u8) < c && c < ('a' as u8)) || ('z' as u8) < c {
                //println!("ROCK3");
            } else {
                out.push(c as char);
            }
        } else {
            //println!("ROCK2");
        }
    }
    return out;
}


fn combinations_rock(not_rock : &Vec<u8> ,letters : &Vec<char>) -> u64 {
    let mut combinations: u64 = 0;
    for i in 0..LENGHT_KEY {
        if !not_rock.contains(&(i as u8)) {
            combinations += letters.len() as u64;
        }
    }
    return combinations;
}

fn combinations_paper(letters : &Vec<char>) -> u64 {
    let mut char1 : char;
    let mut char2 : char;
    let mut combinations: u64 = 0;
    for i in 0..letters.len() {
        char1 = letters[i];
        for j in 0..letters.len() {
            char2 = letters[j];
            let test_var : u8 =  ((char1 as u8) ^ (char2 as u8)) + 0x30;

            if test_var < 0x3a && !(test_var < 0x30) && letters.binary_search(&(test_var as char)).is_ok(){
                combinations += 1;
            }
        }
    }
    return combinations;
}

fn combinations_scissors(letters : &Vec<char>) -> u64 {
    let mut chars : [char;4] = ['l';4];
    let mut combinations: u64 = 0;
    
    for i in 0..letters.len() {
        chars[0] = letters[i];
        for j in 0..letters.len() {
            chars[1] = letters[j];
            let test_var1 : u32 =  chars[0] as u32 + chars[1] as u32;
            if !(test_var1 < 0xab) {
                for k in 0..letters.len() {
                    chars[2] = letters[k];
                    for l in 0..letters.len() {
                        chars[3] = letters[l];
                        let test_var2 : u32 =  chars[2] as u32 + chars[3] as u32;
                        if !(test_var2 < 0xab) && (test_var1 != test_var2) {
                            combinations += 1;
                        }
                    }
                }
            }
        }
    }
    return combinations;
}

fn combinations_cracker(letters : &Vec<char>) -> u64 {
    let mut chars : [char;3] = ['l';3];
    let mut combinations: u64 = 0;
    for i in 0..letters.len() {
        chars[0] = letters[i];
        for j in 0..letters.len() {
            chars[1] = letters[j];
            for k in 0..letters.len() {
                chars[2] = letters[k];
                if chars[0] as u32 + chars[1] as u32 + chars[2] as u32 == 135{
                    combinations += 1;
                }
            }
        }
    }
    return combinations;
}

fn solve_rock(not_rock : &Vec<u8> , letters : &Vec<char>,key : &mut [char;19],seed : u64) -> u64 {
    let combinations = combinations_rock(not_rock,letters);
    let mut m_seed = seed;
    for i in 0..LENGHT_KEY {
        if !not_rock.contains(&(i as u8)) {
            key[i] = letters[(m_seed % letters.len() as u64) as usize];
            m_seed /= letters.len() as u64;
        }
    }
    return combinations;
}

fn solve_paper(letters : &Vec<char>,key : &mut [char;19],seed : u64) -> u64 {
    let combinations = combinations_paper(letters);
    let mut counter : u64 = 0;
    'outer: for i in 0..letters.len() {
        key[10] = letters[i];
        for j in 0..letters.len() {
            key[8] = letters[j];
            let test_var1 : u8 =  ((key[10] as u8) ^ (key[8] as u8)) + 0x30;
            if test_var1 < 0x3a && !(test_var1 < 0x30) && letters.binary_search(&(test_var1 as char)).is_ok(){
                if counter == seed % combinations {
                    key[3] = test_var1 as char;
                    key[15] = test_var1 as char;
                    break 'outer;
                }
                counter += 1;
            }
        }
    }

    let mut counter : u64 = 0;
    'outer: for i in 0..letters.len() {
        key[13] = letters[i];
        for j in 0..letters.len() {
            key[5] = letters[j];
            let test_var2 : u8 =  ((key[13] as u8) ^ (key[5] as u8)) + 0x30;
            if test_var2 < 0x3a && !(test_var2 < 0x30) && letters.binary_search(&(test_var2 as char)).is_ok(){
                if counter == (seed / combinations) % combinations  {
                    key[0] = test_var2 as char;
                    key[18] = test_var2 as char;
                    break 'outer;
                }
                counter += 1;
            }
        }
    }

    return combinations * combinations;
}

fn solve_scissors(letters : &Vec<char>,key : &mut [char;19],seed : u64) -> u64 {
    let combinations = combinations_scissors(letters);
    let mut counter : u64 = 0;
    'outer: for i in 0..letters.len() {
        key[2] = letters[i];
        for j in 0..letters.len() {
            key[1] = letters[j];
            let test_var1 : u32 =  key[2] as u32 + key[1] as u32;
            if !(test_var1 < 0xab) {
                for k in 0..letters.len() {
                    key[17] = letters[k];
                    for l in 0..letters.len() {
                        key[16] = letters[l];
                        let test_var2 : u32 =  key[17] as u32 + key[16] as u32;
                        if !(test_var2 < 0xab) && (test_var1 != test_var2) {
                            if counter == seed % combinations {
                                break 'outer;
                            }
                            counter += 1;
                        }
                    }
                }
            }
        }
    }
    return combinations;
}

fn solve_cracker(letters : &Vec<char>,key : &mut [char;19],seed : u64) -> u64 {
    let combinations = combinations_cracker(letters);
    let mut counter : u64 = 0;
    'outer: for i in 0..letters.len() {
        key[14] = letters[i];
        for j in 0..letters.len() {
            key[4] = letters[j];
            for k in 0..letters.len() {
                key[9] = letters[k];
                if key[14] as i32 + key[4] as i32 + key[9] as i32 == 0x87{
                    if counter == seed % combinations {
                        break 'outer;
                    }
                    counter += 1;
                }
            }
        }
    }
    return combinations;
}