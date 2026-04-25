# 弁財天 Benzaiten

Il tuo catalogo personale di Manga, Anime, Film e Musica.

## Struttura del Progetto

```
benzaiten/
├── docker-compose.dev.yml      # Infrastruttura + dev containers
├── .env.dev                    # Configurazione development
├── scripts/
│   └── init-multiple-dbs.sh    # Script init PostgreSQL
└── src/
    ├── UI/                     # Next.js frontend (Bun)
    ├── BFF/                    # Backend-For-Frontend (Bun + Hono)
    └── Backend/                # Backend Rust (Axum)
```

## Sviluppo Locale

### Prerequisiti

- [Docker](https://www.docker.com/) e Docker Compose
- [VS Code](https://code.visualstudio.com/)
- Estensione VS Code [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Setup Iniziale

1. Clona il repository:
   ```bash
   git clone https://github.com/Amebus/benzaiten.git
   cd benzaiten
   ```

2. Avvia l'infrastruttura:
   ```bash
   docker compose -f docker-compose.dev.yml --env-file .env.dev up -d
   ```

3. Verifica che tutti i servizi siano attivi:
   ```bash
   docker compose -f docker-compose.dev.yml ps
   ```

### Aprire i Devcontainer

Ogni componente ha il proprio devcontainer. Per aprirlo:

1. In VS Code, apri la cartella del componente desiderato:
   - `src/UI` per il frontend
   - `src/BFF` per il BFF
   - `src/Backend` per il backend Rust

2. Quando VS Code rileva il `.devcontainer`, clicca su **"Reopen in Container"** (oppure usa il comando `Dev Containers: Reopen in Container` dalla palette comandi `Ctrl+Shift+P`).

3. VS Code si connetterà al container già avviato da `docker-compose.dev.yml`.

### URL Servizi Disponibili

| Servizio      | URL                         | Descrizione                                       |
|---------------|-----------------------------|----------------------------------------------------|
| Traefik proxy | http://localhost:50080      | Reverse proxy (UI `/`, BFF `/bff`, API `/api`)    |
| Traefik UI    | http://localhost:50088      | Dashboard Traefik                                  |
| UI (Next.js)  | http://localhost:53000      | Frontend applicazione (accesso diretto)            |
| BFF           | http://localhost:54000      | Backend-For-Frontend (accesso diretto)             |
| Backend Rust  | http://localhost:58000      | API Backend (accesso diretto)                      |
| Keycloak      | http://localhost:58080      | Identity Provider (admin/admin)                    |
| Adminer       | http://localhost:58081      | Gestione database PostgreSQL                       |
| PostgreSQL    | localhost:25432             | Database (dev_user/dev_password)                   |
| Redis         | localhost:26379             | Session/Cache store                                |

**Nota:** Il mount della cache Bun (`src/UI/.devcontainer/devcontainer.json`) usa il percorso `~/Progetti/informatica/bun/cache`. Se la tua struttura di cartelle è diversa, aggiorna il campo `source` nel `mounts` del devcontainer.json della UI prima di aprire il container.

### Fermare i Servizi

```bash
docker compose -f docker-compose.dev.yml down
```

Per rimuovere anche i volumi (attenzione: cancella i dati):
```bash
docker compose -f docker-compose.dev.yml down -v
```

### Troubleshooting

**Keycloak non si avvia**
Keycloak dipende da PostgreSQL. Attendi che il database sia healthy prima che Keycloak parta. Puoi verificare con:
```bash
docker compose -f docker-compose.dev.yml logs keycloak
```

**Errori di permessi sullo script init PostgreSQL**
Assicurati che lo script sia eseguibile:
```bash
chmod +x scripts/init-multiple-dbs.sh
```

**Il devcontainer non si connette al container esistente**
Assicurati di aver avviato prima l'infrastruttura con `docker compose up -d`, poi apri VS Code nella cartella del componente.

**Porte già in uso**
Verifica che nessun altro servizio stia usando le porte 3000, 4000, 5432, 6379, 8000, 8080, 8081.

**Pulire tutto e ripartire da zero**
```bash
docker compose -f docker-compose.dev.yml down -v
docker compose -f docker-compose.dev.yml --env-file .env.dev up -d --build
```
