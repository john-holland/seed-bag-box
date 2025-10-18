# ⚠️ Food Recall Monitoring & Customer Notification System

## Overview

> 🚬🐸 *"Look, I love the turtles in shipping but they gotta keep their mitts off the food - they gots salmonella, I tells ya"*

This system monitors USDA and FDA recall databases, matches recalls to our customers by shipping address and product species, and automates customer notifications.

## Why This Matters

Food safety recalls can affect our operations in multiple ways:
1. **Seeds we're storing** might be from recalled batches
2. **Plants we're growing** might be affected species
3. **Customers in affected states** need to be notified
4. **Legal liability** if we ship contaminated products

## Data Sources

### 1. USDA FSIS (Food Safety and Inspection Service)
- **URL**: https://www.fsis.usda.gov/recalls
- **API**: https://www.fsis.usda.gov/fsis-content/api/recalls
- **Coverage**: Meat, poultry, some produce
- **Format**: JSON
- **Frequency**: Check daily

### 2. FDA Food Recalls
- **URL**: https://www.fda.gov/safety/recalls-market-withdrawals-safety-alerts
- **API**: https://api.fda.gov/food/enforcement.json
- **Coverage**: Most produce, seeds, food products
- **Format**: JSON (openFDA format)
- **Frequency**: Check daily

### 3. CDC Outbreaks (Manual Check)
- **URL**: https://www.cdc.gov/foodsafety/outbreaks/
- **Format**: Website only (no API)
- **Coverage**: Active outbreaks
- **Frequency**: Check weekly

## Workflow

### Step 1: Fetch Recalls (Automated)
```
Daily Lambda Trigger → Fetch USDA API → Fetch FDA API → Save to DynamoDB
                                                              ↓
                                                    Mark as "NEW" for review
```

### Step 2: Moderation Review (Manual)
Moderator reviews each new recall:
- ✅ **Is it relevant to our products?**
- ✅ **Which species are affected?**
- ✅ **Which states are affected?**
- ✅ **What contamination type?**
- ✅ **Manual verification completed?**

### Step 3: Impact Assessment (Automated)
If recall is marked relevant:
```
Query Customers by:
  ├─ Shipping address in affected states
  ├─ Products matching affected species
  └─ Shipment dates in recall window
                    ↓
            Generate Impact Report:
              • # of affected customers
              • # of affected products
              • Risk level
              • Recommended actions
```

### Step 4: Customer Notification (Semi-Automated)
```
Moderator Reviews Impact → Approves Notification → System Sends Emails
                                                              ↓
                                            Customer receives:
                                              • Recall details
                                              • Products affected
                                              • Actions to take
                                              • Contact information
```

## API Endpoints

### Fetch Recalls
```bash
POST /recalls/fetch-usda
Response: {
  "source": "USDA",
  "recalls_found": 5,
  "recalls": [...],
  "fetched_at": "ISO8601"
}

POST /recalls/fetch-fda
Response: {
  "source": "FDA",
  "recalls_found": 12,
  "recalls": [...],
  "fetched_at": "ISO8601"
}
```

### Review Recalls
```bash
GET /recalls/new
Response: {
  "new_recalls": [...],
  "count": 3,
  "reminder": "⚠️ Manual USDA/FDA website check recommended"
}

POST /recalls/review
{
  "recall_id": "uuid",
  "is_relevant": true,
  "affected_species": ["tomato", "lettuce"],
  "notes": "Affects our CA customers with tomato seeds",
  "manual_check_required": true
}
```

### Impact Assessment
```bash
POST /recalls/assess-impact
{
  "recall_id": "uuid"
}

Response: {
  "potentially_affected_customers": ["uuid", ...],
  "affected_by_state": [
    {"state": "CA", "count": 12},
    {"state": "OR", "count": 8}
  ],
  "total_customers_affected": 20,
  "risk_level": "HIGH",
  "requires_customer_notification": true
}
```

### Customer Notification
```bash
POST /recalls/{id}/notify
Response: {
  "recall_id": "uuid",
  "customers_notified": 20,
  "notification_sent_at": "ISO8601"
}

GET /recalls/affected-customers
Response: {
  "affected_customers": [...],
  "count": 20
}
```

## Matching Logic

### By Shipping Address
```rust
if recall.affected_states.contains(&customer.shipping_address.state) {
    // Customer might be affected
    potentially_affected.push(customer.id);
}
```

### By Product Species
```rust
// Check seeds
for seed in customer_seeds {
    if recall.affected_species.contains(&seed.species) {
        affected_products.push(seed.id);
    }
}

// Check plants
for plant in customer_plants {
    if recall.affected_species.contains(&plant.species) {
        affected_products.push(plant.id);
    }
}
```

### By Date Range
```rust
if shipment.shipped_at >= recall.distribution_start_date 
   && shipment.shipped_at <= recall.distribution_end_date {
    potentially_affected_shipments.push(shipment.id);
}
```

## Hazard Classifications

### Class I (Serious)
- **Risk**: Death or serious adverse health consequences
- **Examples**: Salmonella, E. coli O157:H7, Listeria
- **Action**: IMMEDIATE customer notification required
- **Quarantine**: All matching products immediately

### Class II (Moderate)
- **Risk**: Temporary or reversible adverse health consequences
- **Examples**: Minor contamination, mislabeling
- **Action**: Customer notification within 24 hours
- **Quarantine**: Review matching products

