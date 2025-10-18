# Web Interface Feature Update

## New Features Implemented

### 1. **Index Page** (`index.html`)
- Created a centralized landing page with links to all HTML documents
- Modern card-based design with gradient backgrounds
- Easy navigation to all web interfaces

### 2. **User Presence in Search** (`plant-lookup.html`)
Added real-time user presence indicators:
- **Active Users Section**: Shows who's currently using the plant lookup system
- Live presence indicators with animated dots
- User avatars with emojis and current actions
- Updates every 10 seconds to simulate real-time activity
- Displays 1-3 random active users for demonstration

### 3. **Cannabis Opt-Out Filtering** (`plant-lookup.html` + `user-settings.html`)
Implemented privacy-respecting cannabis filtering:

#### User Settings Page:
- Cannabis opt-in toggle with legal compliance checklist
- State-based validation (only allows opt-in for legal states)
- Age verification and legal acknowledgment requirements
- Settings saved to localStorage for persistence
- Auto-loads saved settings on page visit

#### Plant Lookup Page:
- Automatically filters out cannabis species if user has opted out
- Visual banner indicator showing filter status:
  - 🚫 Red/yellow banner when cannabis is filtered
  - 🌿 Green/blue banner when cannabis program is enabled
- Link to settings page for easy preference changes
- Console logging for debugging filter status

### How It Works

1. **First Visit**:
   - User visits `user-settings.html`
   - Selects their state
   - Opts in or out of cannabis program
   - Saves settings (stored in browser localStorage)

2. **Using Plant Lookup**:
   - User visits `plant-lookup.html`
   - Page loads user settings from localStorage
   - Cannabis items automatically filtered based on preferences
   - Banner displays current filter status
   - Can click banner link to change settings

3. **User Presence**:
   - Active users displayed at top of page
   - Shows user emoji, name, and current activity
   - Refreshes every 10 seconds
   - Uses mock data (can be connected to WebSocket for real-time)

## Technical Implementation

### localStorage Schema
```javascript
{
  cannabis_opt_in: boolean,
  cannabis_acknowledged: boolean,
  shipping_address: {
    state: string,
    city: string,
    zip: string
  },
  delivery_frequency: string,
  user_name: string,
  user_emoji: string,
  preferences: {
    edible_fruit: boolean,
    recall_notifications: boolean
  }
}
```

### Filter Logic
```javascript
// In plant-lookup.html performSearch()
if (!userSettings.cannabis_opt_in) {
  filtered = filtered.filter(item => 
    item.species.toLowerCase() !== 'cannabis'
  );
}
```

## Testing

### Test Cannabis Filtering:
1. Open `user-settings.html`
2. Toggle cannabis to OFF (default)
3. Save settings
4. Open `plant-lookup.html`
5. Search for "cannabis" - should show 0 results
6. Banner should show "🚫 Cannabis filtered (opt-out)"

### Test Cannabis Opt-In:
1. Open `user-settings.html`
2. Select a legal state (e.g., Oregon, California)
3. Toggle cannabis to ON
4. Check the acknowledgment box
5. Save settings
6. Open `plant-lookup.html`
7. Search for "cannabis" - should show 1 result (SEED-006)
8. Banner should show "🌿 Cannabis program enabled"

### Test User Presence:
1. Open `plant-lookup.html`
2. Observe "👥 Active Users" section at top
3. Wait 10 seconds - users should randomly change
4. Each user shows emoji, name, and activity

## Files Modified

- ✅ `web/index.html` - **CREATED** - Landing page with links
- ✅ `web/plant-lookup.html` - Added presence & cannabis filtering
- ✅ `web/user-settings.html` - Added localStorage persistence

## Future Enhancements

- [ ] Connect presence to WebSocket for real-time updates
- [ ] Add user profile pictures instead of emojis
- [ ] Show typing indicators when users are searching
- [ ] Add backend API integration for settings
- [ ] Add more granular filtering options
- [ ] Show which users are viewing the same plant

## Legal Compliance

The cannabis filtering system includes:
- State-based validation (18 legal states)
- Age verification requirement (21+)
- Legal acknowledgment checkbox
- Automatic filtering by default (opt-in only)
- Clear warning messages
- Links to compliance information

**States where cannabis is legal (as of 2025):**
CA, OR, WA, CO, NY, MA, NV, IL, MI, AZ, NJ, MT, VT, NM, CT, RI, ME, AK

---

**Keep Portland Weird** 🌱

