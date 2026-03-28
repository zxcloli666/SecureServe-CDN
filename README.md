# SecureServe CDN

Lightweight file storage & delivery server. Rust + Actix-web + SQLite.

## Quick Start

```bash
docker compose up -d
```

## Configuration

| Variable        | Default         | Description                                  |
|-----------------|-----------------|----------------------------------------------|
| `PORT`          | `3000`          | Server port                                  |
| `ADMIN_TOKEN`   | *(empty)*       | Admin auth token. **Empty = admin disabled** |
| `STORAGE_PATH`  | `./storage`     | Where uploaded files are stored              |
| `DATABASE_PATH` | `./data/cdn.db` | SQLite database file                         |

## API

### Upload Flow

Uploads use a two-step process: sign, then upload.

**1. Sign upload** — authorize a file upload (returns a one-time token, valid 30 min)

```
POST /api/sign-upload
Authorization: Bearer <upload-token>
Content-Type: application/json

{
  "path": "images/photo.jpg",
  "size": 102400,
  "content_type": "image/jpeg"
}
```

```json
{ "token": "one-time-upload-token", "expires_at": 1711612800 }
```

**2. Upload file**

```
POST /api/upload
Content-Type: multipart/form-data

file: <binary>
token: <one-time-upload-token>
```

**3. Access file**

```
GET /images/photo.jpg
```

### Token Management (requires `ADMIN_TOKEN`)

| Method   | Endpoint           | Description                       |
|----------|--------------------|-----------------------------------|
| `GET`    | `/api/tokens`      | List all upload tokens            |
| `POST`   | `/api/tokens`      | Create token `{"name": "my-app"}` |
| `DELETE` | `/api/tokens/{id}` | Delete token                      |
| `POST`   | `/api/auth/verify` | Verify admin token                |

All admin endpoints require `Authorization: Bearer <admin-token>`.

### Admin UI

Open `/admin` in browser — manage upload tokens through a web interface.

## Docker Compose

```yaml
services:
  cdn:
    image: ghcr.io/zxcloli666/secureserve-cdn:latest
    restart: always
    ports:
      - "3000:3000"
    environment:
      - ADMIN_TOKEN=your-secret-token
    volumes:
      - ./storage:/storage
      - ./data:/data
```

## Release

Commit to `main` with `!release: patch`, `!release: minor`, or `!release: major` in the message.

GitHub Actions will create a git tag and push the Docker image to GHCR.

## Build from Source

```bash
cargo build --release
```

## Path Restrictions

- `..` — blocked (path traversal)
- `/` prefix — blocked
- `api/` prefix — blocked (reserved for API routes)
