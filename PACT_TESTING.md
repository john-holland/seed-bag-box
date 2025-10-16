# 🔫 PACT Testing Guide

## Mock API Server

Local mock server implementing all Seed Box Bag Box API endpoints - no AWS, no ShipStation, no CrateJoy signup needed!

## Quick Start

### Start Mock Server
```bash
cd mock-server
cargo run
```

Server starts on: **http://localhost:3000**

### Test Endpoints
```bash
# Health check
curl http://localhost:3000/health

# View PACT contracts
curl http://localhost:3000/api/pact | jq

# Test scan endpoint
curl -X POST http://localhost:3000/api/scan \
  -H "Content-Type: application/json" \
  -d '{"code": "SEED-12345", "type": "Seed", "timestamp": "2025-10-16T12:00:00Z"}'

# List scans
curl http://localhost:3000/api/scans

# Create queue item
curl -X POST http://localhost:3000/api/queue \
  -H "Content-Type: application/json" \
  -d '{"queue_type": "seed_intake", "priority": "normal"}'

# List queue
curl http://localhost:3000/api/queue

# Store seed
curl -X POST http://localhost:3000/api/storage/seeds \
  -H "Content-Type: application/json" \
  -d '{"seed_id": "550e8400-e29b-41d4-a716-446655440000", "species": "tomato", "facility": "Portland", "room": "Cold-1"}'
```

### Use with Web Interface
```bash
# Terminal 1: Start mock server
cd mock-server && cargo run

# Terminal 2: Serve web interface
cd web && python3 -m http.server 8080

# Open browser: http://localhost:8080/manufacturing-queue.html
# Scanner interface will automatically connect to mock server!
```

## API Endpoints

### Scanner System
- `POST /api/scan` - Record barcode scan
- `GET /api/scans` - List all scans

### Queue Management
- `POST /api/queue` - Create queue item
- `GET /api/queue` - List queue items
- `PUT /api/queue/:id/start` - Start processing
- `PUT /api/queue/:id/complete` - Mark complete

### Seed Storage
- `POST /api/storage/seeds` - Store seed with refrigeration
- `GET /api/storage/seeds` - List stored seeds
- `GET /api/storage/guide/:species` - Get storage requirements

### Subscriptions
- `POST /api/subscriptions` - Create subscription ($8/$15/$19)
- `GET /api/subscriptions/:id` - Get subscription details

### Germination
- `POST /api/germination/start` - Start germination
- `GET /api/germination/ready` - List ready to ship

### Greenhouse
- `GET /api/greenhouse/zones` - List zones
- `GET /api/greenhouse/plants` - List plants

### Meta
- `GET /health` - Health check
- `GET /api/pact` - View all PACT contracts

## PACT Contract Format

All endpoints follow PACT contract specification:

```json
{
  "service": "seed-box-bag-box",
  "contracts": {
    "scanner": {
      "POST /api/scan": {
        "request": {
          "body": {
            "code": "string",
            "type": "string",
            "timestamp": "ISO8601"
          }
        },
        "response": {
          "status": 201,
          "body": {
            "scan_id": "uuid",
            "status": "processed"
          }
        }
      }
    }
  }
}
```

## Testing Workflow

### 1. Unit Testing (No Server)
```bash
cargo test
```

### 2. Integration Testing (Mock Server)
```bash
# Start mock server
cd mock-server && cargo run

# Run integration tests
cargo test --test integration
```

### 3. Manual Testing (Browser)
```bash
# Start mock server
cd mock-server && cargo run

# Open web interface
open web/manufacturing-queue.html

# Scan some codes!
```

### 4. Load Testing
```bash
# Start mock server
cd mock-server && cargo run

# Use Apache Bench
ab -n 1000 -c 10 -T application/json -p scan.json http://localhost:3000/api/scan

# Or use k6
k6 run load-test.js
```

## Mock Data

The mock server maintains in-memory state:
- **Scans**: All barcode scans
- **Queue**: Manufacturing queue items
- **Seeds**: Stored seeds with locations
- **Subscriptions**: Active subscriptions

State resets on server restart (no database).

## Switching to Real AWS

When ready to deploy:

1. **Build Lambda functions**:
```bash
cargo lambda build --release --arm64
```

2. **Deploy with SAM**:
```bash
sam build
sam deploy --guided
```

3. **Update web interface**:
```javascript
// Change from:
const API_URL = 'http://localhost:3000';

// To:
const API_URL = 'https://your-api-id.execute-api.us-west-2.amazonaws.com/prod';
```

## CORS Configuration

Mock server has permissive CORS for local development:
- Allows all origins
- Allows all methods
- Allows all headers

**Production**: Restrict CORS in AWS API Gateway.

## Error Responses

All errors follow standard format:
```json
{
  "error": "Error message",
  "status": 400,
  "timestamp": "2025-10-16T12:00:00Z"
}
```

## Development Tips

### Watch Mode
```bash
# Auto-reload on code changes
cargo watch -x run
```

### Pretty JSON Logs
```bash
# Pipe through jq
curl http://localhost:3000/api/pact | jq .
```

### Test Multiple Clients
```bash
# Terminal 1: Server
cargo run

# Terminal 2: Scanner gun simulation
while true; do
  curl -X POST http://localhost:3000/api/scan \
    -H "Content-Type: application/json" \
    -d "{\"code\": \"SEED-$(uuidgen)\", \"type\": \"Seed\", \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}"
  sleep 1
done

# Terminal 3: Watch scans
watch -n 1 'curl -s http://localhost:3000/api/scans | jq ".count"'
```

## Contract Verification

Verify your Lambda functions match the PACT contracts:

```bash
# Get contracts from mock server
curl http://localhost:3000/api/pact > pact-contracts.json

# Run verification (future)
pact-verifier --contracts pact-contracts.json --provider http://localhost:3001
```

## Why PACT?

1. **No AWS signup needed** - Test locally
2. **No external dependencies** - ShipStation, CrateJoy, etc.
3. **Fast feedback** - Instant responses
4. **Contract-first** - Define API before implementation
5. **CI/CD friendly** - Run in pipelines
6. **Documented** - Contracts serve as API docs

## Next Steps

Once you've tested with the mock server:
1. ✅ Fix any contract mismatches
2. ✅ Add authentication (API keys)
3. ✅ Deploy to AWS
4. ✅ Connect to real ShipStation/CrateJoy
5. ✅ Set up DynamoDB tables
6. ✅ Configure secrets in AWS Secrets Manager

---

**Keep Portland Weird** 🕷️🦇  
*Test early, test often, test weird*

