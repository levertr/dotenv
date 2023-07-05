use std::{env, fs::{read_to_string}};

fn main() {
    // Ici on récupère le fichier .env à la racine du projet
    let env_contents = match env::current_dir() {
        Err(_) => panic!("N'a pas pu récupérer current_dir()"),
        Ok(path)  => match read_to_string(path.join(".env")) {
            Err(_) => panic!("Le fichier .env n'existe pas"),
            Ok(contents) => contents
        }
    };

    println!("{:?}", env_contents);
}
