# Wallet Analyzer & AML Checker

**The Wallet Analyzer & AML Checker** is an open-source tool to enhance security in wallet interactions within Web3.  
Integrated into the **Cometix Security Web Extension**, it leverages artificial intelligence (AI)  
to assess wallet risks and ensure anti-money laundering (AML) compliance on Ethereum and Solana blockchains.  
The tool identifies suspicious activities, traces fund origins, and detects vulnerabilities,  
providing detailed warnings to help users and developers maintain secure decentralized interactions.

## Key Features

- **AI-Driven Risk Assessment** *(Open Source)*  
  Analyzes wallet risks in real time using AI, detecting threats like scam affiliations or exploit connections,  
  with instant notifications.

- **AML Compliance Checking** *(Open Source)*  
  Screens wallets for money laundering signs, such as rapid fund cycling or links to sanctioned entities,  
  delivering precise alerts for compliance.

- **Fund Origin Tracing** *(Open Source)*  
  Tracks fund histories to identify tainted assets, like those from hacks, generating detailed reports  
  for secure wallet interactions.

- **Suspicious Activity Detection** *(Open Source)*  
  Uses proprietary AI to identify malicious behavior patterns, like frequent scam interactions,  
  with proactive warnings to mitigate risks.

- **Real-Time Wallet Analysis**  
  Examines wallet activity and connections with AI, flagging anomalies like sudden large transfers  
  for immediate feedback.

- **Detailed Risk Reports**  
  Compiles AI-driven analysis into a dashboard, presenting risk details and mitigation steps  
  to support secure Web3 operations.

---

## Architecture

```
                  ┌────────────────────────────┐
                  │    Cometix Web Extension   │
                  └────────────┬───────────────┘
                               │
        ┌──────────────────────▼──────────────────────┐
        │ Wallet Analyzer & AML Checker (Rust + WASM) │
        │  - Risk Analysis Engine                     │
        │  - AML Compliance Module                    │
        │  - Fund Tracing System                      │
        │  - Suspicious Pattern Detector              │
        │  - AI Engine                                │
        └──────────────────────┬──────────────────────┘
                               │
              ┌────────────────▼─────────────────┐
              │      Ethereum / Solana RPC       │
              └──────────────────────────────────┘
```

---

### Requirements

| Tool                  | Version      |
|-----------------------|--------------|
| cometix-sdk           | 1.0.16       |
| solana-instruction    | >=2.2.1      |
| eth-instruction       | >=3.143.1    |
| cometix-security      | 1.0.52       |
| cometix-utils         | 1.1.12       |
| wasm-pack             | >=0.10.2     |
| Rust                  | >=1.70.0     |
| Node.js               | >=18.0.0     |
| Vite                  | >=4.0.0      |

---

## Local Development

Install Rust toolchain:

```bash
curl https://sh.rustup.rs -sSf | sh
cargo install wasm-pack
```

Install frontend deps:

```bash
cd frontend/
npm install
```

## Building the WASM Module

```bash
cd wallet-analyzer/
wasm-pack build --target web --out-dir ../output/out
```

Flags:
- `--target web`: for browser-based WASM  
- `--out-dir`: exports to frontend for integration
