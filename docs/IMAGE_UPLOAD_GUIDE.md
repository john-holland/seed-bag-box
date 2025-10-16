# 📸 Image Upload & Moderation System

## Overview

The image upload system allows users to upload photos of their plants at various growth stages, with a complete moderation workflow and audit trail.

## Architecture

```
User Upload → S3 Presigned URL → S3 Bucket → DynamoDB Record → Moderation Queue
                                                                        ↓
                                                    Moderator Reviews ← Pending Images
                                                                        ↓
                                                            Approve / Reject / Flag
                                                                        ↓
                                                              Audit Log Created
```

## Upload Workflow

### Step 1: Request Presigned URL
```javascript
POST /images/request-upload
{
  "item_id": "uuid",
  "item_type": "plant",  // or "seed", "greenhouse"
  "filename": "my-tomato.jpg",
  "content_type": "image/jpeg"
}

Response:
{
  "upload_id": "uuid",
  "presigned_url": "https://s3.amazonaws.com/...",
  "s3_key": "images/plant/uuid/uuid",
  "expires_in_seconds": 3600
}
```

### Step 2: Upload to S3
```javascript
// Use presigned URL
PUT presigned_url
Content-Type: image/jpeg
Body: [binary image data]
```

### Step 3: Confirm Upload
```javascript
POST /images/confirm-upload
{
  "upload_id": "uuid",
  "metadata": {
    "item_id": "uuid",
    "item_type": "plant",
    "filename": "my-tomato.jpg",
    "content_type": "image/jpeg",
    "caption": "First true leaves!",
    "growth_stage": "TRUE_LEAF_EMERGENCE",
    "tags": ["tomato", "true-leaf", "success"]
  }
}

Response:
{
  "id": "uuid",
  "s3_url": "https://...",
  "moderation_status": "PENDING",
  "uploaded_at": "ISO8601"
}
```

## User Image Management

### List My Images
```javascript
GET /images/my-images

Response:
{
  "images": [
    {
      "id": "uuid",
      "filename": "my-plant.jpg",
      "s3_url": "https://...",
      "moderation_status": "APPROVED",
      "uploaded_at": "ISO8601"
    }
  ],
  "count": 1
}
```

### Delete My Image (Soft Delete)
```javascript
DELETE /images/{id}

Response:
{
  "status": "deleted",
  "audit_logged": true
}
```

**Important**: 
- This is a **soft delete** - image stays in database
- Marked with `deleted_at` timestamp
- **Audit log entry created** showing who deleted and when
- Moderators can still see deleted images in audit

## Moderation Workflow

### List Pending Images
```javascript
GET /images/pending-moderation

Response:
{
  "pending_images": [...],
  "count": 5
}
```

### Moderate Image
```javascript
POST /images/moderate
{
  "image_id": "uuid",
  "status": "APPROVED",  // or "REJECTED", "FLAGGED"
  "notes": "Looks good!"
}

Response:
{
  "image_id": "uuid",
  "status": "APPROVED",
  "moderated_by": "uuid",
  "audit_logged": true
}
```

### Moderation Statuses
- **PENDING** - Awaiting review
- **APPROVED** - Safe to display
- **REJECTED** - Not suitable
- **FLAGGED** - Needs admin review
- **DELETED** - User deleted

## Audit Logging

Every action creates an audit log entry:

### Actions Logged
- `UPLOADED` - User uploads image
- `APPROVED` - Moderator approves
- `REJECTED` - Moderator rejects
- `FLAGGED` - Moderator flags for review
- `DELETED` - User deletes own image
- `RESTORED` - Admin restores deleted image
- `VIEWED_BY_MODERATOR` - Moderator views image

### Get Audit Log for Image
```javascript
GET /images/{id}/audit

Response:
{
  "image_id": "uuid",
  "audit_logs": [
    {
      "id": "uuid",
      "action": "UPLOADED",
      "performed_by": "user-uuid",
      "performed_at": "ISO8601",
      "details": "Uploaded tomato-true-leaf.jpg",
      "ip_address": "192.168.1.1"
    },
    {
      "id": "uuid",
      "action": "APPROVED",
      "performed_by": "moderator-uuid",
      "performed_at": "ISO8601",
      "details": "Looks healthy!"
    }
  ],
  "count": 2
}
```

### Get All Audit Logs
```javascript
GET /images/audit-log?limit=50&offset=0

Response:
{
  "audit_logs": [...],
  "count": 50,
  "total": 500
}
```

## S3 Configuration

### Bucket Structure
```
seed-box-images/
├── images/
│   ├── plant/
│   │   └── {plant-id}/
│   │       └── {upload-id}.jpg
│   ├── seed/
│   │   └── {seed-id}/
│   │       └── {upload-id}.jpg
│   └── greenhouse/
│       └── {zone-id}/
│           └── {upload-id}.jpg
└── thumbnails/  (auto-generated)
    └── [same structure]
```

### S3 Bucket Policy
```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {"Service": "lambda.amazonaws.com"},
      "Action": ["s3:PutObject", "s3:GetObject", "s3:DeleteObject"],
      "Resource": "arn:aws:s3:::seed-box-images/*"
    }
  ]
}
```

