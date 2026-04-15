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
| `STORAGE_PATH`  | `./storage`     | Where uploaded files are stored (local mode) |
| `DATABASE_PATH` | `./data/cdn.db` | SQLite database file                         |
| `STORAGE_MODE`  | `local`         | Storage backend: `local` or `s3`             |

### S3 Mode

Set `STORAGE_MODE=s3` and the variables below. Works with AWS S3, MinIO,
Cloudflare R2, Backblaze B2 and other S3-compatible services.

- Uploads are forwarded from the server to S3 (`PutObject`).
- Downloads return a **302 redirect** to a presigned `GetObject` URL
  (TTL configurable via `S3_PRESIGN_TTL`, default 1h). The client fetches
  bytes directly from S3 — the CDN server does not proxy the file body,
  so there's no egress through it.
- Missing objects return the regular 404 page (a `HeadObject` check runs
  before signing).

| Variable              | Default      | Description                                                                      |
|-----------------------|--------------|----------------------------------------------------------------------------------|
| `S3_BUCKET`           | *(required)* | Bucket name                                                                      |
| `S3_ACCESS_KEY`       | *(required)* | Access key ID                                                                    |
| `S3_SECRET_KEY`       | *(required)* | Secret access key                                                                |
| `S3_REGION`           | `us-east-1`  | Region                                                                           |
| `S3_ENDPOINT`         | *(empty)*    | Custom endpoint URL for non-AWS providers (e.g. `http://minio:9000`)             |
| `S3_FORCE_PATH_STYLE` | `true`       | Use path-style addressing (`host/bucket/key`). Set `false` for AWS virtual-host. |
| `S3_PRESIGN_TTL`      | `3600`       | Presigned download URL lifetime in seconds.                                      |

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
