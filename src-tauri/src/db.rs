use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Surreal<Db>>,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    pin: String,
    role: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: String,
    pub barcode: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}

pub async fn setup_db(app_data_dir: std::path::PathBuf) -> Result<Arc<Surreal<Db>>, Box<dyn std::error::Error>> {
    let db_path = app_data_dir.join("minimarket.db");
    let db = Surreal::new::<SurrealKv>(db_path.to_str().unwrap()).await?;

    db.use_ns("minimarket").use_db("pos").await?;

    // Seed users if they don't exist
    let users: Vec<User> = db.select("users").await?;
    if users.is_empty() {
        let _ : Option<User> = db.create(("users", "admin")).content(User {
            id: "users:admin".to_string(),
            name: "Siti".to_string(),
            pin: "admin".to_string(),
            role: "owner".to_string(),
        }).await?;

        let _ : Option<User> = db.create(("users", "cashier")).content(User {
            id: "users:cashier".to_string(),
            name: "Budi".to_string(),
            pin: "1234".to_string(),
            role: "cashier".to_string(),
        }).await?;
        println!("Seeded initial users.");
    }
    
    // Seed sample products
    let products: Vec<Product> = db.select("products").await?;
    if products.is_empty() {
        let samples = vec![
            Product { id: "p1".into(), barcode: "8998866200570".into(), name: "Indomie Goreng".into(), price: 3500.0, stock: 100 },
            Product { id: "p2".into(), barcode: "8999909007609".into(), name: "Aqua 600ml".into(), price: 3000.0, stock: 50 },
            Product { id: "p3".into(), barcode: "8991906101234".into(), name: "Rokok Surya 16".into(), price: 25000.0, stock: 20 },
        ];
        
        for p in samples {
            let _: Option<Product> = db.create("products").content(p).await?;
        }
        println!("Seeded sample products.");
    }

    Ok(Arc::new(db))
}
