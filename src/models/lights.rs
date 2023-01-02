use serde::{Deserialize, Serialize};

use crate::color::{Color, Component, Temperature};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct On {
	pub on: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimming {
	pub brightness: f32,
	pub min_dim_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dynamics {
	pub status: String,
	pub status_values: Vec<String>,
	pub speed: f32,
	pub speed_valid: bool

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLightsResponse {
	pub data: Option<Vec<GetLightsResponseItem>>,
	pub error: Option<crate::models::Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLightsResponseItem {
	#[serde(rename = "type")]
	pub r#type: String,

	pub id: uuid::Uuid,
	pub metadata: super::generic::Metadata,
	pub dimming: Option<Dimming>,
	pub dynamics: Option<Dynamics>,
	pub on: On,

	pub color: Option<Color>,
	pub color_temperature: Option<Temperature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightOnRequest {
	pub on: On,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSetColorRequestXY {
	pub xy: Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSetColorRequest {
	pub color: LightSetColorRequestXY,
	pub dynamics: LightSetRequestDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSetBrightnessRequestBrightness {
	pub brightness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSetBrightnessRequest {
	pub dimming: LightSetBrightnessRequestBrightness,
	pub dynamics: LightSetRequestDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSetRequestDuration {
	pub duration: i32,
	pub speed: f32
}



impl LightOnRequest {
	pub fn new(on: bool) -> LightOnRequest {
		LightOnRequest { on: On { on } }
	}
}

impl LightSetColorRequest {
	pub fn new(color: Component, speed: f32) -> LightSetColorRequest {
		LightSetColorRequest {
			color: LightSetColorRequestXY { xy: color },
		},
		dynamics : LightSetRequestDuration{
			duration: 0,// value dose not need to be set 
			speed: speed.max(0.0).min(1.0),
		}
	}
}

impl LightSetBrightnessRequest {
	pub fn new(brightness: f32,speed: f32) -> LightSetBrightnessRequest {
		LightSetBrightnessRequest {
			dimming: LightSetBrightnessRequestBrightness {
				brightness: brightness.max(0.0).min(100.0),
			},
			dynamics : LightSetRequestDuration{
				duration: 0,// value dose not need to be set 
				speed: speed.max(0.0).min(1.0),
			}

		}
	}
}
