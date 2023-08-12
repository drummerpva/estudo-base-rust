use std::{
    error::Error,
    fs::{self, File},
    io::{self},
};

const FILE_NAME: &str = "hello.txt";

/* fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open(FILE_NAME)?;
    Ok(())
} */
fn main() {
    // panic!("crash and burn");
    // let f = File::open(FILE_NAME);
    /* let _file = match f {
        Ok(file) => file,
        Err(error) => panic!("Arquivo não pode ser aberto: {:?}", error),
    }; */
    /* let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_NAME) {
                Ok(file_created) => file_created,
                Err(error_created) => panic!("Problem creating file{:?}", error_created),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    }; */
    /* let _f = File::open(FILE_NAME).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(FILE_NAME).unwrap_or_else(|error_created| {
                panic!("Problem creating the file: {:?}", error_created)
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    }); */

    // let _f = File::open("unexist_file.txt").unwrap();
    // let _f = File::open("unexist_file.txt").expect("Failed to open file!");
    let content_file = read_unername_from_file();
    match content_file {
        Ok(texto) => {
            let last_char =
                last_char_of_first_line(&texto).expect("Sem texto para pegar o ultimo caractere");
            println!(
                "Content of file: {}, e o ultimo caractere da primeira linha: '{}'",
                texto, &last_char
            )
        }
        Err(e) => println!("Erro ao pegar conteudo: {:?}", e),
    }
}

fn read_unername_from_file() -> Result<String, io::Error> {
    fs::read_to_string(FILE_NAME)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

/* fn read_unername_from_file() -> Result<String, io::Error> {
    let f = File::open(FILE_NAME);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
} */
/* fn read_unername_from_file() -> Result<String, io::Error> {
    let mut f = File::open(FILE_NAME)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
} */
/* fn read_unername_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(FILE_NAME)?.read_to_string(&mut s)?;
    Ok(s)
} */