### S3 CORS Configuration
```json
[
  {
    "AllowedOrigins": ["*"],
    "AllowedMethods": ["PUT", "GET"],
    "AllowedHeaders": ["*"],
    "ExposeHeaders": ["ETag"],
    "MaxAgeSeconds": 3000
  }
]
```

## Security

### Upload Validation
- **Max file size**: 5MB
- **Allowed types**: `image/jpeg`, `image/png`, `image/webp`
- **Filename sanitization**: Remove special characters
- **Virus scanning**: Optional with AWS Lambda
- **Content moderation**: AWS Rekognition (optional)

### Access Control
- Users can only:
  - Upload images for their own items
  - View their own images
  - Delete their own images
- Moderators can:
  - View all pending images
  - Approve/reject/flag images
  - View audit logs
- Admins can:
  - View all audit logs
  - Restore deleted images
  - Ban users

## DynamoDB Schema

### Images Table
```
PK: image_id (UUID)
SK: uploaded_at (timestamp)

Attributes:
- uploaded_by (UUID)
- item_id (UUID)
- item_type (string)
- s3_bucket, s3_key, s3_url
- filename, content_type, size_bytes
- caption, growth_stage, tags[]
- moderation_status
- moderated_by, moderated_at
- deleted_at, deleted_by

GSI:
- uploaded_by-index (for user's images)
- moderation_status-index (for pending queue)
- item_id-index (for plant/seed images)
```

### Audit Logs Table
```
PK: audit_id (UUID)
SK: performed_at (timestamp)

Attributes:
- image_id (UUID)
- action (string)
- performed_by (UUID)
- details, ip_address, user_agent

GSI:
- image_id-index (for image history)
- performed_by-index (for user activity)
```

## Web Interfaces

### 1. My Images (`my-images.html`)
**User-facing**:
- Upload new images
- View all uploaded images
- Delete own images
- See moderation status
- View audit trail

**Features**:
- Drag & drop upload
- Image preview before upload
- Caption and tags
- Growth stage selection
- Delete with confirmation

### 2. Moderation Dashboard (`image-moderation.html`)
**Moderator-facing**:
- View pending images
- Approve/reject/flag
- Add moderation notes
- View statistics
- Search and filter

**Features**:
- Tabbed interface (Pending/Approved/Rejected)
- Bulk actions
- Audit log viewer
- Quick keyboard shortcuts

## Image Processing (Future)

### Thumbnail Generation
- Lambda triggered on S3 upload
- Generate 100x100, 300x300, 800x800
- Store in `/thumbnails/` folder

### Metadata Extraction
- Read EXIF data
- Extract dimensions
- Check orientation
- Strip GPS data (privacy)

### Content Analysis (Optional)
- AWS Rekognition for auto-moderation
- Detect inappropriate content
- Identify plant species
- Extract labels/tags

## API Examples

### Upload Flow (JavaScript)
```javascript
async function uploadImage(file, itemId, itemType) {
  // 1. Get presigned URL
  const urlRes = await fetch('/images/request-upload', {
    method: 'POST',
    body: JSON.stringify({
      item_id: itemId,
      item_type: itemType,
      filename: file.name,
      content_type: file.type
    })
  });
  const { upload_id, presigned_url } = await urlRes.json();
  
  // 2. Upload to S3
  await fetch(presigned_url, {
    method: 'PUT',
    body: file,
    headers: { 'Content-Type': file.type }
  });
  
  // 3. Confirm upload
  const confirmRes = await fetch('/images/confirm-upload', {
    method: 'POST',
    body: JSON.stringify({
      upload_id,
      metadata: {
        item_id: itemId,
        item_type: itemType,
        filename: file.name,
        content_type: file.type,
        caption: 'My plant!',
        growth_stage: 'TRUE_LEAF_EMERGENCE',
        tags: ['tomato', 'success']
      }
    })
  });
  
  return await confirmRes.json();
}
```

### Delete with Audit
```javascript
async function deleteMyImage(imageId) {
  const res = await fetch(`/images/${imageId}`, {
    method: 'DELETE'
  });
  
  // Check audit log
  const audit = await fetch(`/images/${imageId}/audit`);
  const logs = await audit.json();
  
  console.log('Audit trail:', logs);
}
```

## Compliance & Privacy

### GDPR Compliance
- Users can delete own images
- Complete audit trail
- Data export available
- Right to be forgotten

### Content Guidelines
- No inappropriate content
- Plant-related only
- No personal information visible
- No location data in EXIF

### Retention Policy
- Approved images: Kept indefinitely
- Rejected images: Kept 30 days
- Deleted images: Soft delete, purge after 90 days
- Audit logs: Kept 7 years

## Testing

### Upload Test
```bash
# Start mock server
cd mock-server && cargo run

# Open user interface
open web/my-images.html

# Click to upload, select image
# Check console for API calls
```

### Moderation Test
```bash
# Open moderation dashboard
open web/image-moderation.html

# Review pending images
# Click approve/reject
# View audit log
```

---

**Document Version**: 1.0  
**Last Updated**: October 16, 2025  
**Keep Portland Weird** 🕷️📸

