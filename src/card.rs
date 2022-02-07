//! https://dev.blues.io/reference/notecard-api/card-requests/

#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};
use serde::{Deserialize, Serialize};

use super::{FutureResponse, NoteError, Notecard};

pub struct Card<'a, IOM: Write<SevenBitAddress> + Read<SevenBitAddress>> {
    note: &'a mut Notecard<IOM>,
}

impl<'a, IOM: Write<SevenBitAddress> + Read<SevenBitAddress>> Card<'a, IOM> {
    pub fn from(note: &mut Notecard<IOM>) -> Card<'_, IOM> {
        Card { note }
    }

    /// Retrieves current date and time information. Upon power-up, the Notecard must complete a
    /// sync to Notehub in order to obtain time and location data. Before the time is obtained,
    /// this request will return `{"zone":"UTC,Unknown"}`.
    pub fn time(self) -> Result<FutureResponse<'a, res::Time, IOM>, NoteError> {
        self.note.request_raw(b"{\"req\":\"card.time\"}\n")?;
        Ok(FutureResponse::from(self.note))
    }

    /// Returns general information about the Notecard's operating status.
    pub fn status(self) -> Result<FutureResponse<'a, res::Status, IOM>, NoteError> {
        self.note.request_raw(b"{\"req\":\"card.status\"}\n")?;
        Ok(FutureResponse::from(self.note))
    }

    /// Performs a firmware restart of the Notecard.
    pub fn restart(self) -> Result<FutureResponse<'a, res::Empty, IOM>, NoteError> {
        self.note.request_raw(b"{\"req\":\"card.restart\"}\n")?;
        Ok(FutureResponse::from(self.note))
    }

    /// Retrieves the current location of the Notecard.
    pub fn location(self) -> Result<FutureResponse<'a, res::Location, IOM>, NoteError> {
        self.note.request_raw(b"{\"req\":\"card.location\"}\n")?;
        Ok(FutureResponse::from(self.note))
    }

    /// Sets location-related configuration settings. Retrieves the current location mode when passed with no argument.
    pub fn location_mode(
        self,
        mode: Option<&str>,
        seconds: Option<u32>,
        vseconds: Option<&str>,
        delete: Option<bool>,
        max: Option<u32>,
        lat: Option<f32>,
        lon: Option<f32>,
        minutes: Option<u32>,
    ) -> Result<FutureResponse<'a, res::LocationMode, IOM>, NoteError> {
        self.note.request(req::LocationMode {
            req: "card.location.mode",
            mode: mode.map(heapless::String::from),
            seconds,
            vseconds: vseconds.map(heapless::String::from),
            delete,
            max,
            lat,
            lon,
            minutes,
        })?;
        Ok(FutureResponse::from(self.note))
    }

    pub fn location_track(
        self,
        start: bool,
        heartbeat: bool,
        sync: bool,
        hours: Option<u32>,
        file: Option<&str>,
    ) -> Result<FutureResponse<'a, res::LocationTrack, IOM>, NoteError> {
        self.note.request(req::LocationTrack {
            req: "card.location.track",
            start: start.then(|| true),
            stop: (!start).then(|| true),
            heartbeat: heartbeat.then(|| true),
            sync: sync.then(|| true),
            hours,
            file: file.map(heapless::String::from),
        })?;

        Ok(FutureResponse::from(self.note))
    }

    pub fn wireless(self) -> Result<FutureResponse<'a, res::Wireless, IOM>, NoteError> {
        self.note.request_raw(b"{\"req\":\"card.wireless\"}\n")?;
        Ok(FutureResponse::from(self.note))
    }
}

pub mod req {
    use super::*;

    #[derive(Deserialize, Serialize, defmt::Format, Default)]
    pub struct LocationTrack {
        pub req: &'static str,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub start: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub heartbeat: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub sync: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub hours: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub file: Option<heapless::String<20>>,
    }

    #[derive(Deserialize, Serialize, defmt::Format, Default)]
    pub struct LocationMode {
        pub req: &'static str,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<heapless::String<20>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub seconds: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub vseconds: Option<heapless::String<20>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub delete: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub max: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub lat: Option<f32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub lon: Option<f32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub minutes: Option<u32>,
    }
}

pub mod res {
    use super::*;

    #[derive(Deserialize, defmt::Format)]
    pub struct Empty {}

    #[derive(Deserialize, defmt::Format)]
    pub struct LocationTrack {
        pub start: Option<bool>,
        pub stop: Option<bool>,
        pub heartbeat: Option<bool>,
        pub seconds: Option<u32>,
        pub hours: Option<u32>,
        pub file: Option<heapless::String<20>>,
    }

