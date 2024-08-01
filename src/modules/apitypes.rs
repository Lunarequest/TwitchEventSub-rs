//use twitch_eventsub_structs::serde::{Deserialize as Deserialise, Serialize as Serialise};
#[macro_use]
use crate::serde::{self, Deserialize as Deserialise, Serialize as Serialise};

use twitch_eventsub_structs::User;

#[derive(Serialise, Deserialise, Debug)]
#[serde(crate = "self::serde")]
pub struct AdSchedule {
  pub data: Vec<AdDetails>,
}

#[derive(Serialise, Deserialise, Debug)]
#[serde(crate = "self::serde")]
pub struct AdDetails {
  pub next_ad_at: u32,
  pub last_ad_at: u32,
  pub duration: u32,
  pub preroll_free_time: u32,
  pub snooze_count: u32,
  pub snooze_refresh_at: u32,
}

#[derive(Serialise, Deserialise, Debug)]
#[serde(crate = "self::serde")]
pub struct Pagination {
  pub cursor: Option<String>,
}

#[derive(Serialise, Deserialise, Debug)]
#[serde(crate = "self::serde")]
pub struct GetChatters {
  pub data: Vec<User>,
  pub pagination: Pagination,
  pub total: i32,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
#[serde(crate = "self::serde")]
pub struct EmoteStaticImages {
  pub url_1x: String,
  pub url_2x: String,
  pub url_4x: String,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
#[serde(crate = "self::serde")]
pub enum EmoteType {
  #[serde(rename = "bitstier")]
  BitsTier,
  #[serde(rename = "follower")]
  Follower,
  #[serde(rename = "subscriptions")]
  Subscriptions,
  #[serde(rename = "globals")]
  Global,
  #[serde(rename = "prime")]
  Prime,
  #[serde(rename = "turbo")]
  Turbo,
  #[serde(rename = "smilies")]
  Smilies,
  #[serde(rename = "limitedtime")]
  LimitedTime,
  #[serde(rename = "rewards")]
  Rewards,
  #[serde(rename = "none")]
  None,
  #[serde(rename = "owl2019")]
  Owl2019,
  #[serde(rename = "hypetrain")]
  HypeTrain,
  #[serde(rename = "PSTgasm")]
  PSTGasm,
  #[serde(rename = "twofactor")]
  TwoFactor,
}

#[derive(Serialise, Deserialise, Debug, PartialEq, Clone)]
#[serde(crate = "self::serde")]
pub enum EmoteFormat {
  #[serde(rename = "static")]
  Static,
  #[serde(rename = "animated")]
  Animated,
}

impl EmoteFormat {
  pub fn string(&self) -> String {
    match self {
      EmoteFormat::Static => "static",
      EmoteFormat::Animated => "animated",
    }
    .to_string()
  }
}

#[derive(Serialise, Deserialise, Debug, Clone)]
#[serde(crate = "self::serde")]
pub enum ThemeMode {
  #[serde(rename = "light")]
  Light,
  #[serde(rename = "dark")]
  Dark,
}

impl ThemeMode {
  pub fn string(&self) -> String {
    match self {
      ThemeMode::Light => "light",
      ThemeMode::Dark => "dark",
    }
    .to_string()
  }
}

#[derive(Serialise, Deserialise, Debug, Clone)]
#[serde(crate = "self::serde")]
pub struct EmoteData {
  pub id: String,
  pub name: String,
  pub images: EmoteStaticImages,
  pub tier: String,
  pub emote_type: EmoteType,
  pub emote_set_id: String,
  pub format: Vec<EmoteFormat>,
  pub scale: Vec<String>,
  pub theme_mode: Vec<ThemeMode>,
}

#[derive(Serialise, Deserialise, Debug)]
#[serde(crate = "self::serde")]
pub struct ChannelEmotes {
  pub data: Vec<EmoteData>,
  pub template: String,
}

impl ChannelEmotes {
  pub fn from_id<S: Into<String>>(&self, id: S) -> EmoteData {
    let id = id.into();
    self
      .data
      .iter()
      .filter(|emote| id == emote.id)
      .nth(0)
      .unwrap()
      .clone()
  }

  pub fn from_idx(&self, idx: usize, animated: bool) -> String {
    self.from_emote(&self.data[idx], animated)
  }

  pub fn from_emote(&self, emote: &EmoteData, animated: bool) -> String {
    self.get_emote(
      &emote.id,
      emote.format[if animated { 1 } else { 0 }].string(),
      emote.theme_mode[0].string(),
      &emote.scale[0],
    )
  }

  pub fn get_emote<S: Into<String>, T: Into<String>, X: Into<String>, Z: Into<String>>(
    &self,
    id: S,
    format: T,
    theme_mode: X,
    scale: Z,
  ) -> String {
    self
      .template
      .replace("{{id}}", &id.into())
      .replace("{{format}}", &format.into())
      .replace("{{theme_mode}}", &theme_mode.into())
      .replace("{{scale}}", &scale.into())
  }
}
