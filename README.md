# Jogo de Adivinhação em Rust

Este é um simples jogo de adivinhação desenvolvido em Rust. O programa gera um número secreto aleatório e o jogador deve tentar adivinhar qual é esse número.

## Pré-requisitos

* **Rust:** Certifique-se de que o Rust esteja instalado em seu sistema.

### Verificando a instalação do Rust

Para verificar se o Rust está instalado, abra um terminal e execute o seguinte comando:

    ```bash
    rustc --version
    ```

Se o Rust estiver instalado, você verá a versão do compilador Rust. Caso contrário, siga as instruções de instalação abaixo.

### Instalação do Rust

Recomenda-se a instalação através do rustup, um instalador e gerenciador de versões do Rust.

#### Linux (Debian/Ubuntu)

1.  Abra um terminal e execute o seguinte comando:

    ```bash
    sudo apt install build-essential
    ```

E enseguida o comando:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://www.google.com/search?q=https://sh.rustup.rs) | sh
    ```

2.  Siga as instruções na tela.
3.  Após a instalação, feche e reabra o terminal para que as alterações no PATH sejam aplicadas.

#### Linux (Fedora/CentOS)

1.  Abra um terminal e execute o seguinte comando:

    ```bash
    sudo dnf install gcc g++
    ```

E enseguida o comando:

    ```bash
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

    ```

2.  Siga as instruções na tela.
3.  Após a instalação, feche e reabra o terminal para que as alterações no PATH sejam aplicadas.


#### Windows

1.  Acesse o site oficial do Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2.  Baixe o instalador `rustup-init.exe`.
3.  Execute o instalador e siga as instruções na tela.
4.  Após a instalação, feche e reabra o terminal (PowerShell ou CMD).

#### macOS

1.  Abra um terminal e execute o seguinte comando:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://www.google.com/search?q=https://sh.rustup.rs) | sh
    ```

2.  Siga as instruções na tela.
3.  Após a instalação, feche e reabra o terminal.

## Como Jogar

1.  **Clone o repositório:**

    ```bash
    git clone [https://github.com/dolthub/dolt](https://github.com/dolthub/dolt)
    cd nome-do-seu-projeto
    ```

2.  **Compile e execute o jogo:**

    ```bash
    cargo run
    ```

3.  O jogo irá gerar um número secreto entre 1 e 100.
4.  O jogador deve inserir um palpite no console.
5.  O jogo irá informar se o palpite está muito alto, muito baixo ou correto.
6.  O jogo continua até que o jogador adivinhe o número secreto.

## Dependências

* **rand:** Usado para gerar o número secreto aleatório.
* **cargo:** Usado para construir e gerenciar o projeto Rust.

## Como Contribuir

1.  Faça um fork do repositório.
2.  Crie uma branch com a sua feature (`git checkout -b feature/minha-feature`).
3.  Faça commit das suas mudanças (`git commit -am 'Adiciona nova feature'`).
4.  Faça push para a branch (`git push origin feature/minha-feature`).
5.  Abra um Pull Request.

## Licença

Este projeto está sob a licença [Nome da sua licença].