    #[derive(Deserialize, defmt::Format)]
    pub struct LocationMode {
        pub mode: heapless::String<20>,
        pub seconds: Option<u32>,
        pub vseconds: Option<heapless::String<20>>,
        pub max: Option<u32>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub minutes: Option<u32>,
    }

    #[derive(Deserialize, defmt::Format)]
    pub struct Location {
        pub status: heapless::String<120>,
        pub mode: heapless::String<20>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub time: Option<u32>,
        pub max: Option<u32>,
    }

    #[derive(Deserialize, defmt::Format)]
    pub struct Time {
        pub time: Option<u32>,
        pub area: Option<heapless::String<20>>,
        pub zone: Option<heapless::String<20>>,
        pub minutes: Option<i32>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub country: Option<heapless::String<10>>,
    }

    #[derive(Deserialize, defmt::Format)]
    pub struct Status {
        pub status: heapless::String<10>,
        #[serde(default)]
        pub usb: bool,
        pub storage: usize,
        pub time: Option<u64>,
        #[serde(default)]
        pub connected: bool,
    }

    #[derive(Deserialize, defmt::Format)]
    pub struct WirelessNet {
        iccid: Option<heapless::String<24>>,
        imsi: Option<heapless::String<24>>,
        imei: Option<heapless::String<24>>,
        modem: Option<heapless::String<35>>,
        band: Option<heapless::String<24>>,
        rat: Option<heapless::String<24>>,
        rssir: Option<i32>,
        rssi: Option<i32>,
        rsrp: Option<i32>,
        sinr: Option<i32>,
        rsrq: Option<i32>,
        bars: Option<i32>,
        mcc: Option<i32>,
        mnc: Option<i32>,
        lac: Option<i32>,
        cid: Option<i32>,
        updated: Option<u32>,
    }

    #[derive(Deserialize, defmt::Format)]
    pub struct Wireless {
        pub status: heapless::String<24>,
        pub count: Option<u8>,
        pub net: Option<WirelessNet>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NotecardError;

    #[test]
    fn test_card_wireless() {
        let r = br##"{"status":"{modem-on}","count":3,"net":{"iccid":"89011703278520607527","imsi":"310170852060752","imei":"864475044204278","modem":"BG95M3LAR02A03_01.006.01.006","band":"GSM 900","rat":"gsm","rssir":-77,"rssi":-77,"bars":3,"mcc":242,"mnc":1,"lac":11001,"cid":12313,"updated":1643923524}}"##;
        serde_json_core::from_slice::<res::Wireless>(r).unwrap();

        let r = br##"{"status":"{cell-registration-wait}","net":{"iccid":"89011703278520606586","imsi":"310170852060658","imei":"864475044197092","modem":"BG95M3LAR02A03_01.006.01.006"}}"##;
        serde_json_core::from_slice::<res::Wireless>(r).unwrap();

        let r = br##"{"status":"{modem-off}","net":{}}"##;
        serde_json_core::from_slice::<res::Wireless>(r).unwrap();
    }

    #[test]
    fn test_card_time_ok() {
        let r = br##"
        {
          "time": 1599769214,
          "area": "Beverly, MA",
          "zone": "CDT,America/New York",
          "minutes": -300,
          "lat": 42.5776,
          "lon": -70.87134,
          "country": "US"
        }
        "##;

        serde_json_core::from_slice::<res::Time>(r).unwrap();
    }

    #[test]
    fn test_card_time_err() {
        let r = br##"{"err":"time is not yet set","zone":"UTC,Unknown"}"##;
        serde_json_core::from_slice::<NotecardError>(r).unwrap();
    }

    #[test]
    pub fn test_status_ok() {
        serde_json_core::from_str::<res::Status>(
            r#"
          {
            "status":    "{normal}",
            "usb":       true,
            "storage":   8,
            "time":      1599684765,
            "connected": true
          }"#,
        )
        .unwrap();
    }

    #[test]
    pub fn test_status_mising() {
        serde_json_core::from_str::<res::Status>(
            r#"
          {
            "status":    "{normal}",
            "usb":       true,
            "storage":   8
          }"#,
        )
        .unwrap();
    }

    #[test]
    fn test_partial_location_mode() {
        serde_json_core::from_str::<res::LocationMode>(r#"{"seconds":60,"mode":"periodic"}"#)
            .unwrap();
    }

    #[test]
    fn test_location_searching() {
        serde_json_core::from_str::<res::Location>(
            r#"{"status":"GPS search (111 sec, 32/33 dB SNR, 0/1 sats) {gps-active} {gps-signal} {gps-sats}","mode":"continuous"}"#).unwrap();
    }

    #[test]
    fn test_location_mode_err() {
        let r = br##"{"err":"seconds: field seconds: unmarshal: expected a int32 {io}"}"##;
        serde_json_core::from_slice::<NotecardError>(r).unwrap();
    }
}
