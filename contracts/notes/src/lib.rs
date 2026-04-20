#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data mobil
#[contracttype]
#[derive(Clone, Debug)]
pub struct Car {
    id: u64,
    brand: String,
    model: String,
    price: u64,
}

// Key untuk storage
const CAR_DATA: Symbol = symbol_short!("CAR_DATA");

#[contract]
pub struct ShowroomContract;

#[contractimpl]
impl ShowroomContract {

    // 🔍 Ambil semua data mobil
    pub fn get_cars(env: Env) -> Vec<Car> {
        env.storage().instance().get(&CAR_DATA).unwrap_or(Vec::new(&env))
    }

    // ➕ Tambah mobil baru
    pub fn add_car(env: Env, brand: String, model: String, price: u64) -> String {
        let mut cars: Vec<Car> = env.storage().instance().get(&CAR_DATA).unwrap_or(Vec::new(&env));

        let car = Car {
            id: env.prng().gen::<u64>(),
            brand,
            model,
            price,
        };

        cars.push_back(car);
        env.storage().instance().set(&CAR_DATA, &cars);

        String::from_str(&env, "Mobil berhasil ditambahkan")
    }

    // ❌ Hapus mobil berdasarkan ID
    pub fn delete_car(env: Env, id: u64) -> String {
        let mut cars: Vec<Car> = env.storage().instance().get(&CAR_DATA).unwrap_or(Vec::new(&env));

        for i in 0..cars.len() {
            if cars.get(i).unwrap().id == id {
                cars.remove(i);
                env.storage().instance().set(&CAR_DATA, &cars);
                return String::from_str(&env, "Mobil berhasil dihapus");
            }
        }

        String::from_str(&env, "Mobil tidak ditemukan")
    }
}