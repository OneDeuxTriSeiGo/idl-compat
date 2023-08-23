// SPDX-FileCopyrightText: 2023 Jacob Abel <jacobabel@nullpo.dev>
//
// SPDX-License-Identifier: LGPL-3.0-or-later AND CC-BY-SA-4.0
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
//
// IDL-Compat Documentation is licensed under the Creative Commons
// Attribution-ShareAlike 4.0 International License. This includes any
// documentation comments or `#[doc]` attributes present in this file.
//
// You should have received a copy of the Creative Commons
// Attribution-ShareAlike 4.0 International License along with the IDL-Compat
// Documentation. If not, see <https://creativecommons.org/licenses/by-sa/4.0/>. 

use crate::proto3::untyped;

// Implemented by each Protobuf Message
// - N fields : N impls. Same impl/over as Field.
// - Const context.
pub trait FieldType {
    type TValue: 'static; // Any(), EnumMsg, Map, Msg, or Scalar

    const UNTYPED_REPR: &'static untyped::FieldType;
    const VALUE: &'static Self::TValue;
}

// Implemented by each Protobuf Message
// - N messages with M fields each : NxM impls over (type, id) tuple
// - - OR NxM impls by struct(Tmsg, Tfield, id).
// - Const context.
pub trait Field {
    type TFieldType: 'static; //FieldType impl
    const ID: u32;

    const UNTYPED_REPR: &'static untyped::Field;
    const FIELD_TYPE: &'static Self::TFieldType;
}

// Implemented by each Protobuf Message
// - N messages : N impls
// - Const context.
pub trait Message {
    type TFields: 'static; // HList/right tuple (T,(U,(V,(W,(X,(Y,Z))))))

    const UNTYPED_REPR: &'static untyped::Message;
    const FIELDS: &'static Self::TFields;
}
