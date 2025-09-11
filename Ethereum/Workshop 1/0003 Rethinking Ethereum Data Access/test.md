---
description: 
auto_execution_mode: 1
---

# Full‑Stack Take‑Home — Crypto Heatmap (Next.js + CoinGecko)

**Contact for questions:** felipe@hero.io  
**Submission:** private GitHub repository **or** a **.zip** file.

> **Inspiration:** Please look at **https://hero.io/market** — replicate a similar visual style and UX (large dominant tiles, clean labels, green/red coloring, hover tooltips). The attached reference image also indicates the desired look & feel.

---

## Objective

Build a responsive web app that renders a **crypto treemap (“heatmap”)** of the **top 150 coins**.

- **Tile area = market dominance** (**market cap** share across fetched coins) — use CoinGecko’s `market_cap` if available in the free API (it is).  
- **Tile color = 24h price change %** — use CoinGecko’s `price_change_percentage_24h`.  
- Show on larger tiles (similar to the reference / hero.io/market): **SYMBOL**, **$PRICE**, **±% 24h**, and **Dominance: X%**.  
- Tooltip on hover: **Name, Price, 24h %, Market Cap, Volume (24h)**.
- Must be **fully responsive** (mobile → desktop).

> Note: We use trading **volume (24h)** in the tooltip only; area is based on **market cap dominance**.

---

## Stack

- **Frontend (required):** **Next.js + React** (you may choose any charting or UI libraries: ECharts, Nivo, D3, etc.).  
- **Backend (recommended):** **Python** (FastAPI suggested) — however you may use **any language** you prefer. Keep the API key server‑side.

---

## Data Source — CoinGecko (Free Demo API)

1) **Create a free API key**  
- Sign up for the **CoinGecko API Demo plan** (free).  
- Generate a **Demo API Key**.

2) **Base URL & Auth**  
- Base URL: `https://pro-api.coingecko.com/api/v3/`  
- Auth (choose one):  
  - Header: `x-cg-pro-api-key: YOUR_API_KEY`  
  - Query: `?x_cg_pro_api_key=YOUR_API_KEY`

3) **Endpoint** (markets)  
Fetch markets sorted by volume, then keep the **top 150**:
```
GET /coins/markets
  ?vs_currency=usd
  &order=volume_desc
  &per_page=250
  &page=1
  &price_change_percentage=24h
```
- You may need to request `page=2`. Cap the merged list at **150**.
- Use these fields: `id`, `symbol`, `name`, `image`, `current_price`, `market_cap`,
  `price_change_percentage_24h`, `total_volume`.

**Example (curl):**
```bash
curl -s "https://pro-api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=volume_desc&per_page=250&page=1&price_change_percentage=24h" \
  -H "x-cg-pro-api-key: $COINGECKO_API_KEY"
```

---

## What to Build

### 1) Backend (language of your choice; **Python recommended**)
Expose a **single endpoint** consumed by the frontend:
```
GET /api/heatmap
```
**Response shape:**
```json
{
  "as_of": "2025-09-02T12:34:56Z",
  "coins": [
    {
      "id": "bitcoin",
      "symbol": "BTC",
      "name": "Bitcoin",
      "image": "https://.../bitcoin.png",
      "price_usd": 110325.00,
      "price_change_pct_24h": 0.48,
      "market_cap_usd": 2197140225619,
      "volume_usd_24h": 41065049623
    }
  ]
}
```
**Responsibilities**
- Call CoinGecko **/coins/markets**, aggregate the **top 150** items, and map them to the response above.
- Add **caching** (e.g., 5 minutes) to respect rate limits.
- Gracefully handle upstream errors (return an empty `coins: []` with a helpful message if necessary).
- **Never expose** the API key to the browser (read from env on the server).

**Config via env (document defaults):**
```
COINGECKO_BASE_URL=https://pro-api.coingecko.com/api/v3
COINGECKO_API_KEY=YOUR_KEY
CACHE_TTL_SECONDS=300
MAX_COINS=150
```

### 2) Frontend (Next.js + React, required)
- Render a **treemap heatmap** (or visually equivalent) where:
  - **Area** ∝ **market cap**, and **Dominance: X%** is computed as `market_cap / sum(market_cap) * 100`.
  - **Color** based on `price_change_percentage_24h` (diverging red↔green). Include a small legend.
  - **Labels on larger tiles**: `SYMBOL`, `$PRICE`, `±% 24h`, `Dominance: X%` (match the hierarchy of the reference and the hero.io/market page).
- **Tooltip** on hover: name, price, ±% 24h, market cap, and 24h volume.
- **Responsive**: tiles reflow smoothly across breakpoints (look at the attached images).
- **Controls**:
  - **Refresh** button.
  - (Optional) **Search** (filter by name/symbol).
  - **Icons on tiles**: **your choice** (optional).

**Notes**
- **Dark mode**: **not required**.
- **Browser support**: targeting **latest Chrome** is sufficient.

---

## Project Structure & Local Run

Deliver a project that runs from a fresh clone with minimal steps.

```
/backend
  ├─ src/...
  ├─ tests/...
  ├─ .env.example
  └─ README.md

/frontend
  ├─ app/ (or pages/)
  ├─ components/
  ├─ tests/
  ├─ .env.example
  └─ README.md

/README.md  (root quickstart)
```

**Environment examples**
- `/backend/.env.example`
  ```dotenv
  COINGECKO_BASE_URL=https://pro-api.coingecko.com/api/v3
  COINGECKO_API_KEY=YOUR_KEY
  CACHE_TTL_SECONDS=300
  MAX_COINS=150
  ```
- `/frontend/.env.example`
  ```dotenv
  BACKEND_BASE_URL=http://localhost:8000
  ```

**Commands (example; adapt and document)**
- Backend: `make dev` or `uvicorn app:app --reload` / your language’s run command
- Frontend: `npm install && npm run dev`
- Open: `http://localhost:3000`

**Optional**: Docker Compose to run both services together.

---

## Deliverables

- A **private GitHub repo** (grant reviewer access) **or** a **.zip** with the full project.
- Clear **README(s)** describing env setup, how to run locally, and any assumptions.
- Short notes on design choices (chart lib, color scale mapping, responsiveness techniques).

---

## Visual Requirements (match our style)
- Large, dominant tiles for the biggest market caps (BTC/ETH), smaller tiles for the rest.
- On‑tile text hierarchy exactly as in the reference:
  - **SYMBOL** (very large), then **$PRICE**, then **±% 24h**, then **Dominance: X%**.
- Diverging red/green color mapping for the **24h % change**. Include a small legend.
- Professional number formatting: `1,234`, `12.3M`, `4.5B`, `2.19T`, etc.
- Ensure text contrast is readable across colors (adjust text color automatically if needed).

---

## License / Usage
Use open‑source libraries compatible with a permissive license (MIT/Apache). Do not commit secrets.

---

**Contact:** felipe@hero.io
