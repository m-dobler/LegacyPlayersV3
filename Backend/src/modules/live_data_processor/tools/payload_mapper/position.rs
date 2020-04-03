use crate::modules::live_data_processor::dto::{LiveDataProcessorFailure, Position};
use crate::modules::live_data_processor::tools::byte_reader;

pub trait MapPosition {
  fn to_position(&self) -> Result<Position, LiveDataProcessorFailure>;
}

impl MapPosition for [u8] {
  fn to_position(&self) -> Result<Position, LiveDataProcessorFailure> {
    Ok(Position {
      map_id: byte_reader::read_u32(&self[0..4])?,
      instance_id: byte_reader::read_u32(&self[4..8])?,
      map_difficulty: self[8],
      unit: byte_reader::read_u64(&self[9..17])?,
      x: byte_reader::read_i32(&self[17..21])?,
      y: byte_reader::read_i32(&self[21..25])?,
      z: byte_reader::read_i32(&self[25..29])?,
      orientation: byte_reader::read_i32(&self[29..33])?
    })
  }
}