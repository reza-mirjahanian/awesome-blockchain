https://docs.solanapay.com/

-   **Solana Pay** is a specification for encoding Solana transaction requests within URLs, enabling standardized transaction requests across different Solana apps and wallets.
-   **Partial signing** of transactions allows the creation of transactions that require multiple signatures before they are submitted to the network.
-   **Transaction gating** involves implementing rules that determine whether certain transactions are allowed to be processed, based on specific conditions or the presence of particular data in the transaction.


--------------------------

# **Solana Pay Overview**

## **What is Solana Pay?**
- **Decentralized, Open Source, Peer-to-Peer Payments Protocol**
  - Built on the **Solana network**.
  - Specifies a **standard way** to create and interpret **payment requests**.
  - Simplifies payments for **wallets** and **apps** like e-commerce stores.

## **How It Works**
- **Uses URLs** (similar to web addresses) to compose transactions.
  - **URL parameters** define what's being sent.
  - Supports fields like:
    - **Reference**
    - **Memo** (to store payment details on-chain).

### **Payment Tracking**
- In a transfer request:
  - Use the **memo field** to track the payment.
  - Use **option label** and **message fields** to identify it.

### **QR Codes and NFC Tags**
- URLs can be embedded in:
  - **QR codes**
  - **NFC tags**
- Ideal for:
  - **Point-of-sale terminals**
  - **Physical products**

## **Interactive Transaction Requests**
- URLs contain links for wallet interaction.
- Process:
  1. User scans a QR code.
  2. Wallet creates a **POST request** to the server with the user's account address.
  3. Server creates a **custom transaction**.
  4. Custom transaction is sent back to the wallet.
  5. Wallet parses and presents the transaction to the user.
  6. If the user signs, the transaction is broadcasted to the blockchain.
  7. Server releases the product.

### **E-commerce Integration**
- Both **Shopify** and **WooCommerce** have integrated Solana Pay.
- Example: The **official Solana merch store** allows seamless purchases (e.g., buying hats).

## **Advantages of On-Chain Payments**
- **Loyalty Programs Made Easy**
  - Traditionally expensive and cumbersome.
  - Businesses often still use punch cards.
  - With Solana Pay:
    - Issue **NFTs** redeemable for discounts on future orders.

### **Blockchain Benefits**
- **No payment processor fees**
- **No holding times**
- **No chargebacks**

## **Getting Started**
