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

use crate::proto3::untyped::enums::EnumMessage;
use crate::proto3::untyped::maps::{MapKeyType, MapValueType};
use crate::proto3::untyped::scalars::Scalar;

#[derive(Eq, Hash, PartialEq)]
pub enum FieldType {
    Any,
    EnumMessage(&'static EnumMessage),
    Map(MapKeyType, MapValueType),
    Message(&'static Message),
    Scalar(Scalar),
}

#[derive(Eq, Hash, PartialEq)]
pub enum FieldModifier {
    OneOf(&'static [u32]), //List of fields included in oneof
    Optional,
    OptionalRepeated,
    Repeated,
}

#[derive(Eq, Hash, PartialEq)]
pub struct Field {
    name: &'static str,
    id: u32,
    field_type: FieldType,
    field_mod: FieldModifier,
}

#[derive(Eq, Hash, PartialEq)]
pub struct Message {
    name: &'static str,
    fields: &'static [Field],
}
