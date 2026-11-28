// Netzwerk-Protokoll zwischen Host und Clients.
//
// Transport: TCP mit 4-Byte-Längen-Präfix + JSON.
// serde/serde_json sind bereits im Projekt - für ein Koop-Spiel mit
// 20 Snapshots pro Sekunde und wenigen Spielern reicht das völlig.

use std::io::{self, Read, Write};
use std::net::TcpStream;

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

use crate::ingame::animation::HumanoidState;
use crate::ingame::items::structs::ItemId;

pub const PROTOCOL_VERSION: u32 = 1;
pub const DEFAULT_PORT: u16 = 3455;
/// Schutz vor Müll-Daten: größte erlaubte Nachricht.
pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

// ---------------------------------------------------------------------
// Nachrichten
// ---------------------------------------------------------------------

/// Host -> Client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    JoinDenied { reason: String },
    /// Erste Nachricht nach erfolgreichem Join:
    /// Welt-Seed, eigene Spieler-ID und der aktuelle Weltzustand.
    Welcome {
        seed: u64,
        your_id: u64,
        players: Vec<NetPlayer>,
        enemies: Vec<NetEnemy>,
    },
    /// Regelmäßiger Weltzustand (~20x pro Sekunde).
    Snapshot {
        players: Vec<NetPlayer>,
        enemies: Vec<NetEnemy>,
    },
}

/// Client -> Host
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    JoinRequest { protocol: u32 },
    /// Position + Animation des eigenen Spielers
    /// (der Client ist für seinen eigenen Spieler zuständig).
    PlayerState { pos: (f32, f32), anim: HumanoidState },
    /// Angriff auslösen - der Host führt Treffer/Schaden aus.
    Attack { item: ItemId, aim: (f32, f32) },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetPlayer {
    pub id: u64,
    pub pos: (f32, f32),
    pub anim: HumanoidState,
    /// Läuft gerade ein Angriff? (`seq` zählt pro Angriff hoch,
    /// damit Empfänger denselben Angriff nicht doppelt abspielen.)
    pub attack: Option<NetAttack>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetAttack {
    pub seq: u32,
    pub item: ItemId,
    pub aim: (f32, f32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetEnemy {
    /// Entity-Bits auf dem Host - stabile ID übers Netzwerk.
    pub id: u64,
    pub pos: (f32, f32),
    pub anim: HumanoidState,
    pub health: f32,
    pub max_health: f32,
}

// ---------------------------------------------------------------------
// Framing: [u32 Länge][JSON-Bytes]
// ---------------------------------------------------------------------

pub fn write_message<T: Serialize>(stream: &mut TcpStream, msg: &T) -> io::Result<()> {
    let data = serde_json::to_vec(msg).map_err(|e| io::Error::other(e))?;
    stream.write_all(&(data.len() as u32).to_le_bytes())?;
    stream.write_all(&data)?;
    Ok(())
}

/// `Ok(None)` = Verbindung wurde sauber geschlossen.
pub fn read_message<T: DeserializeOwned>(stream: &mut TcpStream) -> io::Result<Option<T>> {
    let mut len_buf = [0u8; 4];
    match stream.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > MAX_MESSAGE_SIZE {
        return Err(io::Error::other(format!("message too large: {len}")));
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    let msg = serde_json::from_slice(&buf).map_err(|e| io::Error::other(e))?;
    Ok(Some(msg))
}