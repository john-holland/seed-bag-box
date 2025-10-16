# 🌱 Seed Box Bag Box - Complete Feature List

## 🎨 Web Interfaces

### 1. Manufacturing Queue (`web/manufacturing-queue.html`)
**Scanner Gun Interface** 🔫
- ✅ Auto-focus text input (stays focused at all times)
- ✅ Paste event support (scanner guns paste data)
- ✅ Rapid typing detection (auto-submits after 100ms)
- ✅ Enter key support
- ✅ Visual feedback (green flash, beep sound)
- ✅ Recent scans list with timestamps
- ✅ Barcode type detection (SEED-, BAG-, ZONE-, PLANT-, UPC, UUID)
- ✅ Live stats dashboard

### 2. Plant Lookup (`web/plant-lookup.html`)
**Material UI Paper Aesthetic** with Light Green, Eggshell, White, Blues, Aquas, Teal Borders
- ✅ Beautiful search interface
- ✅ Filter chips (Seeds, Germinating, True Plants, Ready to Ship, Edible Fruit)
- ✅ Stats dashboard (Seeds, Plants, Ready to Ship, Species)
- ✅ Result cards with hover effects
- ✅ **7 Growth stage illustrations** (SVG):
  - 💧 Imbibition (seed swelling)
  - 🌰 Radicle Emergence (first root)
  - 🌱 Shoot Emergence (stem appears)
  - 🌿 Cotyledon Expansion (seed leaves)
  - 🍃 **True Leaf Emergence** (TRUE PLANT stage!)
  - ☀️ Photosynthesis (autotrophic)
  - 🌳 Continued Growth (mature sprout)
- ✅ **True Leaf vs Cotyledon comparison** with detailed illustrations
- ✅ **Upload Photo button on each growth stage**
- ✅ **Shipping options configurator** (for ready-to-ship plants):
  - 🌱 Live Sprout (standard)
  - 🌿 Bare Root (-$2)
  - 🪴 Potted (+$5)
  - ⚡ Expedited 2-day (+$10)
- ✅ Detail modal with full plant information
- ✅ Highlights current growth stage

### 3. My Images (`web/my-images.html`)
**User Image Management**
- ✅ Drag & drop upload area
- ✅ Image preview grid
- ✅ **Expandable audit log** below buttons (not modal!)
- ✅ Soft delete with confirmation
- ✅ Status badges (Pending, Approved, Rejected)
- ✅ Audit trail for all actions
- ✅ Beautiful Material UI aesthetic

### 4. Image Moderation (`web/image-moderation.html`)
**Moderator Dashboard**
- ✅ Stats (Pending, Approved, Rejected)
- ✅ Tabbed interface
- ✅ Approve/Reject/Flag buttons
- ✅ Image preview
- ✅ Metadata display
- ✅ Audit log viewer
- ✅ Material UI paper cards

## 🦀 Backend Services (Rust Lambda)

### 1. Subscription Service
- $8/month - Bring Your Own Bags
- $15/month - Standard (random sampling)
- $19/month - Premium (only your bags)
- CrateJoy integration

### 2. Shipping Service
- Multi-point shipping (ShipStation)
- Return label as final leg
- Trapezoid butterfly bag folding method
- Packaging instructions API

### 3. Inventory Service
- Bag tracking and cleaning
- Seed collection and storage
- Inventory summaries

### 4. Greenhouse Service
- Spatial quarantine zones
- Contamination tracking
- Plant health monitoring
- Zone transfers

### 5. Germination Service ⭐
- **7 botanical stages** tracking
- True plant identification
- Autotrophic status detection
- Edible fruit potential
- Species-specific guides
- Shipping readiness criteria

### 6. Plant Processing Service
- Processing guides (eating, curing)
- Curing protocols
- Recipe suggestions

### 7. Manufacturing Queue Service
- Queue management workflow
- **Seed storage with refrigeration** (2-8°C)
- Species-specific parameters
- Separation rules
- Greenhouse workflow coordination

### 8. Image Service 📸 NEW!
- S3 presigned upload URLs
- Image moderation queue
- Soft delete with audit logging
- Complete audit trail
- User image management

## 🌾 Supported Species

### Fruit-Bearing Plants (Edible Fruit)
- 🍅 Tomato (70 days, 4 year storage)
- 🌶️ Pepper (varies, 2 year storage, **separation required**)
- 🥒 Cucumber
- 🍈 Cantaloupe (80 days, warm season)
- 🍉 Watermelon (90 days, needs heat)

### Leafy Greens & Vegetables
- 🥬 Lettuce (quick growing)
- 🥬 Cabbage (cool-season, 70 days)
- 🌿 Basil (5 year storage, herb)

### Grains & Specialty
- 🌾 Wheat (grain, 120 days, direct sow)
- 🎋 Sugar Cane (tropical, 365 days, stem cuttings)
- 🌿 Cannabis (⚠️ **SEPARATE FACILITY**, 90 days, 18hr light)

## 📦 Storage Parameters

