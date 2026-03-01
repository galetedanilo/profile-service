# 👤 Profile Service

Um microsserviço de alta performance desenvolvido em **Rust**, focado no gerenciamento de perfis de usuários. O projeto utiliza os princípios de **Domain-Driven Design (DDD)** para garantir um código escalável, testável e de fácil manutenção.

## 🏗️ Arquitetura (DDD)

O projeto está sendo estruturado seguindo as camadas do DDD:

- **Domain:** Entidades, Objetos de Valor e Regras de Negócio (Lógica pura).
- **Application:** Casos de uso (Use Cases) que orquestram a lógica da aplicação.
- **Infrastructure:** Implementações técnicas (MongoDB, adaptadores de rede). (Em desenvolvimento)
- **Presentation/API:** Camada de entrada (Axum Handlers). (Em desenvolvimento)

## 📂 Estrutura de Pastas

```text
src/
├── 📁 domain/         # Regras de negócio e lógica pura
├── 📁 application/    # Orquestração e Casos de Uso
├── 📦 infrastructure/ # Persistência (MongoDB) e Clientes Externos (🚧)
├── 🚀 presentation/   # Camada de API (Axum) e Controllers (🚧)
└── 🦀 main.rs         # Ponto de entrada do microserviço (🚧)
```

## 🚀 Tecnologias Principais

- **Linguagem:** [Rust](https://www.rust-lang.org)
- **Framework Web:** [Axum](https://github.com)
- **Runtime:** [Tokio](https://tokio.rs)
- **Banco de Dados:** [MongoDB](https://www.mongodb.com) (Pendente implementação)
- **Serialização:** [Serde](https://serde.rs) (Pendente implementação)

## 📋 Status dos Endpoints (API)

| Método | Endpoint        | Descrição                 | Status                               |
| :----- | :-------------- | :------------------------ | :----------------------------------- |
| `GET`  | `/health`       | Check de saúde do sistema | 🚧 Em progresso (Application/Domain) |
| `POST` | `/profiles`     | Criar um novo perfil      | 🚧 Em progresso (Application/Domain) |
| `GET`  | `/profiles/:id` | Buscar perfil por ID      | 🚧 Em progresso (Application/Domain) |
| `PUT`  | `/profiles/:id` | Atualizar dados do perfil | 🚧 Em progresso (Application/Domain) |

## 🧪 Testes

A qualidade do projeto é garantida através de **testes unitários** rigorosos, especialmente na camada de **Domain** e **Application**, onde reside a lógica central.

```bash
# Rodar todos os testes unitários
cargo test
```

## 🏁 Configuração Local

1. **Clonar o repositório:**
   ```bash
   git clone https://github.com/galetedanilo/profile-service.git
   cd profile-service
   ```
