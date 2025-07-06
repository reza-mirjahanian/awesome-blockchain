---

# 🌐 **What Are NATs?**

## 🧭 **Overview**

* The internet connects many **private networks** via shared address spaces using **transport protocols**.
* **NAT (Network Address Translation)** maps addresses between internal and external networks.

---

## 🧱 **Why NAT Exists**

* IPv4 has limited addresses (32-bit). NAT enables:

  * Multiple devices to share **one public IP**.
  * Use of **private IP ranges** like `10.0.0.0/8`, `192.168.0.0/16`.

> 📦 Example:
> Your device gets IP `10.0.1.15` from your home router.
> When you access the internet, the router replaces it with its **public IP**.
> Response traffic is translated back to your device.

---

## 🚫 **NAT Limitations**

* **Outgoing connections**: usually work fine.
* **Incoming connections**: require manual or automated configuration:

  * One public IP → many internal devices.
  * Router must **map specific ports** to internal IPs (port forwarding).

---

## ⚠️ **Challenges for P2P**

* Manual port forwarding isn't always feasible.
* Users might lack access or knowledge to configure routers.

---

# ⚙️ **NAT Traversal in libp2p**

## ✅ **Automatic Router Configuration**

* Many routers support **automatic port forwarding** via:

  * **UPnP (Universal Plug and Play)**
  * **NAT-PMP (NAT Port Mapping Protocol)**

> 🛠️ If supported:
>
> * libp2p attempts **automatic port mapping**.
> * Easiest way to enable incoming connections.

---
