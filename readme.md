# 弁財天 Benzaiten

Catalogo personale per Manga, Anime e altri media. Gestisci la tua collezione con tag dinamici, autori, stati di rilascio per paese, progressi di lettura/visione e prezzi di acquisto.

## Funzionalità

- 📚 **Catalogo** — Gestione di Manga, Anime, Film e Musica con titoli originali, sinossi, anno
- 🏷️ **Tag dinamici** — Crea e assegna tag (Isekai, Shonen, Seinen…) senza rilasci
- 👤 **Autori & Staff** — Autori, illustratori, registi con ruoli per ogni opera
- 🌏 **Stato per paese** — Traccia stato separato per Italia/Giappone/USA (In corso, Completato, Interrotto…)
- 📖 **Libreria personale** — Episodi letti/visti, volumi posseduti, varianti, prezzi
- ⭐ **Valutazioni** — Rating personale 1-10
- 🔐 **Autenticazione** — Keycloak OIDC, sessione server-side con cookie HTTP-only

## Architettura

```
┌─────────────────┐    HTTP/BFF     ┌──────────────────┐    SQL    ┌──────────────┐
│  Next.js 14     │ ──────────────► │  Rust/Axum       │ ────────► │ PostgreSQL   │
│  (Frontend/BFF) │ ◄────────────── │  (Backend API)   │          └──────────────┘
└─────────────────┘    JSON         └──────────────────┘    S3     ┌──────────────┐
         │                                  │              ────────► │    MinIO     │
         │ OIDC                             │                        └──────────────┘
         ▼                                  │ JWT validate
┌─────────────────┐                ┌──────────────────┐
│    Keycloak     │ ◄──────────────│  (JWT validator) │
└─────────────────┘                └──────────────────┘
```

**Backend (Rust)** — Domain-Driven Design (DDD) con repository pattern per futura migrazione a DB a grafi (Neo4j/ArangoDB):
- `domain/` — Entità pure, value objects, trait dei repository (DB-agnostic)
- `application/` — Servizi applicativi, DTO
- `infrastructure/` — Implementazioni PostgreSQL, MinIO, Keycloak
- `api/` — Handler Axum, middleware, extractor

**Frontend (Next.js)** — BFF pattern: il browser non vede mai JWT in chiaro
- Autenticazione via cookie HTTP-only con next-auth
- Chiamate API sempre attraverso il proxy BFF `/api/proxy/`

## Stack Tecnologico

| Layer | Tecnologia |
|-------|-----------|
| Frontend | Next.js 14 (App Router), React, TypeScript, Tailwind CSS |
| Auth Frontend | next-auth + Keycloak provider (cookie HTTP-only) |
| Backend | Rust + Axum 0.7 |
| Database | PostgreSQL 16 + SQLx |
| Storage Immagini | MinIO (S3-compatible) |
| Auth Provider | Keycloak 23 |
| Deploy | Docker Compose |

## Avvio Rapido

### Prerequisiti
- Docker e Docker Compose
- Git

### 1. Clona e configura

```bash
git clone <repo-url>
cd benzaiten
cp .env.example .env
# Modifica .env con le tue password
```

### 2. Avvia tutto

```bash
docker-compose up -d
```

I servizi saranno disponibili su:
- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **Keycloak Admin**: http://localhost:8180 (admin / password da .env)
- **MinIO Console**: http://localhost:9001

### 3. Configura Keycloak

1. Vai su http://localhost:8180 e accedi come admin
2. Crea un nuovo realm chiamato `benzaiten`
3. Crea un client `benzaiten-web` con:
   - Client type: OpenID Connect
   - Valid redirect URIs: `http://localhost:3000/*`
   - Web origins: `http://localhost:3000`
4. Copia il client secret in `.env` → `KEYCLOAK_CLIENT_SECRET`
5. Crea un utente di test

### 4. Crea il bucket MinIO

1. Vai su http://localhost:9001 e accedi con le credenziali da `.env`
2. Crea un bucket chiamato `benzaiten`
3. (Opzionale) Rendi il bucket pubblico per le copertine

## Sviluppo Locale

### Backend
```bash
cd backend
cp .env.example .env  # o esporta DATABASE_URL
cargo run
```

### Frontend
```bash
cd frontend
cp .env.local.example .env.local
# Modifica con i valori corretti
npm install
npm run dev
```

## Schema Database

```
works ←──── work_tags ────→ tags
  │
  ├──── work_people ────→ people
  │
  ├──── work_release_status (IT, JP, US…)
  │
  ├──── images (copertine, banner)
  │
  └──── user_library_items (collezione personale)
```

## API Endpoints

### Opere (Works)
| Metodo | Endpoint | Descrizione |
|--------|----------|-------------|
| GET | `/api/works` | Lista opere (filtra per tipo, cerca per testo) |
| GET | `/api/works/:id` | Dettaglio opera |
| POST | `/api/works` | Crea nuova opera |
| PUT | `/api/works/:id` | Modifica opera |
| DELETE | `/api/works/:id` | Elimina opera |
| POST | `/api/works/:id/tags` | Aggiungi tag |
| DELETE | `/api/works/:id/tags/:tag_id` | Rimuovi tag |
| POST | `/api/works/:id/people` | Aggiungi persona/ruolo |

### Tag
| Metodo | Endpoint | Descrizione |
|--------|----------|-------------|
| GET | `/api/tags` | Lista tag |
| POST | `/api/tags` | Crea tag |
| PUT | `/api/tags/:id` | Modifica tag |
| DELETE | `/api/tags/:id` | Elimina tag |

### Libreria
| Metodo | Endpoint | Descrizione |
|--------|----------|-------------|
| GET | `/api/library` | La mia libreria |
| POST | `/api/library` | Aggiungi a libreria |
| PUT | `/api/library/:id` | Aggiorna progresso/note |
| DELETE | `/api/library/:id` | Rimuovi da libreria |

### Health
| Metodo | Endpoint | Descrizione |
|--------|----------|-------------|
| GET | `/api/health` | Stato del servizio |

## Decisioni Architetturali

### Perché DDD + Repository Pattern?
Il layer `domain/repositories/` contiene solo **trait** Rust (interfacce). Le implementazioni PostgreSQL vivono in `infrastructure/database/postgres/`. Questo permette di aggiungere una nuova implementazione Neo4j/ArangoDB in `infrastructure/database/neo4j/` **senza toccare domain né application layer**.

### Perché PostgreSQL e non un DB a Grafi?
Per MVP e uso personale, PostgreSQL offre:
- Relazioni many-to-many pulite (tag, autori)
- JSONB per metadati flessibili per tipo di media
- Full-text search integrata
- Query complesse (filtra per tag + paese + tipo)

Grazie al repository pattern, la migrazione a Neo4j resta possibile in futuro.

### Perché Next.js BFF?
Il browser non riceve mai il JWT di Keycloak. Il token è gestito server-side da next-auth (cookie HTTP-only). Il frontend chiama `/api/proxy/` → Next.js inietta il Bearer token → Backend Rust.

## Deploy su QNAP NAS

```bash
# Sul NAS, assicurati che Docker e Docker Compose siano installati
# Tramite Container Station o SSH:

git clone <repo-url> /share/benzaiten
cd /share/benzaiten
cp .env.example .env
nano .env  # Imposta le password

docker-compose up -d
```

> ⚠️ **Sicurezza**: Per produzione, modifica il validatore JWT in `backend/src/infrastructure/auth/keycloak.rs` per verificare la firma del token con le JWKS di Keycloak.

