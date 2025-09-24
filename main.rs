use std::env;
use std::fs;

 fn main() {

    let raw_key = get_key(); //retuns unformatted keyfile addressed on the filepath and assigns it to raw_key

    let key_matrix: Vec<Vec<i64>> = clean_key(&raw_key); //formats the raw_key variable into an appropriate array matrix and assigns it to key_matrix

    let raw_text: String = get_plaintext(); //returns unformatted keyfile addressed on the filepat and assigns it to raw_key

    let text_array: Vec<char> = clean_plaintext(&raw_text); //returns an array of characters and assigns it to text_array

    let plaintext_number_array: Vec<u8> = letter_to_index(text_array.clone()); //returns an array of of numbers for each letter representation

    let matrix_size: usize = get_key_matrix_size(&key_matrix);

    let clean_key_matrix = get_clean_matrix(&key_matrix);

    let clean_plaintext: Vec<Vec<i64>> = get_clean_plaintext(&matrix_size, &plaintext_number_array);
    
    let cipher_int: Vec<i64> = compute_hill_cipher(&matrix_size, &clean_key_matrix, &clean_plaintext);
    
    let cipher_char = num_to_letter(&cipher_int);

    let depth: usize = get_depth();

    let transposed = compute_rail_transposition(&cipher_char, depth);

    print_key(&clean_key_matrix);

    println!("Plaintext:");
    print_text_wrapped(&text_array, 80);
    println!();

    if depth == 1 { // this will control whether to just print out the cipher char by itself or with a transposition
        println!("Ciphertext:");
        print_text_wrapped(&cipher_char, 80);
        println!();
    } else {
        println!("Ciphertext:");
        print_text_wrapped(&transposed, 80);
        println!();
    }

    println!("Depth: {:?}", depth);

}



fn get_depth() -> usize { //this will grab the fouth position of the input from the command line {depth} and assign it to a variable in the main function
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 4 {
        panic!("Missing depth argument. Usage: <program> encrypt key.txt plaintext.txt <depth>");
    }

    args[4]
        .trim()
        .parse()
        .expect("Depth must be a valid positive integer")
}

fn get_key() -> String{ //this will grab the second position of the input from the comand line {key filepath} read it and assign the contents to a variable in the main function
    
    let args:Vec<String> = env::args().collect();
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the key file");

    contents  
}

fn clean_key(raw_key: &String) -> Vec<Vec<i64>>{ //this will remove unwanted characters such as \n and create a matrix of the contents


    let key_matrix: Vec<Vec<i64>> = raw_key
        .lines()
        .map(|line|{
            line.split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
        })
        .collect();

    key_matrix
}

fn get_plaintext() -> String{ //this will grab the third position of the input from the command line {plaintext filepath} read it and assign the contents to a variable in the main function

    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[3];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the plaintext file");
    
    contents
}

fn clean_plaintext(raw_text:&String) -> Vec<char>{// this will remove any symbols or numbers from the plaintext array

    let text_array: Vec<char> = raw_text
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect();

    text_array

}

fn letter_to_index(text_array: Vec<char>) -> Vec<u8> { //this will convert plaintext leter to their number counterparts ex. A->0, B->1 etc

    text_array
        .into_iter()
        .filter(|c| c.is_ascii_uppercase())
        .map(|c| c as u8 - b'A')
        .collect()

}

fn get_key_matrix_size(key_matrix: &Vec<Vec<i64>>) -> usize { //this will return the key matrix size located as the first indexed position in the key file

    let size:usize = key_matrix[0][0] as usize; // Extract the size from the first row
    size

}

fn get_clean_matrix(key_matrix: &Vec<Vec<i64>>) -> Vec<Vec<i64>> { //this will return just the key matrix without the first index position which is a representation of the matrix size
    
    let size:usize = key_matrix[0][0] as usize; // Extract the size from the first row
    let _matrix = key_matrix[1..size + 1].to_vec(); // Slice rows 1 through size (inclusive)
    _matrix

}

fn get_clean_plaintext(size: &usize, plaintext: &Vec<u8>) -> Vec<Vec<i64>>{ //this will return a plaintext matrix based on the key matrix size

    let mut plaintext_matrix: Vec<Vec<i64>> = plaintext
    .chunks(*size)
    .map(|chunk| chunk.iter().map(|&x| x as i64).collect())
    .collect();

    if let Some(last_row) = plaintext_matrix.last_mut() { //fixes any lines that doens't equal the lenght of size
        while last_row.len() < *size {
            last_row.push(23);
        }   
    }
    plaintext_matrix

}

fn compute_hill_cipher(key_size: &usize, key_matrix: &Vec<Vec<i64>>, plaintext_matrix: &Vec<Vec<i64>>) ->Vec<i64> { //this line will take in the key matrix size, key matrix, and plaintext matrix and output the ciphertext

    let mut cipher_text: Vec<i64> = Vec::new();

    for block in plaintext_matrix.iter() {
        for row in key_matrix.iter() {
            let mut sum = 0;
            for j in 0..*key_size {
                sum += row[j] * block[j];
            }
            cipher_text.push(sum % 26);
        }
    }

    cipher_text

}

fn compute_rail_transposition(cipher_char: &Vec<char>, rails: usize) -> Vec<char> { //this line will compute the rail transposition based on the depth and the ciphertext already calculated
    if rails == 1 {
        return cipher_char.clone();
    }

    let mut fence: Vec<Vec<char>> = vec![Vec::new(); rails];
    let mut rail = 0;
    let mut direction = 1;

    for &ch in cipher_char {
        fence[rail].push(ch);

        if rail == 0 {
            direction = 1;
        } else if rail == rails - 1 {
            direction = -1;
        }

        rail = (rail as isize + direction) as usize;
    }

    fence.concat()
}

fn num_to_letter(int_cipher: &Vec<i64>) -> Vec<char> { //this line will convert the ciphertext from integers to their letter representation

    int_cipher
        .iter()
        .map(|&n| (n as u8 + b'A')as char)
        .collect()
}

fn print_key(key_matrix: &Vec<Vec<i64>>) { //this function is just used to print the key
    
    println!("Key matrix:");

    for row in key_matrix {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }

    println!();
}

fn print_text_wrapped(text: &[char], width: usize) { //this function is used to print a formatted version of the plaintext and ciphertext that wrap after 80 characters 
    for chunk in text.chunks(width) {
        let line: String = chunk.iter().collect();
        println!("{}", line);
    }
}
