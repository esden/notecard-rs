//! <https://dev.blues.io/reference/notecard-api/hub-requests/>

#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use embedded_hal::blocking::i2c::{Read, SevenBitAddress, Write};
use serde::{Deserialize, Serialize};

use super::{FutureResponse, Notecard, NoteError};

pub struct Hub<'a, IOM: Write<SevenBitAddress> + Read<SevenBitAddress>> {
    note: &'a mut Notecard<IOM>,
}

impl<'a, IOM: Write<SevenBitAddress> + Read<SevenBitAddress>> Hub<'a, IOM> {
    pub fn from(note: &mut Notecard<IOM>) -> Hub<'_, IOM> {
        Hub { note }
    }

    /// Add a "device health" log message to send to Notehub on the next sync.
    pub fn log(
        self,
        text: &str,
        alert: bool,
        sync: bool,
    ) -> Result<FutureResponse<'a, res::Empty, IOM>, NoteError> {
        self.note.request(req::HubLog {
            req: "hub.log",
            text,
            alert,
            sync,
        })?;
        Ok(FutureResponse::from(self.note))
    }

    /// The [hub.set](https://dev.blues.io/reference/notecard-api/hub-requests/#hub-set) request is
    /// the primary method for controlling the Notecard's Notehub connection and sync behavior.
    pub fn set(
        self,
        product: Option<&str>,
        host: Option<&str>,
        mode: Option<req::HubMode>,
        sn: Option<&str>,
    ) -> Result<FutureResponse<'a, res::Empty, IOM>, NoteError> {
        self.note.request(req::HubSet {
            req: "hub.set",
            product,
            host,
            mode,
            sn,
        })?;
        Ok(FutureResponse::from(self.note))
    }

    /// Manually initiates a sync with Notehub.
    pub fn sync(self) -> Result<FutureResponse<'a, res::Empty, IOM>, NoteError> {
        self.note.request_raw(b"{\"req\":\"hub.sync\"}\n")?;
        Ok(FutureResponse::from(self.note))
    }

    /// Check on the status of a recently triggered or previous sync.
    pub fn sync_status(self) -> Result<FutureResponse<'a, res::SyncStatus, IOM>, NoteError> {
        self.note.request_raw(b"{\"req\":\"hub.sync.status\"}\n")?;
        Ok(FutureResponse::from(self.note))
    }
}

mod req {
    use super::*;

    #[derive(Deserialize, Serialize, defmt::Format)]
    #[serde(rename_all = "lowercase")]
    pub enum HubMode {
        Periodic,
        Continuous,
        Minimum,
        Off,
        DFU,
    }

    #[derive(Deserialize, Serialize, defmt::Format, Default)]
    pub struct HubSet<'a> {
        pub req: &'static str,

        pub product: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub host: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<HubMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sn: Option<&'a str>,
    }

    #[derive(Deserialize, Serialize, defmt::Format, Default)]
    pub struct HubLog<'a> {
        pub req: &'static str,
        pub text: &'a str,
        pub alert: bool,
        pub sync: bool,
    }
}

pub mod res {
    use super::*;

    #[derive(Deserialize, defmt::Format)]
    pub struct Empty {}

    #[derive(Deserialize, defmt::Format)]
    pub struct SyncStatus {
        pub status: heapless::String<1024>,
        pub time: Option<u32>,
        pub sync: Option<bool>,
        pub completed: Option<u32>,
        pub requested: Option<u32>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_empty() {
        serde_json_core::from_str::<res::Empty>(r#"{}"#).unwrap();
    }

    #[test]
    pub fn hub_set_some() {
        let hb = req::HubSet {
            req: "hub.set",
            product: Some("testprod"),
            host: Some("testhost"),
            mode: Some(req::HubMode::Periodic),
            ..Default::default()
        };

        assert_eq!(
            &serde_json_core::to_string::<_, 1024>(&hb).unwrap(),
            r#"{"req":"hub.set","product":"testprod","host":"testhost","mode":"periodic"}"#
        );
    }
}
