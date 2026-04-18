use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// A Rust type corresponding to a Torrent file.
#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub announce: String, // Tracker URL
    pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub length: i64, // Total file size
    pub name: String,

    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: Vec<[u8; 20]>, // Vector of SHA-1 hashes (20 bytes each)
}

/// Peer -> Tracker
/// 
/// A heartbeat + registration + status update in one message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnnounceRequest {
    pub info_hash: String, // To identify the swarm
    pub peer_id: [u8; 20],
    pub port: u16, // Port where other peers should connect

    // Peer state to determine seeders and leechers
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,

    pub event: Option<AnnounceEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnnounceEvent {
    Started, // First accounce
    Completed, // Become seeder
    Stopped, // Disconnect
}

/// Tracker -> Peer
#[derive(Debug, Serialize, Deserialize)]
pub struct AnnounceResponse {
    pub interval: u64, // Time before next AnnounceRequest
    pub peers: Vec<PeerContact>, // List of peers in the swarm
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerContact {
    pub peer_id: [u8; 20],
    pub ip: String,
    pub port: u16,
}

/// Peer -> Peer
#[derive(Debug, Serialize, Deserialize)]
pub enum PeerMessage {
    Handshake(PeerHandshake), // To establish connection
    Bitfield(Bitfield), // Which pieces the peer has

    // Indicates whether this peer wants to download pieces from the remote peer.
    // Determined by comparing the remote bitfield with missing pieces.
    Interested,
    NotInterested,

    // Controls whether the remote peer is allowed to request data from us.
    // Used for upload bandwidth management and peer selection (tit-for-tat).
    Choke,
    Unchoke,

    Have(Have), // Sent when completing a piece

    Request(PeerRequestMessage), // Request a piece
    Piece(PeerPieceMessage), // Send a piece
    Cancel(Cancel), // Cancel request
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerHandshake {
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20], // Sender's own ID
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bitfield {
    pub pieces: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerRequestMessage {
    pub piece_index: u32,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerPieceMessage {
    pub piece_index: u32,
    pub offset: u32,
    pub data: Bytes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Have {
    pub piece_index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cancel {
    pub piece_index: u32,
    pub offset: u32,
    pub length: u32,
}
