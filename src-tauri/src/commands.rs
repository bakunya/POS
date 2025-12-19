use tauri::{AppHandle, State, Window};
use crate::db::{AppState, Product};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
    user: Option<String>,
    role: Option<String>,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    pin: String,
    role: String,
}

#[tauri::command]
pub async fn login(state: State<'_, AppState>, pin: String) -> Result<LoginResponse, String> {
    let db = &state.db;
    
    // In a real app we would query carefully. Here we scan users.
    // SELECT * FROM users WHERE pin = $pin
    let mut result = db.query("SELECT * FROM users WHERE pin = $pin")
        .bind(("pin", pin))
        .await
        .map_err(|e| e.to_string())?;

    let users: Vec<User> = result.take(0).map_err(|e| e.to_string())?;

    if let Some(user) = users.first() {
        Ok(LoginResponse {
            success: true,
            user: Some(user.name.clone()),
            role: Some(user.role.clone()),
            message: None,
        })
    } else {
        Ok(LoginResponse {
            success: false,
            user: None,
            role: None,
            message: Some("Invalid PIN".into()),
        })
    }
}

#[tauri::command]
pub async fn start_shift(state: State<'_, AppState>, cashier: String, float_amount: f64) -> Result<String, String> {
    let db = &state.db;
    let shift = serde_json::json!({
        "cashier": cashier,
        "start_time": chrono::Local::now(),
        "float_amount": float_amount,
        "status": "active"
    });

    let _: Option<serde_json::Value> = db.create("shifts").content(shift).await.map_err(|e| e.to_string())?;
    
    Ok("Shift started".into())
}

#[tauri::command]
pub async fn end_shift(state: State<'_, AppState>, cashier: String, final_cash: f64) -> Result<String, String> {
     let db = &state.db;
     // Find active shift for cashier (simplified)
     // In real app: Update the shift record with end_time and final_cash
     
     Ok("Shift ended (Mock)".into())
}

#[tauri::command]
pub async fn search_product(state: State<'_, AppState>, query: String) -> Result<Vec<Product>, String> {
    let db = &state.db;
    
    // Simple search: check barcode exactly, or name contains query
    // In SurrealQL: SELECT * FROM products WHERE barcode = $query OR name ~ $query
    let mut result = db.query("SELECT * FROM products WHERE barcode = $query OR string::lowercase(name) CONTAINS string::lowercase($query)")
        .bind(("query", query))
        .await
        .map_err(|e| e.to_string())?;

    let products: Vec<Product> = result.take(0).map_err(|e| e.to_string())?;
    Ok(products)
}

#[derive(Deserialize, Serialize)]
pub struct CartItem {
    id: String,
    qty: i32,
    price: f64,
}

#[tauri::command]
pub async fn process_transaction(state: State<'_, AppState>, cashier: String, items: Vec<CartItem>, total: f64, payment: f64) -> Result<String, String> {
    let db = &state.db;
    
    // 1. Create Transaction Record
    let tx = serde_json::json!({
        "cashier": cashier,
        "timestamp": chrono::Local::now(),
        "total": total,
        "payment": payment,
        "change": payment - total,
        "items": items,
    });
    
    let _: Option<serde_json::Value> = db.create("transactions").content(tx).await.map_err(|e| e.to_string())?;

    // 2. Decrease Stock (simplified loop)
    for item in items {
        // UPDATE products SET stock = stock - $qty WHERE id = $id
        // Using a basic query for each item
        let _ = db.query("UPDATE type::thing($id) SET stock = stock - $qty")
            .bind(("id", item.id))
            .bind(("qty", item.qty))
            .await;
    }

    Ok("Transaction Success".into())
}
