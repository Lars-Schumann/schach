use core::ops::Add;
use core::ops::Not;
use core::ops::Mul;

use crate::board::COL_COUNT;
use crate::board::ROW_COUNT;
use self::Square as S;
use self::Col as C;
use self::Row as R;

#[derive(Debug, Copy,  Hash)]
#[derive_const(Clone, PartialEq, Eq)]
pub struct Square {
    pub col: Col,
    pub row: Row,
}

const fn s(col: Col, row: Row) -> Square{
    Square::new(col, row)
}

#[allow(clippy::use_self)]
impl Square {
    #[must_use]
    pub const fn new(col: Col, row: Row) -> Self {
        Self { col, row }
    }
    #[must_use]
    pub const fn is_black(self) -> bool {
        (u8::from(self.col) + u8::from(self.row)).is_multiple_of(2)
    }
    #[must_use]
    pub const fn is_white(self) -> bool {
        self.is_black().not()
    }

    pub const A1: S = s(C::_1, R::_1);
    pub const A2: S = s(C::_1, R::_2);
    pub const A3: S = s(C::_1, R::_3);
    pub const A4: S = s(C::_1, R::_4);
    pub const A5: S = s(C::_1, R::_5);
    pub const A6: S = s(C::_1, R::_6);
    pub const A7: S = s(C::_1, R::_7);
    pub const A8: S = s(C::_1, R::_8);

    pub const B1: S = s(C::_2, R::_1);
    pub const B2: S = s(C::_2, R::_2);
    pub const B3: S = s(C::_2, R::_3);
    pub const B4: S = s(C::_2, R::_4);
    pub const B5: S = s(C::_2, R::_5);
    pub const B6: S = s(C::_2, R::_6);
    pub const B7: S = s(C::_2, R::_7);
    pub const B8: S = s(C::_2, R::_8);

    pub const C1: S = s(C::_3, R::_1);
    pub const C2: S = s(C::_3, R::_2);
    pub const C3: S = s(C::_3, R::_3);
    pub const C4: S = s(C::_3, R::_4);
    pub const C5: S = s(C::_3, R::_5);
    pub const C6: S = s(C::_3, R::_6);
    pub const C7: S = s(C::_3, R::_7);
    pub const C8: S = s(C::_3, R::_8);

    pub const D1: S = s(C::_4, R::_1);
    pub const D2: S = s(C::_4, R::_2);
    pub const D3: S = s(C::_4, R::_3);
    pub const D4: S = s(C::_4, R::_4);
    pub const D5: S = s(C::_4, R::_5);
    pub const D6: S = s(C::_4, R::_6);
    pub const D7: S = s(C::_4, R::_7);
    pub const D8: S = s(C::_4, R::_8);

    pub const E1: S = s(C::_5, R::_1);
    pub const E2: S = s(C::_5, R::_2);
    pub const E3: S = s(C::_5, R::_3);
    pub const E4: S = s(C::_5, R::_4);
    pub const E5: S = s(C::_5, R::_5);
    pub const E6: S = s(C::_5, R::_6);
    pub const E7: S = s(C::_5, R::_7);
    pub const E8: S = s(C::_5, R::_8);

    pub const F1: S = s(C::_6, R::_1);
    pub const F2: S = s(C::_6, R::_2);
    pub const F3: S = s(C::_6, R::_3);
    pub const F4: S = s(C::_6, R::_4);
    pub const F5: S = s(C::_6, R::_5);
    pub const F6: S = s(C::_6, R::_6);
    pub const F7: S = s(C::_6, R::_7);
    pub const F8: S = s(C::_6, R::_8);

    pub const G1: S = s(C::_7, R::_1);
    pub const G2: S = s(C::_7, R::_2);
    pub const G3: S = s(C::_7, R::_3);
    pub const G4: S = s(C::_7, R::_4);
    pub const G5: S = s(C::_7, R::_5);
    pub const G6: S = s(C::_7, R::_6);
    pub const G7: S = s(C::_7, R::_7);
    pub const G8: S = s(C::_7, R::_8);

