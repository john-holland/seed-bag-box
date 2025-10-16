# 🔫 Scanner Gun Interface

## Manufacturing Queue Web Interface

### Features 🦇

1. **Auto-Focus Input**
   - Page auto-focuses text field on load
   - Re-focuses automatically if user clicks away
   - Keeps scanner always ready

2. **Scanner Gun Compatible**
   - Handles paste events (scanner sends data as paste)
   - Handles rapid keypress events
   - Auto-submits after 100ms of no input
   - Recognizes Enter key (most scanners append Enter)

3. **Visual Feedback**
   - Green flash on successful scan
   - Beep sound (if browser allows)
   - Success/error messages
   - Recent scans list

4. **Barcode Type Detection**
   - `SEED-*` → Seed barcode
   - `BAG-*` → Bag barcode
   - `ZONE-*` → Zone barcode
   - `PLANT-*` → Plant barcode
   - 12-13 digits → UPC
   - UUID format → System UUID
   - Anything else → Unknown

## How to Use

### Option 1: Open Directly
```bash
open web/manufacturing-queue.html
```

### Option 2: Serve with Python
```bash
cd web
python3 -m http.server 8080
# Open http://localhost:8080/manufacturing-queue.html
```

### Option 3: Serve with Node
```bash
cd web
npx serve
```

### Option 4: Deploy to S3
```bash
aws s3 cp manufacturing-queue.html s3://your-bucket/queue.html --acl public-read
```

## Scanner Gun Setup

### Most Common Scanner Guns:
- **Symbol/Zebra DS series** - Set to "USB Keyboard" mode
- **Honeywell Voyager series** - Default "Keyboard Wedge" mode
- **Datalogic QuickScan series** - Set to "USB-COM" or "Keyboard" mode

### Configuration:
1. Set scanner to **Keyboard Wedge** mode (emulates typing)
2. Set suffix to **Enter** (CR or LF)
3. Test with any text field - should "type" instantly

## Testing Without Scanner

Use keyboard to simulate:
1. Type: `SEED-12345`
2. Press Enter
3. Should process immediately

Or paste:
1. Copy: `BAG-67890`
2. Paste into field (Ctrl+V / Cmd+V)
3. Should process immediately

## API Integration

Update the `sendToAPI()` function:

```javascript
async function sendToAPI(code, type) {
    const apiUrl = 'https://your-api-gateway-url.amazonaws.com/prod';
    
    const response = await fetch(`${apiUrl}/queue`, {
        method: 'POST',
        headers: { 
            'Content-Type': 'application/json',
            'x-api-key': 'your-api-key'
        },
        body: JSON.stringify({ 
            code, 
            type, 
            timestamp: new Date().toISOString() 
        })
    });
    
    return response.json();
}
```

## Barcode Label Generation

### Seed Labels
```
SEED-[UUID]
e.g., SEED-550e8400-e29b-41d4-a716-446655440000
```

### Bag Labels
```
BAG-[UUID]
e.g., BAG-123e4567-e89b-12d3-a456-426614174000
```

### Zone Labels (Greenhouse)
```
ZONE-[UUID]
e.g., ZONE-a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Plant Labels
```
PLANT-[UUID]
e.g., PLANT-98765432-1234-5678-9012-345678901234
```

## Warehouse Workflow

### 1. Seed Intake
1. Scan seed packet barcode
2. System creates seed record
3. Generates storage location
4. Prints storage label

### 2. Cold Storage
1. Scan seed barcode
2. Scan storage location barcode
3. System updates location
4. Tracks refrigeration unit

### 3. Germination Start
1. Scan seed barcode
2. Scan germination zone barcode
3. System starts germination record
4. Begins timeline tracking

### 4. Zone Transfer
1. Scan plant barcode
2. Scan destination zone barcode
3. System validates transfer
4. Updates plant location

### 5. Shipment Prep
1. Scan plant barcode(s)
2. Scan customer order barcode
3. System creates shipment
4. Generates shipping label

## Keyboard Shortcuts

- **Ctrl+R** - Refresh (keeps focus)
- **Esc** - Clear current input
- **Ctrl+L** - Clear scan list
- **Tab** - Navigate buttons (but auto-refocuses input)

## Troubleshooting

### Scanner not working?
1. Check USB connection
2. Verify "Keyboard Wedge" mode
3. Test in Notepad/TextEdit - should type instantly
4. Check suffix setting (should be Enter/CR)

### Input losing focus?
- Page has aggressive re-focus script
- Should re-focus every second
- Click anywhere to trigger re-focus

### No beep sound?
- Browser may block autoplay audio
- Click page first to enable sounds
- Or disable beep in code

### Scans too slow?
- Reduce `scanTimeout` from 100ms to 50ms
- Or remove timeout and rely on Enter key

## Portland Halloween Edition Features 🕷️🦇

- Spooky gradient background
- Portlandia-themed icons
- "Keep Portland Weird" aesthetic
- Dark mode friendly
- Scanner gun emojis (🔫)

---

**Keep Portland Weird**  
*"AWS is like the movie Se7en but your career is in the box"*

