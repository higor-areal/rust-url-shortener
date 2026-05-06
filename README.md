# Rust URL Shortener API 🦀🔗

Uma API simples de encurtamento de links construída com Rust e Axum.

Este projeto foi criado com foco em aprendizado de backend e aprofundamento em Rust, praticando conceitos como:

- APIs REST
- roteamento com Axum
- async/await com Tokio
- gerenciamento de estado compartilhado
- HashMap
- geração de códigos aleatórios
- modularização de projeto

---

## Funcionalidades

- Criar link encurtado
- Buscar URL original a partir de código curto
- Listar links cadastrados
- Contabilizar acessos por link

---

## Stack

- Rust
- Axum
- Tokio
- Serde
- Serde JSON
- Rand

---

## Estrutura do projeto

```txt
src/
├── handlers/
│   ├── mod.rs
│   └── url_handler.rs
│
├── models/
│   ├── mod.rs
│   └── link.rs
│
├── responses/
│   ├── mod.rs
│   └── response.rs
│
├── state/
│   ├── mod.rs
│   └── app_state.rs
│
└── main.rs
```

---

## Modelos

### Link

```rust
pub struct Link {
    pub original_url: String,
    pub clicks: u32,
}
```

Representa um link salvo na memória.

---

### NewLink

```rust
pub struct NewLink {
    pub url: String,
}
```

Payload recebido para criação de novo link.

---

## Estado da aplicação

A aplicação armazena links em memória usando HashMap.

```rust
HashMap<String, Link>
```

Onde:

- key = short_code
- value = Link

Exemplo:

```txt
abc123 -> https://google.com
```

---

## Endpoints

### GET /

Health route.

Response:

```json
{
  "message": "Rust URL Shortener API"
}
```

---

### POST /shorten

Cria novo link curto.

Request:

```json
{
  "url": "https://www.youtube.com/watch?v=123"
}
```

Response:

```json
{
  "status_code": 201,
  "short_code": "ab12CD"
}
```

---

### GET /r/{code}

Busca URL original pelo código.

Exemplo:

```txt
GET /r/ab12CD
```

Response:

```json
{
  "url": "https://www.youtube.com/watch?v=123"
}
```

Também incrementa contador de acessos.

---

### GET /links

Lista todos links cadastrados.

Response:

```json
[
  {
    "code": "ab12CD",
    "original_url": "https://www.youtube.com/watch?v=123",
    "clicks": 4
  }
]
```

---

### DELETE /links/{code}

Remove link salvo.

Response:

```json
{
  "status_code": 200,
  "message": "deleted"
}
```

---

## Regras de negócio

- URL não pode ser vazia
- código curto deve ser aleatório
- código deve ser único
- acessos incrementam contador

---

## Como rodar

Clone:

```bash
git clone https://github.com/higor-areal/rust-url-shortener.git
```

Entre na pasta:

```bash
cd rust-url-shortener
```

Execute:

```bash
cargo run
```

Servidor:

```txt
http://localhost:3000
```

---

## Fluxo interno

1. usuário envia URL
2. API gera código aleatório
3. salva em HashMap
4. retorna código
5. usuário consulta código
6. API retorna URL original

---

## Conceitos praticados

- ownership
- borrowing
- Arc
- Mutex
- HashMap
- modularização
- handlers
- shared state
- path params
- JSON serialization
- Result e error handling

---

## Próximas melhorias

- persistência com SQLite
- persistência com Supabase
- expiração de links
- autenticação
- redirect HTTP real
- testes automatizados

---

## Objetivo

Projeto educacional criado para praticar Rust backend e construção de APIs reais.