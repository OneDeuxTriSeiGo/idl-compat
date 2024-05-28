// SPDX-FileCopyrightText: 2023 Jacob Abel <jacobabel@nullpo.dev>
//
// SPDX-License-Identifier: LGPL-3.0-or-later
//
// This file is part of IDL-Compat.
//
// IDL-Compat is free software: you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// IDL-Compat is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with IDL-Compat. If not, see <https://www.gnu.org/licenses/>.

mod any;
mod enums;
mod maps;
mod messages;
mod scalars;

pub use any::Any;
pub use enums::EnumMessage;
pub use maps::{Map, MapKey, MapValue};
pub use messages::{Field, FieldType, Message};
pub use scalars::{
    Bool, Bytes, Double, Fixed32, Fixed64, Float, Int32, Int64, Integer, SFixed32, SFixed64,
    SInt32, SInt64, Scalar, String, UInt32, UInt64,
};
