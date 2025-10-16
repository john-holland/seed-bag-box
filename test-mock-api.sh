#!/bin/bash
# Quick test script for mock API server

echo "🔫 Testing Mock API Server"
echo "=========================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Health check
echo -e "${BLUE}1. Health Check${NC}"
curl -s http://localhost:3000/health | jq .
echo ""

# Test scan endpoint
echo -e "${BLUE}2. Scanning a barcode${NC}"
curl -s -X POST http://localhost:3000/api/scan \
  -H "Content-Type: application/json" \
  -d '{"code": "SEED-12345", "type": "Seed", "timestamp": "2025-10-16T12:00:00Z"}' | jq .
echo ""

# List scans
echo -e "${BLUE}3. List scans${NC}"
curl -s http://localhost:3000/api/scans | jq .
echo ""

# Create queue item
echo -e "${BLUE}4. Create queue item${NC}"
curl -s -X POST http://localhost:3000/api/queue \
  -H "Content-Type: application/json" \
  -d '{"queue_type": "SEED_INTAKE", "priority": "normal"}' | jq .
echo ""

# List queue
echo -e "${BLUE}5. List queue${NC}"
curl -s http://localhost:3000/api/queue | jq .
echo ""

# Store seed
echo -e "${BLUE}6. Store seed in cold storage${NC}"
curl -s -X POST http://localhost:3000/api/storage/seeds \
  -H "Content-Type: application/json" \
  -d '{"seed_id": "550e8400-e29b-41d4-a716-446655440000", "species": "tomato", "facility": "Portland", "room": "Cold-1"}' | jq .
echo ""

# Get storage guide
echo -e "${BLUE}7. Get storage guide for tomato${NC}"
curl -s http://localhost:3000/api/storage/guide/tomato | jq .
echo ""

# View PACT contracts
echo -e "${BLUE}8. View PACT contracts${NC}"
curl -s http://localhost:3000/api/pact | jq '.contracts | keys'
echo ""

echo -e "${GREEN}✅ All tests passed!${NC}"

