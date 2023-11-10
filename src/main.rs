use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
fn generate_password(length: usize) -> String {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(|c| c as char)
        .collect();
    password
}


fn get_file_path() -> String {
    let config_folder = "configuracao";
    fs::create_dir_all(config_folder).expect("Falha ao criar o diretório de configuração");

    let config_path = format!("{}/config.txt", config_folder);

    // Tenta ler o caminho do arquivo de configuração
    match fs::read_to_string(&config_path) {
        Ok(contents) => contents.trim().to_string(),
        Err(_) => {
            // Se não houver um arquivo de configuração, solicita ao usuário que insira o caminho
            println!("Digite o caminho completo do arquivo (ex: /caminho/do/arquivo/nome_arquivo.txt):");
            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path).expect("Falha ao ler a entrada");

            // Salva o caminho no arquivo de configuração
            fs::write(&config_path, &file_path).expect("Erro ao escrever no arquivo de configuração");

            file_path.trim().to_string()
        }
    }
}
fn main() {
    //mensagem de boas vindas 
    println!("Bem vindo ao gerador e administrador de senhas!");

    // Solicita informações do usuário
    println!("Digite o e-mail:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Falha ao ler a entrada");

    println!("Digite o site:");
    let mut site = String::new();
    io::stdin().read_line(&mut site).expect("Falha ao ler a entrada");

   // Pergunta se o usuário quer fornecer sua própria senha
   println!("Deseja inserir sua própria senha? (S/N)");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
   let use_custom_password = input.trim().to_lowercase() == "s";

   let password = if use_custom_password {
       // Solicita a senha do usuário
       println!("Digite sua senha:");
       let mut custom_password = String::new();
       io::stdin().read_line(&mut custom_password).expect("Falha ao ler a entrada");
       custom_password.trim().to_string()
   } else {
       // Gera a senha aleatória
       println!("Digite o comprimento da senha:");
       let mut length_str = String::new();
       io::stdin().read_line(&mut length_str).expect("Falha ao ler a entrada");
       let length: usize = length_str.trim().parse().expect("Erro ao converter para número");

       generate_password(length)
   };

    // Obtém o caminho do arquivo
    let file_path = get_file_path();

    // Cria ou abre o arquivo para escrita
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .expect("Erro ao abrir o arquivo");

    // Escreve as informações no arquivo
    writeln!(file, "Email: {}", email.trim()).expect("Erro ao escrever no arquivo");
    writeln!(file, "Site: {}", site.trim()).expect("Erro ao escrever no arquivo");
    writeln!(file, "Senha: {}", password).expect("Erro ao escrever no arquivo");
    writeln!(file, "\n").expect("Erro ao escrever no arquivo");

    println!("As informações foram salvas no caminho: {}",file_path);

    // Aguarda por 3 segundos antes de fechar
    thread::sleep(Duration::from_secs(5));
}
