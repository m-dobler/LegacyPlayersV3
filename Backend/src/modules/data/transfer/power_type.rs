use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::PowerType;
use crate::modules::data::tools::RetrievePowerType;

#[openapi]
#[get("/power_type/<id>")]
pub fn get_power_type(me: State<Data>, id: u8) -> Option<Json<PowerType>>
{
  me.get_power_type(id)
    .and_then(|power_type| Some(Json(power_type)))
}

#[openapi]
#[get("/power_type")]
pub fn get_all_power_types(me: State<Data>) -> Json<Vec<PowerType>>
{
  Json(me.get_all_power_types())
}