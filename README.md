# 👤 Profile Service

Um microsserviço de alta performance desenvolvido em **Rust**, focado no gerenciamento de perfis de usuários. O projeto utiliza os princípios de **Domain-Driven Design (DDD)** para garantir um código escalável, testável e de fácil manutenção.

## 🏗️ Arquitetura (DDD)

O projeto está sendo estruturado seguindo as camadas do DDD:

- **Domain:** Entidades, Objetos de Valor e Regras de Negócio (Lógica pura).
- **Application:** Casos de uso (Use Cases) que orquestram a lógica da aplicação.
- **Infrastructure:** Implementações técnicas (MongoDB, adaptadores de rede). (Em desenvolvimento)
- **Presentation/API:** Camada de entrada (Axum Handlers). (Em desenvolvimento)

## 🛡️ Health & Fitness Functions
   
### Status da Fitness Function

[ ![Nome do Badge](https://github.com/galetedanilo/profile-service/actions/workflows/fitness.yml/badge.svg) ](https://github.com/galetedanilo/profile-service/actions/workflows/fitness.yml)


### Regras de Aptidão (Fitness Rules):
- **Cobertura Mínima:** 80% (Linhas de código)
- **Complexidade Máxima:** Clippy Cognitive Threshold
- **Status:** Automatizado via GitHub Actions

## 📂 Estrutura de Pastas

```text
src/
├── 📁 domain/         # Regras de negócio e lógica pura
├── 📁 application/    # Orquestração e Casos de Uso
├── 📦 infrastructure/ # Persistência (MongoDB) e Clientes Externos
├── 🚀 presentation/   # Camada de API (Axum) e Controllers
└── 🦀 main.rs         # Ponto de entrada do microserviço
```

## 🚀 Tecnologias Principais

- **Linguagem:** [Rust](https://www.rust-lang.org)
- **Framework Web:** [Axum](https://github.com/tokio-rs/axum)
- **Runtime:** [Tokio](https://tokio.rs)
- **Banco de Dados:** [MongoDB](https://www.mongodb.com) (Pendente implementação)
- **Serialização:** [Serde](https://serde.rs) (Pendente implementação)
- **Validação:** [Validify](https://github.com/biblius/validify) (Validação e transformação de dados de entrada)

## 🛠 Fitness Functions

Este projeto utiliza Fitness Functions automatizadas via GitHub Actions:

- Mínimo de 80% de cobertura de testes.
- Complexidade controlada via Clippy.

Se o build falhar, verifique se você adicionou testes para as novas funcionalidades ou se a lógica da sua função pode ser simplificada (refatorada).

## 📋 Status dos Endpoints (API)

| Método | Endpoint        | Descrição                 | Status                               |
| :----- | :-------------- | :------------------------ | :----------------------------------- |
| `GET`  | `/health`       | Check de saúde do sistema | 🚧 Em progresso (Application/Domain) |
| `POST` | `/profiles`     | Criar um novo perfil      | ✅ Concluído |
| `GET`  | `/profiles/:id` | Buscar perfil por ID      | ✅ Concluído |
| `PUT`  | `/profiles/:id` | Atualizar dados do perfil | ✅ Concluído |

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
   cargo run
   ```
