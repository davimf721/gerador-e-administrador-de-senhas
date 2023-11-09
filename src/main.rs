use std::fs::OpenOptions;
use std::io::{self, Write};
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

fn main() {
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
   
    // Cria ou abre o arquivo para escrita
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("H:/segurança/administrador-informacoes/src/senhas/minhasSenhas.txt")
        .expect("Erro ao abrir o arquivo");

    // Escreve as informações no arquivo
    writeln!(file, "Email: {}", email.trim()).expect("Erro ao escrever no arquivo");
    writeln!(file, "Site: {}", site.trim()).expect("Erro ao escrever no arquivo");
    writeln!(file, "Senha: {}", password).expect("Erro ao escrever no arquivo");
    writeln!(file, "\n").expect("Erro ao escrever no arquivo");
    println!("As informações foram salvas no arquivo minhas senhas.txt");
}
