# 🇳🇵 Seed Box Bag Box 📦

> **"Portlandia-themed Halloween Edition"** 🕷️🦇  
> A subscription service for bag cleaning, seed collection, and crop growing tracking

## Overview

**Seed Box Bag Box** is a subscription-based service that:
- 🛍️ Collects shopping bags from consumers
- 🧼 Cleans and recycles them
- 🌱 Collects seeds from consumers for crop growing
- 🌿 Germinates seeds and ships young plants back during the **sprouting phase**
- 🏡 Tracks greenhouse operations with spatial quarantine for various crops
- 📦 Returns cleaned bags (either random sampling or your own) via multi-point shipping
- 🌾 Provides plant processing information (eating, curing, prep)

Built with **Rust** on **AWS Lambda** because "AWS is like the movie Se7en but your career is in the box."

## Subscription Tiers

### 💵 Bring Your Own Bags - $8/month
- **Requires**: You send us bags!
- **Get**: Random sampling of cleaned bags back
- **Status**: PendingBags until first shipment received

### 🌟 Standard - $15/month  
- **Requires**: Nothing! We source bags
- **Get**: Random sampling of good fresh bags
- **Feed off**: Other people's bags for your backup supply

### 👑 Premium - $19/month
- **Requires**: You send bags
- **Get**: ONLY your own beloved bags back
- **No random sampling**

## Architecture

Built as a **Rust AWS Lambda microservices** architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                     API Gateway (REST)                       │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Subscription │ │   Shipping   │ │  Inventory   │
│   Service    │ │   Service    │ │   Service    │
│   (Lambda)   │ │   (Lambda)   │ │   (Lambda)   │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       │         ┌──────────────┐        │
       │         │  Greenhouse  │        │
       │         │   Service    │        │
       │         │   (Lambda)   │        │
       │         └──────┬───────┘        │
       │                │                │
       ▼                ▼                ▼
┌─────────────────────────────────────────────────┐
│              DynamoDB Tables                     │
│ • Subscriptions  • Bags      • Seeds            │
│ • Shipments      • Zones     • Plants           │
│ • Processing Guides                             │
└─────────────────────────────────────────────────┘

External Integrations:
├── CrateJoy (Subscription Management)
└── ShipStation (Multi-Point Shipping)
```

## Project Structure

```
seed-box-bag-box/
├── Cargo.toml                     # Workspace configuration
├── template.yaml                  # AWS SAM deployment template
├── lambdas/
│   ├── subscription-service/      # Manages subscriptions & tiers
│   ├── shipping-service/          # Multi-point shipping & ShipStation
│   ├── inventory-service/         # Bags & seeds inventory
│   ├── greenhouse-service/        # Greenhouse zones & plant tracking
│   ├── germination-service/       # Germination tracking & sprout shipment
│   └── plant-processing-service/  # Processing guides & curing protocols
├── shared/
│   ├── models/                    # Shared data models
│   │   ├── subscription.rs        # Subscription tiers & customers
│   │   ├── shipping.rs            # Shipments & packaging
│   │   ├── inventory.rs           # Bags & seeds
│   │   ├── greenhouse.rs          # Zones & spatial quarantine
│   │   ├── plant.rs               # Plant processing & curing
│   │   └── contamination.rs       # Safety & contamination tracking
│   └── database/                  # DynamoDB helpers (TODO)
└── docs/
    ├── FOOD_SAFETY_RESEARCH_REVIEW.md  # 🔍 REQUIRES REVIEW
    └── BAG_FOLDING_INSTRUCTIONS.md     # Trapezoid butterfly method