    pub const H1: S = s(C::_8, R::_1);
    pub const H2: S = s(C::_8, R::_2);
    pub const H3: S = s(C::_8, R::_3);
    pub const H4: S = s(C::_8, R::_4);
    pub const H5: S = s(C::_8, R::_5);
    pub const H6: S = s(C::_8, R::_6);
    pub const H7: S = s(C::_8, R::_7);
    pub const H8: S = s(C::_8, R::_8);
    
    #[rustfmt::skip]
    pub const ALL: [Self; 64] = [
        S::A8, S::B8, S::C8, S::D8, S::E8, S::F8, S::G8, S::H8,
        S::A7, S::B7, S::C7, S::D7, S::E7, S::F7, S::G7, S::H7,
        S::A6, S::B6, S::C6, S::D6, S::E6, S::F6, S::G6, S::H6,
        S::A5, S::B5, S::C5, S::D5, S::E5, S::F5, S::G5, S::H5,
        S::A4, S::B4, S::C4, S::D4, S::E4, S::F4, S::G4, S::H4,
        S::A3, S::B3, S::C3, S::D3, S::E3, S::F3, S::G3, S::H3,
        S::A2, S::B2, S::C2, S::D2, S::E2, S::F2, S::G2, S::H2,
        S::A1, S::B1, S::C1, S::D1, S::E1, S::F1, S::G1, S::H1,
    ];
}

#[derive_const(PartialEq, Eq, PartialOrd, Ord, Clone)]
#[derive(Debug, Copy, Hash)]
pub enum Col {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}
impl Col {
    pub const ALL: [Self; COL_COUNT] = [
        Self::_1,
        Self::_2,
        Self::_3,
        Self::_4,
        Self::_5,
        Self::_6,
        Self::_7,
        Self::_8,
    ];
}

#[derive_const(PartialEq, Eq, PartialOrd, Ord, Clone)]
#[derive(Debug, Copy, Hash)]
pub enum Row {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}
impl Row {
    pub const ALL: [Self; ROW_COUNT] = [
        Self::_1,
        Self::_2,
        Self::_3,
        Self::_4,
        Self::_5,
        Self::_6,
        Self::_7,
        Self::_8,
    ];
}
impl const Add<i32> for Row {
    type Output = Result<Self, RowIndexOutOfRange>;
    fn add(self, rhs: i32) -> Self::Output {
        let row_number: i32 = self.into();
        (row_number + rhs).try_into()
    }
}
impl const Add<i32> for Col {
    type Output = Result<Self, ColIndexOutOfRange>;
    fn add(self, rhs: i32) -> Self::Output {
        let column_number: i32 = self.into();
        (column_number + rhs).try_into()
    }
}

macro_rules! col_into_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const From<Col> for $ty {
                fn from(value: Col) -> $ty {
                    match value {
                        Col::_1 => 1,
                        Col::_2 => 2,
                        Col::_3 => 3,
                        Col::_4 => 4,
                        Col::_5 => 5,
                        Col::_6 => 6,
                        Col::_7 => 7,
                        Col::_8 => 8,
                    }
                }
            }
        )*
    }
}

macro_rules! row_into_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const From<Row> for $ty {
                fn from(value: Row) -> $ty {
                    match value {
                        Row::_1 => 1,
                        Row::_2 => 2,
                        Row::_3 => 3,
                        Row::_4 => 4,
                        Row::_5 => 5,
                        Row::_6 => 6,
                        Row::_7 => 7,
                        Row::_8 => 8,
                    }
                }
            }
        )*
    }
}

#[derive(Debug)]
pub enum ColIndexOutOfRange {
    TooLow,
    TooHigh,
}

#[derive(Debug)]
pub enum RowIndexOutOfRange {
    TooLow,
    TooHigh,
}

macro_rules! col_try_from_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const TryFrom<$ty> for Col {
                type Error = ColIndexOutOfRange;
                fn try_from(value: $ty) -> Result<Self, Self::Error> {
                    match value {
                        ..=0 => Err(ColIndexOutOfRange::TooLow),
                         1 => Ok(Self::_1),
                         2 => Ok(Self::_2),
                         3 => Ok(Self::_3),
                         4 => Ok(Self::_4),
                         5 => Ok(Self::_5),
                         6 => Ok(Self::_6),
                         7 => Ok(Self::_7),
                         8 => Ok(Self::_8),
                         9.. =>  Err(ColIndexOutOfRange::TooHigh),

                    }
                }
            }
        )*
    }
}

