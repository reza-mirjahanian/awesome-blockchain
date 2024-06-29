


# Liquidity Bridge for Crypto Exchanges

## Introduction

* Not about blockchain bridges

## Types of Liquidity Provision

1. **Organic liquidity**: Traders exchange assets directly
2. **Professional market makers**: Connect via API to provide liquidity
3. **API connection to larger exchanges**: Startup exchanges can connect to established venues

## Liquidity Bridge Ecosystem

* Connects external liquidity providers (market makers, exchanges, dark pools)
* Supplies liquidity to ecosystem partners (exchanges, brokerages)
* Allows risk hedging on external providers
* Facilitates liquidity distribution to various clients

## Key Components of Liquidity Bridge

### 1. Price Discovery
* Connects to makers using various technologies (WebSocket, REST API, FIX protocol)
* Supports proprietary methods for price discovery

### 2. Price Construction
* Applies markups, volume modifiers, and decimal precision adjustments
* Modifies order book depth

### 3. Liquidity Distribution
* Distributes constructed prices to client platforms and aggregators

## Execution Models

### 1. Post-Execution Model (for exchanges)
* Trade executed first
* Risk hedged afterwards

### 2. Pre-Execution Model (for brokerages)
* Order transmitted to liquidity bridge first
* Executed only after confirmation from hedging platform

## Hedging Strategies

* Flexible parameters: hedge ratio, order size limits
* Cross-asset hedging possibilities
* Venue-based, symbol-based, or user-based application
* Automated hedging options: transaction-based or systematic

## Reporting and Analytics

* Generates various reports
* Provides execution quality metrics
* Sends alerts and notifications

## Marksman Liquidity Hub

* Specific product implementing liquidity bridge functionality
* Supports major spot and derivatives exchanges
* Integrates with other platforms like B2Trader

## Company Overview (Beetle Broker)

* 11 offices, ~350 employees
* 7 regulatory licenses
* Offers various products:
  - B2Core (front-end, CRM)
  - B2Trader (exchange platform)
  - B2BinPay (payment solution)
  - Marksman (liquidity hub)
  - Other services (copy trading, digital banking)

