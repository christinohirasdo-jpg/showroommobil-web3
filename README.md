# 🚗 Car Showroom - Soroban Smart Contract

## 📌 Description
This project is a **Soroban (Stellar) smart contract** used to manage car showroom data.  
Users can add, view, and delete car data in a decentralized way.

---

## ⚙️ Main Features
- ➕ Add new car data
- 📋 View list of cars
- ❌ Delete car data

---

## 🧱 Data Structure

Each car has the following attributes:

- `id` → Unique car ID  
- `brand` → Car brand (e.g., Toyota, Honda)  
- `model` → Car model (e.g., Avanza, Civic)  
- `price` → Car price  

---

## 🛠️ Available Functions

### 1. get_cars()
Retrieve all car data stored on the blockchain.

---

### 2. add_car(brand, model, price)
Add a new car to the showroom.

**Parameters:**
- `brand` (String)
- `model` (String)
- `price` (u64)

**Returns:**
- Success message

---

### 3. delete_car(id)
Delete a car based on its ID.

**Parameters:**
- `id` (u64)

**Returns:**
- Success or failure message

---

## 🧪 How to Run

### 1. Install Soroban CLI
Make sure you have Rust and Soroban CLI installed.

### 2. Build the Project
```bash
cargo build --target wasm32-unknown-unknown --release