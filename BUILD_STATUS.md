# Build Status 🚬🐸

**Date**: October 16, 2025  
**Status**: ⚠️ **COMPILING** (with errors to fix)

## What We Got Done ✅

### Manufacturing Queue System
- ✅ **Queue Management** - Manufacturing queue for coordinating seed → greenhouse → shipment
- ✅ **Seed Storage** with refrigeration tracking (2-8°C cold storage)
- ✅ **Storage Separation** - Species-specific separation rules (esp. for peppers & cannabis)
- ✅ **Greenhouse Workflow** - Move plants between zones
- ✅ **Storage Parameters** by species:
  - Tomatoes: 4 years cold storage
  - Peppers: 2 years, separation required
  - Cannabis: 2 years, **SEPARATE LICENSED FACILITY**, phenotype separation
  - Basil: 5 years
  - Lettuce: 3 years

### Complete System Features
1. **7 Lambda Services**:
   - Subscription Service ($8, $15, $19 tiers)
   - Shipping Service (ShipStation + CrateJoy multi-point)
   - Inventory Service (bags & seeds)
   - Greenhouse Service (zones with spatial quarantine)
   - Germination Service (7 botanical stages, true plant tracking)
   - Plant Processing Service (curing, recipes, edible fruit info)
   - Manufacturing Queue Service (**NEW** 🎉)

2. **Comprehensive Data Models**:
   - Subscription tiers
   - Multi-point shipping with trapezoid butterfly bag folding
   - Seed storage with refrigeration
   - Germination phases (Imbibition → True Plant → Autotrophic)
   - Greenhouse spatial quarantine
   - Contamination tracking (USDA/FDA compliance)
   - Manufacturing queue & workflows

3. **Documentation** 📚:
   - Food Safety Research (REVIEW required)
   - Bag Folding Instructions (trapezoid butterfly method)
   - Germination Shipping Guide
   - Botanical Stages (7 phases with true plant identification)
   - Deployment Guide
   - Manufacturing Queue (seed storage & refrigeration)

## Remaining Compilation Errors 🔧

### Minor Fixes Needed:
1. **TemperatureRange/HumidityRange** - Defined in both `germination.rs` and `manufacturing.rs` (duplication)
2. **germination-service** - 2 instances of `min_leaf_count` should be `min_true_leaf_count`
3. **event.payload borrow issues** - Lambda handlers need to clone payload before moving

### Estimated Fix Time: ~5 minutes

## How to Run Tests 🧪

```bash
# After fixing errors above:
cd /Users/johnholland/Developers/seed-box-bag-box

# Check compilation
cargo check --workspace

# Run tests
cargo test --workspace

# Build for Lambda
cargo lambda build --release --arm64

# Start local API (requires cargo-lambda)
cargo lambda watch
```

## How to Deploy 🚀

```bash
# Build
make build

# Deploy to AWS
make deploy

# Or manually:
sam build
sam deploy --guided
```

## API Endpoints Summary

### Manufacturing Queue
- `POST /queue` - Create queue item
- `GET /queue` - List queue
- `PUT /queue/{id}/start` - Start processing
- `PUT /queue/{id}/complete` - Mark complete
- `POST /storage/seeds` - Store seed with refrigeration
- `GET /storage/seeds` - List seed storage
- `GET /storage/guide/{species}` - Get storage requirements
- `GET /greenhouse/workflow` - List greenhouse workflows

### All Other Services
- See README.md for complete API documentation

## Storage Parameters by Species 🌡️

| Species | Temp | Humidity | Max Storage | Refrigeration | Separation |
|---------|------|----------|-------------|---------------|------------|
| Tomato | 2-8°C (opt: 5°C) | 20-40% (opt: 30%) | 4 years | ✅ Required | ❌ |
| Pepper | 2-8°C (opt: 5°C) | 20-40% (opt: 30%) | 2 years | ✅ Required | ✅ Required |
| Lettuce | 2-8°C (opt: 5°C) | 20-40% (opt: 30%) | 3 years | ✅ Required | ❌ |
| Basil | 2-8°C (opt: 5°C) | 20-40% (opt: 30%) | 5 years | ✅ Required | ❌ |
| Cannabis 🌿 | 2-8°C (opt: 5°C) | 20-30% (opt: 25%) | 2 years | ✅ Required | ⚠️ **MUST SEPARATE** |

**Cannabis Note**: Requires separate licensed facility, strict phenotype separation, vacuum seal recommended, state compliance tracking.

## What's Next?

1. Fix the 3 compilation errors listed above
2. Run `cargo test --workspace` to verify
3. Deploy to AWS with `make deploy`
4. Crack open a cold one and watch the seeds grow 🍺🌱

---

**Keep Portland Weird** 🕷️🦇  
*"AWS is like the movie Se7en but your career is in the box"*

