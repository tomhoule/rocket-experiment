use rocket_contrib::Json;

use schemas::ethics::*;
use schemas::ethics::schema::ETHICA;

#[get("/v1/ethics/schema")]
pub fn schema() -> Json<Schema> {
    Json(ETHICA)
}