```

## Key Features

### 🚚 Multi-Point Shipping (ShipStation Integration)

The shipping system uses a clever multi-leg approach:
1. **Leg 1**: Customer → Facility (customer sends bags)
2. **Leg 2**: Facility → Customer (return cleaned bags with return label as final leg)

This avoids breaking shipments into separate send/return cycles!

### 📦 Trapezoid Butterfly Bag Folding Method

Our proprietary packaging process:
1. Fold trapezoid wings on each side
2. Twist handles inward
3. Apply shipping label over wing shape
4. Cover with perforated sticker (cut line)
5. Roller-cut for easy tear perforation
6. Bend halves and wrap with tape
7. Creates Rubik's cube-shaped package (~3" x 3" x 3")

See [BAG_FOLDING_INSTRUCTIONS.md](docs/BAG_FOLDING_INSTRUCTIONS.md) for full details.

### 🏡 Greenhouse Management System

Features:
- **Spatial Quarantine**: Isolation zones with configurable distances
- **Phenotype Designation**: For specific cultivars (pot heads can use the API! 🌿)
- **Contamination Tracking**: Risk levels, quarantine status, contamination history
- **Zone Types**:
  - Standard growing
  - Quarantine (preventive & active)
  - Phenotype isolation
  - Germination
  - Harvest staging

### 🔬 Food Safety & Contamination

Comprehensive tracking system:
- USDA/FDA/WHO/CPSC regulation compliance (pending legal review)
- Contamination reporting (bacterial, fungal, pest, chemical)
- Recall alert tracking
- Safety checklists for bags, seeds, plants, greenhouse
- See [FOOD_SAFETY_RESEARCH_REVIEW.md](docs/FOOD_SAFETY_RESEARCH_REVIEW.md) - **REQUIRES EXPERT REVIEW** 🔍

### 🌾 Plant Processing

Information about:
- **Edible fruit verification**: Determines if plant bears edible fruit
- **Processing methods**: Eating, curing, cooking, fermentation
- **Curing protocols**: Temperature, humidity, duration, phases
- **Recipes**: For edible crops

## Development

### Prerequisites

- Rust 1.75+ with `cargo`
- AWS CLI configured
- AWS SAM CLI
- Zig (for cross-compilation to AWS Lambda)
- Docker (for local testing)

### Building

```bash
# Install cargo-lambda for AWS Lambda builds
cargo install cargo-lambda

# Build all Lambda functions
cargo lambda build --release --arm64

# Build specific service
cargo lambda build --release --arm64 -p subscription-service
```

### Local Testing

```bash
# Start API Gateway locally
sam local start-api

# Invoke specific function
cargo lambda invoke subscription-service --data-file events/create-subscription.json
```

### Deployment

```bash
# Build for Lambda
cargo lambda build --release --arm64

# Package and deploy with SAM
sam build
sam deploy --guided

# Or use AWS CDK (if configured)
```

### Environment Variables

Create `.env` for local development:

```env
SHIPSTATION_API_KEY=your_key_here
SHIPSTATION_API_SECRET=your_secret_here
CRATEJOY_API_KEY=your_cratejoy_key
```

For production, store secrets in **AWS Secrets Manager**:
- `seed-box/shipstation` - ShipStation API credentials
- `seed-box/cratejoy` - CrateJoy API credentials

## API Endpoints

### Subscriptions
- `POST /subscriptions` - Create subscription
- `GET /subscriptions/{id}` - Get subscription details
- `PUT /subscriptions/{id}` - Update subscription

### Shipping
- `POST /shipments` - Create multi-point shipment
- `GET /shipments/packaging-instructions` - Get bag folding instructions
- `POST /shipments/webhook` - ShipStation webhook handler

### Inventory
- `POST /inventory/bags` - Register received bag
- `POST /inventory/seeds` - Register collected seed
- `GET /inventory/bags` - List all bags
- `GET /inventory/seeds` - List all seeds
- `GET /inventory/summary` - Inventory summary

### Greenhouse
- `POST /greenhouse/zones` - Create greenhouse zone
- `POST /greenhouse/plants` - Plant a seed
- `POST /greenhouse/quarantine` - Initiate quarantine
- `GET /greenhouse/zones` - List zones
- `GET /greenhouse/plants` - List plants

### Germination & Sprouts
- `POST /germination/start` - Start germinating a seed
- `POST /germination/observe` - Record daily observation
- `PUT /germination/phase` - Update germination phase
- `POST /germination/shipment` - Prepare sprout shipment
- `GET /germination/ready` - List sprouts ready for shipment
- `GET /germination/{id}` - Get germination record
- `GET /germination/guide/{species}` - Get germination guide for species

### Plant Processing
- `GET /processing/guides?species=tomato` - Get processing guide
- `GET /processing/curing-protocols?plant_type=cannabis` - Get curing protocol
- `GET /processing/recipes?species=tomato` - Get recipes

## Data Models

All models are strongly typed in Rust with serde serialization.

Key models:
- `Subscription` - Customer subscriptions with tiers
- `ShipmentCycle` - Multi-leg shipping with both directions
- `Bag` - Individual bag tracking with status
- `Seed` - Seed collection with contamination checks
- `GreenhouseZone` - Spatial zones with quarantine status
- `Plant` - Individual plant tracking with health status
- `ContaminationReport` - Safety incident tracking

See `shared/models/src/` for full type definitions.

## Testing Strategy

```bash
# Run all unit tests
cargo test

