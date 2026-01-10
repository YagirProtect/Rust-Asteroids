#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ColLayer {
    Player = 0,
    Asteroid = 1,
    BulletPlayer = 2,
    BulletEnemy = 3,
}

impl ColLayer {
    #[inline]
    pub const fn idx(self) -> usize {
        self as usize
    }
}