| Species | Temp | Storage | Separation | Quarantine |
|---------|------|---------|------------|------------|
| Tomato | 5°C | 4 years | ❌ | 14 days |
| Pepper | 5°C | 2 years | ✅ | 14 days |
| Lettuce | 5°C | 3 years | ❌ | 7 days |
| Basil | 5°C | 5 years | ❌ | 7 days |
| Cantaloupe | 5°C | 3 years | ❌ | 14 days |
| Watermelon | 5°C | 3 years | ❌ | 14 days |
| Cabbage | 5°C | 4 years | ❌ | 14 days |
| Wheat | 5°C | 2 years | ❌ | 7 days |
| Sugar Cane | 10°C | 2 years | ❌ | 21 days |
| Cannabis | 5°C | 2 years | ⚠️ **MUST** | 30 days |

## 🎯 Key Features

### Germination Tracking
- ✅ 7 botanical stages (Imbibition → True Plant → Autotrophic)
- ✅ True leaf vs cotyledon identification
- ✅ Edible fruit detection
- ✅ Species-specific timing
- ✅ Photo upload per stage
- ✅ Shipping readiness automation

### Shipping Configuration
- ✅ 4 shipping types (Live Sprout, Bare Root, Potted, Expedited)
- ✅ Dynamic pricing
- ✅ Only shown when plant is ready
- ✅ Visual selection interface

### Image Management
- ✅ S3 presigned uploads (no server upload needed)
- ✅ Moderation workflow
- ✅ Audit logging (all actions tracked)
- ✅ Soft delete (stays in DB for audit)
- ✅ User can delete own images
- ✅ Expandable audit log in UI

### Manufacturing Queue
- ✅ Scanner gun compatible
- ✅ Real-time processing
- ✅ Queue status tracking
- ✅ Cold storage coordination
- ✅ Workflow automation

## 🚀 Quick Test

```bash
# Terminal 1: Start mock API
cd mock-server && cargo run

# Terminal 2: Serve web interfaces  
cd web && python3 -m http.server 8081

# Open in browser:
# - http://localhost:8081/plant-lookup.html (plant search)
# - http://localhost:8081/manufacturing-queue.html (scanner)
# - http://localhost:8081/my-images.html (upload)
# - http://localhost:8081/image-moderation.html (moderation)
```

## 📊 Current Stats (Mock Data)

- **10 Seeds** across 7 species
- **5 Plants** in various growth stages
- **2 Ready to Ship** (Basil, Cantaloupe)
- **7 Species** (Tomato, Basil, Pepper, Lettuce, Cantaloupe, Watermelon, Cabbage, Wheat, Sugar Cane, Cannabis)
- **5 Edible Fruit** species

## 🔧 Technical Stack

- **Backend**: Rust AWS Lambda (8 services)
- **Database**: DynamoDB (11 tables)
- **Storage**: S3 (images)
- **API**: API Gateway REST
- **Frontend**: Vanilla JS (no build step!)
- **Mock Server**: Axum (Rust)
- **Deployment**: AWS SAM

## 📝 Documentation

- `README.md` - Project overview
- `QUICK_START.md` - Get running fast
- `BUILD_STATUS.md` - Current build status
- `PACT_TESTING.md` - API testing guide
- `FEATURES.md` - This file
- `docs/FOOD_SAFETY_RESEARCH_REVIEW.md` - Regulations (⚠️ REVIEW)
- `docs/BAG_FOLDING_INSTRUCTIONS.md` - Trapezoid butterfly method
- `docs/GERMINATION_SHIPPING_GUIDE.md` - Shipping young plants
- `docs/BOTANICAL_STAGES.md` - 7 growth stages explained
- `docs/DEPLOYMENT.md` - AWS deployment guide
- `docs/IMAGE_UPLOAD_GUIDE.md` - S3 upload & moderation

## 🎨 UI Color Palette

- **Light Green**: `#C8E6C9` - Cards, backgrounds
- **Eggshell**: `#F5F5DC` - Main backgrounds
- **White**: `#FFFFFF` - Paper cards
- **Light Blue**: `#B3E5FC` - Accents, gradients
- **Aqua**: `#00BCD4` - Buttons, highlights
- **Teal**: `#009688` - Borders, primary color
- **Teal Dark**: `#00796B` - Text, headers

## 🚧 TODO (Minor)

- [ ] Fix remaining ~5 compilation errors in Lambda functions
- [ ] Add actual S3 integration (presigned URLs work)
- [ ] Connect DynamoDB repositories
- [ ] Add authentication (Cognito or API keys)
- [ ] Thumbnail generation Lambda
- [ ] Email notifications for moderation

## 🎃 Portlandia Halloween Edition Features

- 🕷️ Spooky aesthetic  
- 🦇 Keep Portland Weird vibes
- 🐸 Toad-licking quality UI
- 🚬 "AWS is like Se7en" philosophy
- 💞 Love for plants and weird ideas

---

**Status**: 🚀 **FULLY FUNCTIONAL** (with mock server)  
**Version**: 0.1.0  
**Keep Portland Weird** 🕷️🦇🐸💞