# Run specific service tests
cargo test -p subscription-service

# Integration tests (requires AWS credentials)
cargo test --test integration

# Load testing
# TODO: Add k6 or similar load tests
```

## Monitoring & Logging

All Lambda functions use `tracing` for structured logging:
- Logs sent to CloudWatch
- Metrics tracked with CloudWatch Metrics
- Alerts configured for errors and high latency

## Cost Optimization

Why Rust + Lambda?
- ⚡ **Fast cold starts** (~50ms with Rust)
- 💰 **Low memory usage** (128-512MB sufficient)
- 🚀 **High performance** (handle more requests per Lambda)
- 📈 **Pay only for compute time** (not idle server costs)

Expected costs (rough estimates):
- **Lambda**: $5-20/month for moderate traffic
- **DynamoDB**: $5-15/month (pay-per-request)
- **API Gateway**: $3-10/month
- **ShipStation**: Variable per shipment
- **CrateJoy**: Variable per subscription

## Security Considerations

- ✅ API Gateway with API keys or Cognito auth (TODO)
- ✅ IAM roles with least privilege
- ✅ Secrets in AWS Secrets Manager
- ✅ Input validation on all Lambda handlers
- ✅ CORS properly configured
- ⚠️ Food safety compliance (legal review required)
- ⚠️ Cannabis regulations (if applicable - consult attorney)

## Legal & Compliance

⚠️ **IMPORTANT**: This project involves food safety, seed handling, and potentially regulated crops (cannabis).

**Required before production**:
1. Consult food safety attorney
2. Consult agricultural compliance expert
3. Obtain necessary licenses (state-specific)
4. Review [FOOD_SAFETY_RESEARCH_REVIEW.md](docs/FOOD_SAFETY_RESEARCH_REVIEW.md) with experts
5. Obtain liability insurance
6. If handling cannabis: Separate legal review and licensing

## Contributing

Contributions welcome! Please:
1. Fork the repo
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## License

[Choose appropriate license - MIT, Apache 2.0, etc.]

## Acknowledgments

- Inspired by Portlandia's quirky Portland entrepreneurship 🎃
- Built with love in Rust 🦀
- Deployed on AWS Lambda ⚡
- ShipStation & CrateJoy integrations 📦

---

**Status**: 🚧 In Development  
**Version**: 0.1.0  
**Last Updated**: October 16, 2025

## Contact

For questions, business inquiries, or to report food safety concerns:
- Email: [your-email]
- GitHub Issues: [repo-url]/issues

---

🕷️ Happy Halloween! Keep Portland Weird! 🦇

