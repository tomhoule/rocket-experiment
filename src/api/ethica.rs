use rocket_contrib::Json;

use schemas::ethica::*;
use schemas::ethica::schema::ETHICA;

#[get("/api/ethica/schema")]
pub fn schema() -> Json<Schema> {
    Json(ETHICA)
}
