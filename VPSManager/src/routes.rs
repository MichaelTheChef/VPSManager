use rocket_contrib::json::{Json, JsonValue};
use crate::models::Vps;
use crate::database::DbConn;

#[post("/vps", format = "json", data = "<vps>")]
pub fn create_vps(vps: Json<Vps>, conn: DbConn) -> JsonValue {
    let inserted_vps = Vps::create(&conn, vps.into_inner());
    json!({ "message": "VPS created successfully", "vps": inserted_vps })
}

#[get("/vps/<id>")]
pub fn get_vps(id: u32, conn: DbConn) -> Option<Json<Vps>> {
    if let Some(vps) = Vps::find(&conn, id) {
        Some(Json(vps))
    } else {
        None
    }
}

#[put("/vps/<id>", format = "json", data = "<vps>")]
pub fn update_vps(id: u32, vps: Json<Vps>, conn: DbConn) -> JsonValue {
    let updated_vps = Vps::update(&conn, id, vps.into_inner());
    json!({ "message": "VPS updated successfully", "vps": updated_vps })
}

#[delete("/vps/<id>")]
pub fn delete_vps(id: u32, conn: DbConn) -> JsonValue {
    if Vps::delete(&conn, id) {
        json!({ "message": "VPS deleted successfully" })
    } else {
        json!({ "message": "VPS not found" })
    }
}
