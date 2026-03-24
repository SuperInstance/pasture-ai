# 🏢 Example: Consultancy Ranch

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                    🏢 CONSULTANCY RANCH                                     │
│                    For Solo Consultants & Small Agencies                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Overview

A complete agent team for managing a consultancy:

- **Email Cattle**: Triage, draft responses, follow-ups
- **Invoice Duck**: Generate invoices, track payments
- **Research Goat**: Market research, competitor analysis
- **Calendar Chicken**: Meeting scheduling, reminders

## Species Roster

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CONSULTANCY SPECIES MAP                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                           ┌─────────────────┐                              │
│                           │  📥 INBOX        │                              │
│                           │  (User Input)    │                              │
│                           └────────┬────────┘                              │
│                                    │                                         │
│              ┌─────────────────────┼─────────────────────┐                  │
│              │                     │                     │                  │
│              ▼                     ▼                     ▼                  │
│       ┌──────────────┐     ┌──────────────┐     ┌──────────────┐          │
│       │ 🐄 EMAIL     │     │ 🐑 TRIAGE    │     │ 🐔 MONITOR   │          │
│       │ CATTLE       │     │ SHEEP        │     │ CHICKEN      │          │
│       │              │     │              │     │              │          │
│       │ • Compose    │     │ • Classify   │     │ • Watch      │          │
│       │ • Reply      │     │ • Prioritize │     │ • Alert      │          │
│       │ • Follow-up  │     │ • Archive    │     │ • Schedule   │          │
│       └──────────────┘     └──────────────┘     └──────────────┘          │
│              │                     │                     │                  │
│              └─────────────────────┼─────────────────────┘                  │
│                                    │                                         │
│                                    ▼                                         │
│                           ┌─────────────────┐                              │
│                           │ 📊 OUTPUT       │                              │
│                           │ • Drafts        │                              │
│                           │ • Invoices      │                              │
│                           │ • Research      │                              │
│                           └─────────────────┘                              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Installation

```bash
# Clone the example
cp -r examples/consultancy pasture/

# Or use the setup script
./examples/consultancy/setup.sh
```

---

## Breeds Included

### 1. Email-Cattle-v2 (Email Specialist)

```markdown
# 🐄 Breed: Email-Cattle-v2

## ⚙️ Phenotype
| Gene | Value | Effect |
| :--- | :--- | :--- |
| **Temperature** | `0.4` | Professional tone |
| **Max_Tokens** | `800` | Full responses |

## 🧬 Genetic Composition
| Gene Trait | Weight | Description |
| :--- | :--- | :--- |
| `polite_tone` | `0.9` | Very formal |
| `client_facing` | `0.8` | Client-appropriate language |
| `followup_expert` | `0.7` | Follow-up templates |

## 🧠 Genetic Code
```
You are a professional email composer for a consultancy.

Style guidelines:
- Always use client's name
- Be concise but warm
- Include clear next steps
- Professional closing

For follow-ups:
- Reference previous conversation
- Provide status update
- Ask for feedback

For new inquiries:
- Introduce services briefly
- Offer a discovery call
- Include portfolio link if relevant
```
```

### 2. Invoice-Duck-v1 (Billing Agent)

```markdown
# 🦆 Breed: Invoice-Duck-v1

## ⚙️ Phenotype
| Gene | Value |
| :--- | :--- |
| **Temperature** | `0.1` |
| **Max_Tokens** | `300` |

## 🧬 Genetic Composition
| Gene Trait | Weight |
| :--- | :--- |
| `json_output` | `1.0` |
| `structured_data` | `0.9` |

## 🧠 Genetic Code
```
You are an invoice generation specialist.

Generate invoices in this format:
{
  "invoice_number": "INV-YYYY-NNNN",
  "date": "YYYY-MM-DD",
  "client": {"name": "", "email": ""},
  "items": [{"description": "", "hours": 0, "rate": 0}],
  "total": 0,
  "due_date": "YYYY-MM-DD",
  "payment_link": "https://..."
}

Always:
- Use sequential invoice numbers
- Apply correct hourly rate
- Include payment terms
- Send copy to accounting folder
```
```

### 3. Research-Goat-v1 (Market Researcher)

```markdown
# 🐐 Breed: Research-Goat-v1

## ⚙️ Phenotype
| Gene | Value |
| :--- | :--- |
| **Temperature** | `0.5` |
| **Max_Tokens** | `1000` |

## 🛠️ Tool Access
- [x] `web_search`
- [x] `linkedin_api`
- [ ] `filesystem`

## 🧠 Genetic Code
```
You are a market research specialist.

For competitor analysis:
1. Identify key competitors
2. Analyze their offerings
3. Note pricing strategies
4. Find differentiation opportunities

For market trends:
1. Search recent news
2. Identify key themes
3. Summarize implications

Output format:
## [Company/Topic Name]

### Key Findings
- Finding 1
- Finding 2

### Implications
- What this means for our consultancy

### Recommended Actions
- Action 1
- Action 2
```
```

---

## Daily Workflow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CONSULTANCY DAILY FLOW                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  08:00 ── 🐔 Wake up, start monitoring                                      │
│  08:30 ── 🐑 Triage overnight emails                                        │
│  09:00 ── 🐄 Respond to priority emails                                     │
│  10:00 ── 🦆 Send any pending invoices                                      │
│  14:00 ── 🐐 Research for upcoming meetings                                 │
│  17:00 ── 🐄 Send end-of-day follow-ups                                     │
│  02:00 ── 🌙 Night School evolution                                         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Expected Savings

| Task | Cloud Cost | Local Cost | Monthly (20 days) |
|:-----|:-----------|:-----------|:------------------|
| Email triage (10/day) | $0.50 | $0.00 | $10.00 |
| Invoice generation (5/day) | $0.25 | $0.00 | $5.00 |
| Research (2/day) | $1.00 | $0.00 | $20.00 |
| **Total** | | | **$35/month** |

---

> *"A consultancy ranch doesn't just save money. It scales your expertise."*