### Class III (Low)
- **Risk**: Not likely to cause adverse health consequences
- **Examples**: Quality issues, minor violations
- **Action**: Monitor, may not require customer notification

## Customer Notification Email Template

```
Subject: IMPORTANT: Food Safety Recall Notice

Dear [Customer Name],

We are contacting you regarding a recent food safety recall that may affect 
products you have received from Seed Box Bag Box.

RECALL DETAILS:
Product: [Product Name]
Company: [Company Name]
Reason: [Contamination Type]
Recall Date: [Date]
Recall Number: [Number]

YOUR POTENTIAL EXPOSURE:
You are receiving this notice because:
- You have a shipping address in [State]
- You have received [Species] products
- Your shipment dates overlap with the affected distribution period

RECOMMENDED ACTIONS:
1. Check your [species] products for lot codes: [codes]
2. Do NOT consume affected products
3. Dispose of affected products safely
4. Contact us if you have questions: support@seedboxbagbox.com
5. Seek medical attention if you experience symptoms

SYMPTOMS TO WATCH FOR:
[Based on contamination type - Salmonella, Listeria, etc.]

This is a precautionary notice. You may not be affected, but we wanted to 
inform you out of an abundance of caution.

For more information:
[Link to official recall]

Seed Box Bag Box Safety Team
```

## Moderation Dashboard Features

### Main View
- ⚠️ **Warning banner** - Manual check reminder
- 🔔 **Fetch buttons** - USDA, FDA, Manual Check
- 📋 **New recalls list** - Pending review
- 🎯 **Customer matches** - Auto-calculated by state
- ⚠️ **Hazard classification** - Visual badges

### Review Actions
- **Mark Relevant** → Triggers impact assessment
- **Mark Not Relevant** → Archives recall
- **Assess Impact** → Shows affected customer count
- **Notify Customers** → Sends email alerts

### Manual Verification
- Opens USDA.gov, FDA.gov, CDC.gov in tabs
- Checklist for manual review
- Notes field for findings
- Completed checkbox

## Automated Monitoring (Production)

### Daily Lambda Trigger
```yaml
RecallFetchFunction:
  Type: AWS::Serverless::Function
  Events:
    DailyCheck:
      Type: Schedule
      Properties:
        Schedule: cron(0 9 * * ? *)  # 9 AM daily
```

### Process
1. Lambda runs at 9 AM daily
2. Fetches last 7 days of recalls from USDA + FDA
3. Compares to existing recalls in DynamoDB
4. Marks new recalls as "NEW"
5. Sends Slack/Email notification to moderators
6. Moderators review during business hours

## Safety Protocols

### When a Relevant Recall is Identified

1. **Immediate Actions** (within 1 hour):
   - Quarantine all matching seeds/plants
   - Halt shipments of affected species
   - Assess customer impact
   - Document in system

2. **Customer Notification** (within 24 hours):
   - Email all potentially affected customers
   - Provide recall details
   - Give clear instructions
   - Offer support contact

3. **Product Management**:
   - Mark affected seeds as "QUARANTINE"
   - Test unaffected batches if possible
   - Dispose of contaminated products
   - Document disposal

4. **Follow-up**:
   - Monitor customer responses
   - Offer refunds/replacements
   - Update safety procedures
   - Report to management

## Integration with Other Systems

### Greenhouse System
```rust
if recall.is_relevant {
    // Quarantine affected plants
    for plant in affected_plants {
        move_to_quarantine_zone(plant.id).await;
    }
}
```

### Inventory System
```rust
// Mark affected seeds
for seed in affected_seeds {
    seed.status = SeedStatus::Quarantine;
    seed.contamination_check = Some(recall_details);
}
```

### Subscription System
```rust
// Notify affected subscribers
for customer in affected_customers {
    send_recall_notification(customer.id, recall.id).await;
    subscription.add_note("Recall notification sent");
}
```

## Testing

### Mock Recalls
The system includes mock recalls for testing:
- Organic Spinach (USDA) - Salmonella - CA, OR, WA
- Fresh Tomatoes (FDA) - Listeria - Nationwide

### Test Workflow
```bash
# Start mock server
cd mock-server && cargo run

# Open recall dashboard
open web/recall-moderation.html

# Click "Fetch USDA" or "Fetch FDA"
# Review mock recalls
# Mark as relevant/not relevant
# Assess customer impact
```

## Compliance

### Legal Requirements
- Maintain records of all recalls reviewed
- Document customer notifications
- Keep audit trail of actions taken
- Report to regulatory agencies if required

### Record Retention
- Recall data: 7 years
- Customer notifications: 7 years
- Impact assessments: 7 years
- Review notes: 7 years

## Important Notes

### ⚠️ Turtles and Salmonella
As the wise frog 🚬🐸 points out:
- Reptiles (turtles, lizards) carry Salmonella naturally
- Keep reptiles OUT of food preparation areas
- Wash hands after handling
- Never allow reptiles near seed/plant processing
- Separate wildlife from operations

### Manual Verification Required
⚠️ **CRITICAL**: Always manually verify recalls on official websites.
- APIs may have delays
- Website has more detail
- Some recalls not in API
- Visual verification recommended

### Customer Privacy
- Only share necessary information
- Comply with data privacy laws
- Secure customer contact information
- Allow customers to opt out of future alerts

---

**Document Version**: 1.0  
**Last Updated**: October 16, 2025  
**Reviewed By**: 🚬🐸 The Wise Frog

**Keep Portland Weird - Keep Food Safe** 🕷️🦇

