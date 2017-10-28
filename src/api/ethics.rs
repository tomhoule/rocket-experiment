use rocket_contrib::Json;

use schemas::ethics::*;
use schemas::ethics::schema::ETHICA;

#[get("/api/ethics/schema")]
pub fn schema() -> Json<Schema> {
    Json(ETHICA)
}
