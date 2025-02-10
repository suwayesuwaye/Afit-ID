# Decentralized Healthcare Management System

## Overview
This project is a decentralized healthcare management system that ensures patient data privacy using blockchain and zero-knowledge proofs (ZKPs). It enables patients to securely store their medical records off-chain while granting controlled access to healthcare providers. By leveraging Ethereum smart contracts, IPFS for encrypted storage, and zk-SNARKs for privacy, the system ensures that only authorized personnel can verify specific medical details without exposing full patient data.

## Features
- **Decentralized Identifiers (DID):** Secure authentication via MetaMask/WalletConnect.
- **Smart Contracts:** Built on Ethereum L2 (Polygon) for access control and consent management.
- **Encrypted Storage:** Medical records are stored off-chain using IPFS.
- **Zero-Knowledge Proofs (ZK-SNARKs):** Privacy-preserving verification for medical records.
- **Audit Logging:** Immutable record of data access for transparency.
- **User-Friendly Interface:** React-based frontend for patient and provider interactions.

## Tech Stack
- **Blockchain:** ......... (Ethereum L2)
- **Smart Contracts:** Solidity (Access Control, Consent Management, Audit Logging)
- **Storage:** IPFS (Encrypted Medical Records)
- **Zero-Knowledge Proofs:** Circom + SnarkJS
- **Identity & Authentication:** DID (Decentralized Identifiers) + MetaMask/WalletConnect
- **Frontend:** React.js, TailwindCSS
- **Backend:** Node.js (Express)

## System Architecture

![System Architecture](https://mermaid.live/view#pako:eNqNVFtPIjEU_isnNdlgthhEURw2Jtw0RnEJqJu47EPtFOjSaWfbjsIS_vueuSEaTbaZB6Z8t9NzpmvCTShIQGaWxXO4GU004HLJU74xIfjke-kKpRXcS6O3yHS1f07IvRPWfXuy55Uh81Jo7ygMrXmWIe5TeBBWTiX-3J-QX1CtnkNnPSHtxM8RKjnLNL9A76qXaQyEZwPmFhR-MKWE7xqt0RjJm9arbycT6qL7OGLWA6K8ZdznOfqobUUSwU0dqjA0ajUzOrXfUeiiAhbY5lw4l_ONgnE3L0TYSDqHyRwMZlHqnhn20LCvuV3FXoQw9saymYDK1fBivF-kD7EkBSPBjQ0dfJ9Oq3zO5CfuaOvwFErbIswoUcJtLT_InITSw42Zlbw7y7SLmRWarz7i9bKdPoZ_FNZUr7V5USLE5NgmM3VQebyujm_bo2uXVYFHZuUz46vq0Aps7rPUs6KNebtcjuovWRQrEeQygM8D41zqDPOu4H4W4QIjdBhfCB1C5RbH7-C3g68oFKNRaV503MGFxa4glEJHGb7IzpFCetg58FJoYZkXDsenmDIHWEpe1bsAF1mASwxQykJlJHBk0P-OSfUidVhWX4wx3MfKsBDVB0ynjS66Wp5RMeK4_ScRzkPevvzfnnSxYitXBGNPSkBXMRnt5sIQE118dn6VImAqlQr2atmiDmdyIYK9abaK1-qLDP08qMfLVknr7dLesj4n9QvSLuytxA6pjMkVc64npiCWXliNk_6pSGuHAW16uaVsxZTUi3GWpUYPaZ0e0WPaoCf0FN4I_U8cXnz_u3HeB_pY6DVjd6vSIpREeAUwGeL9uE5BE4J3SiQmJMCfIbOL9G7cII4l3oxXmpPA20RQYk0ym5NgypTDtyQOcUB7kuGdGpWQmOlHY3ZfSbAmSxLUKFmRoF5vHpw2z-qNxmGj2TiqHR9vKPmbEWoHZ_lqnp4cHSJo8w-N09JJ)
```
+-------------------------+
|       Users (DID)       |
| (Patients, Providers)   |
+-----------+-------------+
            |
            v
+-------------------------+
|  Authentication (DID)   |
|  MetaMask/WalletConnect |
+-------------------------+
            |
            v
+--------------------------------------------+
|        Smart Contracts (Ethereum L2)       |
|  +---------------------+  +--------------+ |
|  | Access Control SC   |  | Consent SC   | |
|  +---------------------+  +--------------+ |
|           +-----------------------------+  |
|           | Audit Log SC (Transparency) |  |
|           +-----------------------------+  |
+--------------------------------------------+
            |
            v
+-------------------------+
|  Encrypted Storage (IPFS) |
+-------------------------+
            |
            v
+--------------------------------------+
|   Zero-Knowledge Proofs (ZK-SNARKs)  |
+--------------------------------------+
            |
            v
+------------------------------------------+
|            Backend (Node.js)             |
+------------------------------------------+
            |
            v
+------------------------------------------+
|         Frontend (React + Tailwind)      |
+------------------------------------------+
```

## Installation & Setup
### Prerequisites
- Node.js & npm
- MetaMask Wallet
- Solidity development environment (Remix, Hardhat, or Truffle)
- IPFS node setup

### Steps
1. Clone the repository:
   ```sh
   git clone https://github.com/Misbah-Engr/Afit-ID.git
   cd healthcare-dapp
   ```
2. Install dependencies:
   ```sh
   npm install
   ```
3. Deploy smart contracts:
   ```sh
   npx hardhat run scripts/deploy.js --network polygon
   ```
4. Start IPFS node:
   ```sh
   ipfs daemon
   ```
5. Run the backend server:
   ```sh
   npm run backend
   ```
6. Start the frontend application:
   ```sh
   npm start
   ```

## Usage
- Patients can upload encrypted medical records and grant access.
- Providers can request access to patient data.
- Verifiers can use ZK-SNARKs to confirm medical conditions without full record access.

## License
This project is licensed under the MIT License.

## Contributors
- [AlAmin](https://x.com/n00r_Btc)
- [misbahu.eth](https://x.com/Designer_Misbah)

For contributions, open a pull request or contact us via [email].

