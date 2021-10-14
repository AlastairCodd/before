use crate::cookies::CookieJarExt;
use crate::favorite_team::FavoriteTeam;
use crate::idol::Idol;
use crate::offset::OffsetTime;
use crate::settings::{DisableMotion, LightMode};
use crate::snacks::{Snack, SnackPack};
use crate::squirrels::Squirrels;
use crate::tarot::Spread;
use crate::time::{datetime, DateTime};
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::Deserialize;
use serde_json::{json, Value};
use std::ops::Deref;

const SIM_NO_COIN: DateTime = datetime!(2021-07-30 03:00:15.845649 UTC);

#[get("/api/getActiveBets")]
pub(crate) fn get_active_bets() -> Json<Vec<()>> {
    Json(vec![])
}

#[get("/api/getUser")]
pub(crate) fn get_user(cookies: &CookieJar<'_>, time: OffsetTime) -> Json<Value> {
    lazy_static::lazy_static! {
        /// Infinity is not representable in JSON, but JavaScript will treat double-precision
        /// floating point overflows as infinity. `1e1000` is sufficient to do this, but requires
        /// the `arbitrary_precision` feature of serde_json.
        ///
        /// ```text
        /// $ node
        /// Welcome to Node.js v14.18.0.
        /// Type ".help" for more information.
        /// > JSON.parse("1e1000")
        /// Infinity
        /// ```
        static ref INFINITY: Value = serde_json::from_str::<Value>("1e1000").unwrap();

        /// Allocates due to `arbitrary_precision`, so let's only do this once.
        static ref ZERO: Value = Value::from(0);
    }

    let coins = if time.0 >= SIM_NO_COIN {
        ZERO.deref()
    } else {
        INFINITY.deref()
    };

    let snacks = cookies.load::<SnackPack>().unwrap_or_default();

    Json(json!({
        "id": "be457c4e-79e6-4016-94f5-76c6705741bb",
        "email": "before@sibr.dev",
        // disable ability to change email on frontend
        "appleId": "what's umpdog",
        "verified": true,

        "motion": cookies.load::<DisableMotion>().unwrap_or_default(),
        "lightMode": cookies.load::<LightMode>().unwrap_or_default(),

        "idol": cookies.load::<Idol>(),
        "favoriteTeam": cookies.load::<FavoriteTeam>(),

        "unlockedShop": true,
        "unlockedElection": true,

        "squirrels": cookies.load::<Squirrels>().unwrap_or_default(),

        "coins": coins,
        "votes": snacks.get(Snack::Votes).unwrap_or_default(),
        "peanuts": snacks.get(Snack::Peanuts).unwrap_or_default(),
        "spread": cookies.load::<Spread>().unwrap_or_else(Spread::generate),
        "relics": {
            "Idol_Strikeouts": snacks.get(Snack::IdolStrikeouts).unwrap_or_default(),
            "Idol_Shutouts": snacks.get(Snack::IdolShutouts).unwrap_or_default(),
            "Idol_Homers": snacks.get(Snack::IdolHomers).unwrap_or_default(),
            "Idol_Hits": snacks.get(Snack::IdolHits).unwrap_or_default(),
        },

        "snacks": snacks.amounts(),
        "snackOrder": snacks.order(),
        "packSize": snacks.len(),

        // set all these to reasonably high values to avoid rendering the "what to do next" actions
        // in the bulletin
        "trackers": {
            "BEGS": 3,
            "BETS": 10,
            "VOTES_CAST": 1,
            "SNACKS_BOUGHT": 2,
            "SNACK_UPGRADES": 3,
        },
    }))
}

#[get("/api/getUserRewards")]
pub(crate) fn get_user_rewards() -> Json<Option<()>> {
    Json(None)
}

#[get("/api/getUserNotifications")]
pub(crate) fn get_user_notifications() -> Json<Option<()>> {
    Json(None)
}

#[post("/api/clearUserNotifications")]
pub(crate) fn clear_user_notifications() -> Json<Option<()>> {
    Json(None)
}

#[derive(Deserialize)]
pub(crate) struct EatADangPeanut {
    pub(crate) amount: i32,
}

#[post("/api/eatADangPeanut", data = "<dang_peanut>")]
pub(crate) fn eat_a_dang_peanut(
    cookies: &CookieJar<'_>,
    dang_peanut: Json<EatADangPeanut>,
) -> Json<Value> {
    cookies.add(crate::new_cookie(
        "peanuts",
        (cookies
            .get_pending("peanuts")
            .and_then(|t| t.value().parse::<i32>().ok())
            .unwrap_or(0)
            - dang_peanut.amount)
            .to_string(),
    ));
    Json(json!({}))
}

#[post("/api/buyADangPeanut")]
pub(crate) fn buy_a_dang_peanut(cookies: &CookieJar<'_>) -> Json<Value> {
    let peanuts = cookies
        .get_pending("peanuts")
        .and_then(|t| t.value().parse::<i32>().ok())
        .unwrap_or(0)
        + 1000;

    cookies.add(crate::new_cookie("peanuts", peanuts.to_string()));
    Json(json!({
        "message": format!("You receive 1000 peanuts. You now have {} peanuts", peanuts)
    }))
}
