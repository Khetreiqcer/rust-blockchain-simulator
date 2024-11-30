
# Rust Blockchain Simulator

Este é um simulador básico de blockchain desenvolvido em **Rust**, que implementa conceitos fundamentais de blockchain, como blocos, transações, hashing e Proof-of-Work. É uma ferramenta educativa para entender como blockchains funcionam por trás dos panos.

## 🧱 Funcionalidades
- **Blocos encadeados**: Cada bloco armazena transações e está conectado ao anterior via hash.
- **Proof-of-Work**: Simula o processo de mineração ajustando o nonce para atender à dificuldade.
- **Validação de blockchain**: Verifica a integridade da cadeia.
- **Persistência**: Salva e carrega a blockchain de um arquivo JSON.
- **Interface CLI**: Interação por linha de comando para adicionar blocos, exibir a blockchain e validar a cadeia.

## 🛠️ Tecnologias Utilizadas
- **Rust**: Linguagem principal para implementar o simulador.
- **Crates**:
  - [`sha2`](https://crates.io/crates/sha2): Para hashing com SHA-256.
  - [`chrono`](https://crates.io/crates/chrono): Para trabalhar com timestamps.
  - [`serde`](https://crates.io/crates/serde) e [`serde_json`](https://crates.io/crates/serde_json): Para serialização e desserialização.

## 🚀 Como Usar

### 1. Clonar o Repositório
```bash
git clone https://github.com/Khetreiqcer/rust-blockchain-simulator.git
cd rust-blockchain-simulator
```

### 2. Instalar Dependências
Certifique-se de que você tem o Rust instalado. Caso não tenha, instale com:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Compilar o Projeto
Compile o projeto usando o comando:
```bash
cargo build
```

### 4. Executar o Simulador
Execute o programa com:
```bash
cargo run
```

### 5. Usar a Interface CLI
Ao executar, você verá um menu interativo:
```plaintext
=== Blockchain CLI ===
1. Add a new block
2. Display the blockchain
3. Validate blockchain
4. Save and exit
```

Escolha uma das opções para interagir com a blockchain:
- **Adicionar um bloco**: Insira transações manualmente.
- **Exibir a blockchain**: Veja todos os blocos e transações.
- **Validar a blockchain**: Verifique a integridade da cadeia.
- **Salvar e sair**: Salva os dados em um arquivo JSON.

## 📄 Estrutura do Projeto
```
rust-blockchain-simulator/
├── src/
│   ├── main.rs          # Interface CLI para o usuário
│   ├── block.rs         # Estrutura e métodos relacionados ao bloco
│   ├── blockchain.rs    # Gerenciamento da blockchain
├── Cargo.toml           # Dependências do projeto
```

## 💡 Conceitos Implementados

1. **Blockchain**:
   - Estrutura em lista encadeada onde cada bloco depende do anterior.
   - Hashes garantem que os dados não foram alterados.

2. **Blocos**:
   - Contêm transações, um timestamp, um hash único e o hash do bloco anterior.
   - O nonce é ajustado para atender à dificuldade no Proof-of-Work.

3. **Proof-of-Work**:
   - Mineração simulada onde o hash do bloco deve começar com um número específico de zeros.

4. **Persistência**:
   - A blockchain é salva em um arquivo JSON e pode ser recarregada em execuções futuras.

5. **Validação**:
   - Confirma que os hashes são consistentes e a cadeia é íntegra.

## 📝 Exemplo de Uso
1. Adicione um bloco com transações:
   ```plaintext
   Sender: Alice
   Receiver: Bob
   Amount: 50
   Transaction added. Add another? (y/n)
   ```

2. Exiba a blockchain:
   ```plaintext
   Block {
       index: 1,
       timestamp: "2024-11-29T12:00:00",
       transactions: [
           Transaction { sender: "Alice", receiver: "Bob", amount: 50.0 },
       ],
       previous_hash: "000abc...",
       hash: "000xyz...",
       nonce: 1423,
   }
   ```

3. Valide a blockchain:
   ```plaintext
   Blockchain is valid!
   ```

## 🛠️ Melhorias Futuras
- **Assinaturas digitais** para autenticar transações.
- **Rede P2P** para comunicação entre nós.
- **Dashboard gráfico** para exibir os blocos e transações.

## 🧑‍💻 Autor
- **[Khetreiqcer](https://github.com/Khetreiqcer)**

Se você gostou deste projeto ou tem sugestões, fique à vontade para contribuir! 🚀
