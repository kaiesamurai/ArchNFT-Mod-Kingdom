# ArchNFT-Mod-Kingdom

ArchNFT-Mod-Kingdom is an innovative NFT marketplace built on the Archway network, aimed at creating a unified platform for trading NFTs across various Cosmos chains. By leveraging Archway's low fees and sustainable architecture, ArchNFT-Mod-Kingdom aspires to be the go-to marketplace for NFT enthusiasts and creators alike.

## Inspiration

With the growing number of Cosmos chains and the emergence of NFT projects and marketplaces, we saw the need for a comprehensive marketplace where NFTs from various chains can be traded seamlessly. Our vision is to create a marketplace that is accessible, user-friendly, and economically viable, and we believe that Archway is the ideal platform to achieve this.

## What It Does

ArchNFT-Mod-Kingdom allows users to:

- Easily publish NFTs by filling out a simple form with details such as name, image URL, and description.
- Use CONST, the native token of the Archway Testnet, to obtain Wrapped tokens for buying and selling NFTs.
- Trade NFTs with minimal fees, ensuring a sustainable marketplace environment.

## How We Built It

We utilized the official Archway CLI to create and deploy smart contracts for the NFT marketplace, NFT issuance, and Wrapped Token issuance. Our tech stack includes:

- **Frontend:** Built using Typescript, React, Next.js, and Tailwind CSS.
- **Blockchain Connection:** Implemented using cosmjs and Keplr to connect Archway's testnet to the Dapp.
- **Deployment:** The frontend is deployed on Vercel, making it accessible to everyone.

## Challenges We Ran Into

- **Testnet Maintenance:** Post-maintenance misconfiguration of the RPC node made it challenging to reference queries or execute transactions for several days. This was resolved with the assistance of the Archway management team.
- **CLI Handling:** Managing Archway's CLI was initially difficult due to its unique specifications compared to other CosmWasm CLIs. We overcame this by setting up a virtual Ubuntu environment for development.

## Accomplishments That We're Proud Of

- Successfully laying the foundation for a meaningful project on Archway and Cosmos.
- Completing the smart contract and front-end development within the allocated timeframe.

## What We Learned

- Managing complex contracts using CW20 and CW721.
- Developing Dapps with Keplr integration.
- Collaborating effectively with the developer community.

## What's Next for ArchNFT-Mod-Kingdom

- **NFT Collection:** Enhance the platform with curated NFT collections.
- **NFT Auction:** Implement auction functionality for NFTs.
- **Floor Price Reference:** Provide reference prices based on collection or attribute.
- **NFT Launch Pad:** Introduce a launch pad for new NFT projects.
- **IBC Integration:** Enable handling of NFTs from other chains via Inter-Blockchain Communication (IBC).

## Built With

- **Archway**
- **Cosmjs**
- **CosmWasm**
- **Keplr**
- **Next.js**
- **React**
- **Rust**
- **Typescript**
- **Vercel**

## Getting Started

To get started with ArchNFT-Mod-Kingdom:

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/archnft-mod-kingdom.git
   cd archnft-mod-kingdom
   ```

2. Install dependencies:

   ```bash
   npm install
   ```

3. Set up your environment variables:

   Create a `.env` file in the root directory and add the necessary environment variables.

4. Run the development server:

   ```bash
   npm run dev
   ```

5. Open your browser and navigate to `http://localhost:3000`.


We hope ArchNFT-Mod-Kingdom will serve as a catalyst for the growth of the Cosmos ecosystem, providing a vibrant and dynamic platform for NFT trading and innovation.