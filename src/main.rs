#![allow(unused)]
use std::env;
use std::ops::Not;

use tokio::test;

use geo::point;
use geo::prelude::*;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use carapax::methods::SendPhoto;
use carapax::types::{
    InlineKeyboardButton, InputFile, KeyboardButton, Message, MessageData, TextEntity,
};
use carapax::types::{ReplyKeyboardMarkup, User};
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text},
    Api, App, Context, ExecuteError, Ref,
};

use crate::database::CoffeeHouse;
use crate::error_handler::Error;

mod database;
mod error_handler;
mod table_to_db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let spreadsheet_reader = table_to_db::to_base();
    if spreadsheet_reader.is_err() {
        println!("Table to db Error: {:?}", spreadsheet_reader);
    }
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN")?;
    let api = Api::new(token)?;

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await;

    Ok(())
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), Error> {
    if let MessageData::Location(location) = message.data {
        for cafe in distance(
            location.latitude.into(),
            location.longitude.into(),
            database::kofe_list().await?,
        ) {
            let caffee_description = &cafe.description;
            let mut vector: Vec<&str> = caffee_description.lines().collect();
            let name_length: u32 = vector[1].len().try_into()?;

            api.execute(
                SendPhoto::new(chat_id.clone(), InputFile::path(&cafe.photo).await?)
                    .caption(&cafe.description)
                    .caption_entities(&[TextEntity::bold(0..(name_length + 1))])
                    .expect("Failed to make caption bold."),
            )
            .await?;
            api.execute(
                SendMessage::new(chat_id.clone(), &cafe.address).reply_markup(vec![vec![
                    InlineKeyboardButton::with_url("📍Посмотреть на карте", &cafe.google_map),
                ]]),
            )
            .await?;
        }
        api.execute(SendMessage::new(
            chat_id.clone(),
            "Если мы снова вам понадобимся, отправьте гео-локацию в чат ☺️",
        ))
        .await?;
    } else {
        let send_location =
            KeyboardButton::request_location(KeyboardButton::new("Отправить гео локацию"));
        let key_raw = ReplyKeyboardMarkup::row(ReplyKeyboardMarkup::default(), vec![send_location]);
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_raw, true);
        let text =
            "Привет! Чтобы найти ближайшую кофейню, пожалуйста, пришлите свою гео-локацию в чат";
        let sendmessage = SendMessage::new(chat_id, text);
        let button_message = SendMessage::reply_markup(sendmessage.clone(), keyboard);
        api.execute(button_message).await?;
    };
    Ok(())
}

fn distance(
    lat_user: f64,
    lon_user: f64,
    mut list_of_coffe_houses: Vec<CoffeeHouse>,
) -> Vec<CoffeeHouse> {
    let point_user = point!(x: lat_user, y: lon_user);
    list_of_coffe_houses.sort_by(|a, b| {
        let distance_a = point_user.geodesic_distance(&point!(x: a.location_x, y: a.location_y));
        let distance_b = point_user.geodesic_distance(&point!(x: b.location_x, y: b.location_y));
        distance_a
            .abs()
            .partial_cmp(&distance_b.abs())
            .expect("Failed to compare points.")
    });
    list_of_coffe_houses.into_iter().take(3).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distance_gives_right_order() -> Result<(), Error> {
        let point0 = (41.6963678, 44.8199377);
        let point1 = (41.7255743, 44.746247);
        let point2 = (41.7106533, 44.7447204);
        let list_of_coffe_houses = database::kofe_list().await?;
        let distance_to_point_0 = distance(point0.0, point0.1, list_of_coffe_houses.clone());
        let distance_to_point_1 = distance(point1.0, point1.1, list_of_coffe_houses.clone());
        let distance_to_point_2 = distance(point2.0, point2.1, list_of_coffe_houses.clone());
        assert_ne!(distance_to_point_0, distance_to_point_1);
        assert_ne!(distance_to_point_1, distance_to_point_2);
        assert_ne!(distance_to_point_2, distance_to_point_0);
        dbg!(distance_to_point_0);
        dbg!(distance_to_point_1);
        dbg!(distance_to_point_2);
        Ok(())
    }

    #[tokio::test]
    async fn test_tbilisi() -> Result<(), Error> {
        let point0 = (41.720802, 44.721416);
        let point1 = (41.727481, 44.793525);
        let list_of_coffe_houses = database::kofe_list().await?;
        let distance_to_point_0 = distance(point0.0, point0.1, list_of_coffe_houses.clone());
        let distance_to_point_1 = distance(point1.0, point1.1, list_of_coffe_houses.clone());
        assert_ne!(distance_to_point_0, distance_to_point_1);
        dbg!(distance_to_point_0);
        dbg!(distance_to_point_1);
        Ok(())
    }
}
