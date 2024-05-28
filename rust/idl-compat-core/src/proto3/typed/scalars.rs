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

use crate::proto3::typed;
use trait_gen::trait_gen;

pub trait Scalar: typed::MapValue {}
pub trait Integer: Scalar + typed::MapKey {}

pub struct String;
pub struct Double;
pub struct Float;
pub struct Bytes;

pub struct Bool;
pub struct Int32;
pub struct Int64;
pub struct UInt32;
pub struct UInt64;
pub struct SInt32;
pub struct SInt64;
pub struct Fixed32;
pub struct Fixed64;
pub struct SFixed32;
pub struct SFixed64;

#[trait_gen(T -> typed::MapKey, typed::MapValue, Scalar, Integer)]
#[trait_gen(S ->
    Bool, Int32, Int64, UInt32, UInt64, SInt32, SInt64,
    Fixed32, Fixed64, SFixed32, SFixed64
)]
impl T for S {}

impl typed::MapKey for String {}

#[trait_gen(T -> typed::MapValue, Scalar)]
#[trait_gen(S -> String, Double, Float, Bytes)]
impl T for S {}
