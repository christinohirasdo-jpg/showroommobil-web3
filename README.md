# 🚗 Showroom Mobil - Soroban Smart Contract

## 📌 Deskripsi
Aplikasi ini adalah smart contract berbasis **Soroban (Stellar)** yang digunakan untuk mengelola data showroom mobil.  
Pengguna dapat menambahkan, melihat, dan menghapus data mobil secara terdesentralisasi.

---

## ⚙️ Fitur Utama
- ➕ Menambahkan data mobil
- 📋 Melihat daftar mobil
- ❌ Menghapus data mobil

---

## 🧱 Struktur Data

Setiap mobil memiliki atribut sebagai berikut:

- `id` → ID unik mobil  
- `brand` → Merk mobil (contoh: Toyota, Honda)  
- `model` → Tipe mobil (contoh: Avanza, Civic)  
- `price` → Harga mobil  

---

## 🛠️ Fungsi yang Tersedia

### 1. get_cars()
Mengambil seluruh data mobil yang tersimpan di blockchain.

### 2. add_car(brand, model, price)
Menambahkan mobil baru ke dalam showroom.

**Parameter:**
- `brand` (String)
- `model` (String)
- `price` (u64)

**Return:**
- Pesan sukses

---

### 3. delete_car(id)
Menghapus mobil berdasarkan ID.

**Parameter:**
- `id` (u64)

**Return:**
- Pesan berhasil / gagal

---

## 🧪 Cara Menjalankan

### 1. Install Soroban CLI
Pastikan sudah install Rust dan Soroban CLI.

### 2. Compile Project
```bash
cargo build --target wasm32-unknown-unknown --release

ID smart contract = CBBC47FVO2LI6DEQWJMMJLXZAPSPEYZ36CQEWRRUK4QJHLT3JA2RUCFZ