macro_rules! row_try_from_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const TryFrom<$ty> for Row {
                type Error = RowIndexOutOfRange;
                fn try_from(value: $ty) -> Result<Self, Self::Error> {
                    match value {
                        ..=0 =>  Err(RowIndexOutOfRange::TooLow),
                         1 => Ok(Self::_1),
                         2 => Ok(Self::_2),
                         3 => Ok(Self::_3),
                         4 => Ok(Self::_4),
                         5 => Ok(Self::_5),
                         6 => Ok(Self::_6),
                         7 => Ok(Self::_7),
                         8 => Ok(Self::_8),
                         9.. =>  Err(RowIndexOutOfRange::TooHigh),
                    }
                }
            }
        )*
    }
}

col_into_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
row_into_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

col_try_from_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
row_try_from_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Offset {
    pub(crate) col: i32,
    pub(crate) row: i32,
}
#[allow(clippy::upper_case_acronyms)]
impl Offset {
    pub(crate) const U: Self = Self { col: 0, row: 1 };
    pub(crate) const D: Self = Self::U * -1;
    const R: Self = Self { col: 1, row: 0 };
    const L: Self = Self::R * -1;
    const UL: Self = Self::U + Self::L;
    const UR: Self = Self::U + Self::R;
    const DL: Self = Self::D + Self::L;
    const DR: Self = Self::D + Self::R;
    const UUL: Self = Self::U + Self::UL;
    const UUR: Self = Self::U + Self::UR;
    const ULL: Self = Self::UL + Self::L;
    const URR: Self = Self::UR + Self::R;
    const DDL: Self = Self::D + Self::DL;
    const DDR: Self = Self::D + Self::DR;
    const DLL: Self = Self::DL + Self::L;
    const DRR: Self = Self::DR + Self::R;

    pub(crate) const ROOK: [Self; 4] = [Self::U, Self::D, Self::L, Self::R];
    pub(crate) const BISHOP: [Self; 4] = [Self::UL, Self::UR, Self::DL, Self::DR];
    pub(crate) const QUEEN: [Self; 8] = [
        Self::U,
        Self::D,
        Self::L,
        Self::R,
        Self::UL,
        Self::UR,
        Self::DL,
        Self::DR,
    ];
    pub(crate) const KING_DIRECT: [Self; 8] = Self::QUEEN;

    pub(crate) const PAWN_UP_DIAGONAL: [Self; 2] = [Self::UL, Self::UR];
    pub(crate) const PAWN_DOWN_DIAGONAL: [Self; 2] = [Self::DL, Self::DR];

    pub(crate) const KNIGHT: [Self; 8] = [
        Self::UUL,
        Self::UUR,
        Self::ULL,
        Self::URR,
        Self::DDL,
        Self::DDR,
        Self::DLL,
        Self::DRR,
    ];
}

impl const Add for Offset {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}
impl const Mul<i32> for Offset {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            col: self.col * rhs,
            row: self.row * rhs,
        }
    }
}

#[derive(Debug)]
pub enum SquareOutOfRange {
    Col(ColIndexOutOfRange),
    Row(RowIndexOutOfRange),
}
impl const From<ColIndexOutOfRange> for SquareOutOfRange {
    fn from(value: ColIndexOutOfRange) -> Self {
        Self::Col(value)
    }
}
impl const From<RowIndexOutOfRange> for SquareOutOfRange {
    fn from(value: RowIndexOutOfRange) -> Self {
        Self::Row(value)
    }
}
impl const Add<Offset> for Square {
    type Output = Result<Self, SquareOutOfRange>;
    fn add(self, rhs: Offset) -> Self::Output {
        Ok(Self {
            col: (self.col + rhs.col)?,
            row: (self.row + rhs.row)?,
        })
    }
}
