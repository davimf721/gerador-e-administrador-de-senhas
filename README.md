![capa do projeto](https://github.com/davimf721/gerador-e-administrador-de-senhas/assets/64716204/d1236904-5163-4f48-8810-d75939032cef)

<h1 > Gerador de Senhas e Armazenamento em Arquivo </h1>

![dados da versão](https://img.shields.io/badge/generator-v0.1.0-brightgreen)
 ![data de lançamento](https://img.shields.io/badge/release%20date-november-red)
![GitHub Org's stars](https://img.shields.io/github/stars/davimf721?style=social)

Este é um programa simples em Rust que permite gerar senhas aleatórias ou inserir uma senha personalizada, junto com informações de e-mail e site. As informações são armazenadas em um arquivo de texto no local escolhido pelo usuário.

## Como Utilizar

- Requisitos:

Certifique-se de ter o Rust instalado em seu sistema. Caso contrário, você pode obtê-lo em <https://www.rust-lang.org/tools/install>.<br>

Ou simplesmente execute o arquivo com o nome "administrador-informacoes.exe".

### Clone o repositório

```bash
git clone https://github.com/davimf721/gerador-e-administrador-de-senhas.git
cd gerador-e-administrador-de-senhas
```

### Compilação e Execução

```bash
cargo build
./target/release/gerador-e-administrador-de-senhas
```

### Siga as Instruções

- O programa solicitará informações, como e-mail, site e se você deseja fornecer sua própria senha.
- Se optar por gerar uma senha aleatória, será solicitado o comprimento desejado da senha.
- Em seguida, o programa pedirá que você insira o caminho completo do arquivo onde deseja armazenar as informações.

### Resultado

- O programa salvará as informações no arquivo especificado, incluindo e-mail, site e senha.

## Exemplo de uso

```linux
Digite o e-mail:
usuario@example.com
Digite o site:
example.com
Deseja inserir sua própria senha? (S/N)
N
Digite o comprimento da senha:
12
Digite o caminho completo do arquivo (ex: /caminho/do/arquivo/nome_arquivo.txt):
/meu/caminho/senhas.txt
As informações foram salvas no arquivo /meu/caminho/senhas.txt
```

### Observações

- Certifique-se de ter permissões de escrita no diretório onde deseja armazenar o arquivo.
- Se você escolher fornecer sua própria senha, ela será salva no arquivo como você a inseriu.
- Não se esqueça de fazer um backup do arquivo com suas senhas para uma maior proteção dos seus dados.

#### Esperamos que este programa seja útil para gerar senhas e armazenar informações de forma simples e segura
