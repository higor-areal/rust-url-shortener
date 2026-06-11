# 🔗 Rust URL Shortener

Um encurtador de URLs desenvolvido em Rust utilizando Axum e Redis.

O projeto permite:

* Criar URLs encurtadas
* Redirecionar para a URL original
* Listar links cadastrados
* Remover links
* Armazenar dados no Redis

---

# 🦀 Tecnologias Utilizadas

* Rust
* Axum
* Tokio
* Redis
* Deadpool Redis
* Serde
* Rand

---

# 📂 Estrutura do Projeto

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
├── repositories/
│   ├── mod.rs
│   └── redis_repository.rs
│
├── reponses/
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

# 📦 Modelos

## Link

Representa um link armazenado no Redis.

```rust
pub struct Link {
    pub code: String,
    pub original_url: String,
    pub clicks: u32,
}
```

---

## NewLink

Payload utilizado para criar um novo link.

```rust
pub struct NewLink {
    pub url: String,
}
```

---

# 🗄️ Persistência

O projeto utiliza Redis para armazenar os links.

Cada link é salvo utilizando um código aleatório gerado pela função:

```rust
short_code(12)
```

Os dados armazenados são:

```txt
url
clicks
```

Além disso, os códigos são adicionados à lista:

```txt
links
```

para posterior consulta.

---

# 🌐 Rotas

## GET /

Healthcheck da aplicação.

### Resposta

```json
{
  "message": "Rust URL Shortener API"
}
```

---

## POST /shorten

Cria uma URL encurtada.

### Payload

```json
{
  "url": "https://google.com"
}
```

### Resposta

```json
{
  "status_code": 201,
  "short_code": "abc123xyz789"
}
```

---

## GET /r/

Busca o código informado e realiza redirecionamento para a URL original.

Exemplo:

```http
GET /r/abc123xyz789
```

Resposta:

```txt
302 Redirect
```

---

## GET /links

Retorna os links cadastrados.

---

## DELETE /links/

Remove um link cadastrado.

Exemplo:

```http
DELETE /links/abc123xyz789
```

---

# ⚙️ Arquitetura

O projeto segue uma separação simples de responsabilidades:

### Handlers

Responsáveis pelas rotas HTTP.

Arquivo:

```txt
handlers/url_handler.rs
```

---

### Models

Estruturas de entrada e saída da aplicação.

Arquivo:

```txt
models/link.rs
```

---

### Repository

Camada responsável pela comunicação com o Redis.

Arquivo:

```txt
repositories/redis_repository.rs
```

---

### State

Estado compartilhado da aplicação.

Arquivo:

```txt
state/app_state.rs
```

---

# 🚀 Executando

Inicie um Redis local:

```bash
redis-server
```

Execute a aplicação:

```bash
cargo run
```

Servidor:

```txt
http://localhost:3000
```

---

# 📚 Aprendizados

Este projeto foi utilizado para estudar:

* Axum
* Rotas HTTP
* Estado compartilhado
* Redis
* Pool de conexões
* Serialização com Serde
* Organização em módulos
* Repository Pattern
* Operações assíncronas com Tokio

---

# 🔧 Melhorias Futuras

Algumas melhorias que podem ser adicionadas futuramente:

* Validação de URL
* Contador de cliques durante o redirecionamento
* Expiração de links
* Códigos personalizados
* Tratamento de erros HTTP mais detalhado
* Variáveis de ambiente para configuração do Redis
* Testes automatizados
