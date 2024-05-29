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

use crate::proto3::{typed, untyped};
use crate::util::{HField, HTree};

// Implemented by each Protobuf Message
// - N fields : N impls. Same impl/over as Field.
// - Const context.
pub trait FieldType {
    type TValue; // Any(), EnumMsg, Map, Msg, or Scalar

    const UNTYPED_REPR: untyped::FieldType;
}

// Implemented by each Protobuf Message
// - N messages with M fields each : NxM impls over (type, id) tuple
// - - OR NxM impls by struct(Tmsg, Tfield, id).
// - Const context.
pub trait Field: HField {
    type TFieldType: FieldType; //FieldType impl

    const UNTYPED_REPR: untyped::Field;
}

// Implemented by each Protobuf Message
// - N messages : N impls
// - Const context.
pub trait Message: typed::MapValue {
    type TFields: HTree;

    const UNTYPED_REPR: untyped::Message;
}
