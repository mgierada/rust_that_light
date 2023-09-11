use std::env::{self, var};

use govee_api::structs::govee::{GoveeCommand, PayloadBody};

use crate::constants::enums::{Device, OfficeDevices};

pub fn tv_light_setup(command: &str) -> PayloadBody {
    let goove_api_device =
        var("GOVEE_DEVICE_ID_TV_LIGHT").expect("GOVEE_DEVICE_ID_TV_LIGHT must be set");
    let goove_model = var("GOVEE_MODEL_TV_LIGHT").expect("GOVEE_MODEL_TV_LIGHT must be set");
    let command = GoveeCommand {
        name: "turn".to_string(),
        value: command.to_string(),
    };
    PayloadBody {
        device: goove_api_device,
        model: goove_model,
        cmd: command,
    }
}

impl OfficeDevices {
    pub fn board_led() -> Self {
        let office_board_led_id =
            env::var("OFFICE_BOARD_LED_ID").expect("OFFICE_BOARD_LED_ID must be set");
        let office_board_led_model =
            env::var("OFFICE_BOARD_LED_MODEL").expect("OFFICE_BOARD_LED_MODEL must be set");
        let board_led = Device {
            device_id: office_board_led_id,
            model: office_board_led_model,
        };
        OfficeDevices::BoardLED(board_led)
    }

    pub fn corner_led() -> Self {
        let office_corner_light_id =
            env::var("OFFICE_CORNER_LIGHT_ID").expect("OFFICE_CORNER_LIGHT_ID must be set");
        let office_corner_light_model =
            env::var("OFFICE_CORNER_LIGHT_MODEL").expect("OFFICE_CORNER_LIGHT_MODEL must be set");
        let corner_led = Device {
            device_id: office_corner_light_id,
            model: office_corner_light_model,
        };
        OfficeDevices::CornerLED(corner_led)
    }

    pub fn table_led() -> Self {
        let office_table_led_id =
            env::var("OFFICE_TABLE_LED_ID").expect("OFFICE_TABLE_LED_ID must be set");
        let office_table_led_model =
            env::var("OFFICE_TABLE_LED_MODEL").expect("OFFICE_TABLE_LED_MODEL must be set");
        let table_led = Device {
            device_id: office_table_led_id,
            model: office_table_led_model,
        };
        OfficeDevices::TableLED(table_led)
    }

    pub fn window_led() -> Self {
        let office_window_led_id =
            env::var("OFFICE_WINDOW_LED_ID").expect("OFFICE_WINDOW_LED_ID must be set");
        let office_window_led_model =
            env::var("OFFICE_WINDOW_LED_MODEL").expect("OFFICE_WINDOW_LED_MODEL must be set");
        let window_led = Device {
            device_id: office_window_led_id,
            model: office_window_led_model,
        };
        OfficeDevices::WindowLED(window_led)
    }
}

pub fn office_light_setup(device: &OfficeDevices, command: &str) -> PayloadBody {
    let command = GoveeCommand {
        name: "turn".to_string(),
        value: command.to_string(),
    };
    match device {
        OfficeDevices::BoardLED(board_led) => PayloadBody {
            device: board_led.device_id.clone(),
            model: board_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::CornerLED(corner_led) => PayloadBody {
            device: corner_led.device_id.clone(),
            model: corner_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::TableLED(table_led) => PayloadBody {
            device: table_led.device_id.clone(),
            model: table_led.model.clone(),
            cmd: command,
        },
        OfficeDevices::WindowLED(window_led) => PayloadBody {
            device: window_led.device_id.clone(),
            model: window_led.model.clone(),
            cmd: command,
        },
    }
}
