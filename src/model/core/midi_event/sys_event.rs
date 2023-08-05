#[derive(Debug, Clone)]
pub struct SysEvent;

impl From<SysEvent> for Vec<u8> {
  fn from(value: SysEvent) -> Self {
    todo!()
  }
}