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
use core::marker::PhantomData;

pub trait MapKey {
    const UNTYPED_REPR: untyped::MapKey;
}

// Implemented for each type used in a map.
// - N unique types : N impls.
// - Const context.
pub trait MapValue {
    const UNTYPED_REPR: untyped::MapValue;
}

pub struct Map<TKey, TValue>
where
    TKey: MapKey,
    TValue: MapValue,
{
    pub key: PhantomData<TKey>,
    pub value: PhantomData<TValue>,
}

impl<TKey, TValue> typed::FieldType for Map<TKey, TValue>
where
    TKey: MapKey,
    TValue: MapValue,
{
    const UNTYPED_REPR: untyped::FieldType = untyped::FieldType::Map(
        <TKey as MapKey>::UNTYPED_REPR,
        <TValue as MapValue>::UNTYPED_REPR,
    );
}
