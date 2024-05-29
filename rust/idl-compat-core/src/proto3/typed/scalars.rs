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
use trait_gen::trait_gen;

pub trait Scalar:
    untyped::ToScalar + typed::FieldType + typed::MapValue
{
}
pub trait Integer: untyped::ToInteger + Scalar + typed::MapKey {}

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

impl untyped::ToInteger for Bool {
    const RESULT: untyped::Integer = untyped::Integer::Bool;
}

impl untyped::ToInteger for Int32 {
    const RESULT: untyped::Integer = untyped::Integer::Int32;
}

impl untyped::ToInteger for Int64 {
    const RESULT: untyped::Integer = untyped::Integer::Int64;
}

impl untyped::ToInteger for UInt32 {
    const RESULT: untyped::Integer = untyped::Integer::UInt32;
}

impl untyped::ToInteger for UInt64 {
    const RESULT: untyped::Integer = untyped::Integer::UInt64;
}

impl untyped::ToInteger for SInt32 {
    const RESULT: untyped::Integer = untyped::Integer::SInt32;
}

impl untyped::ToInteger for SInt64 {
    const RESULT: untyped::Integer = untyped::Integer::SInt64;
}

impl untyped::ToInteger for Fixed32 {
    const RESULT: untyped::Integer = untyped::Integer::Fixed32;
}

impl untyped::ToInteger for Fixed64 {
    const RESULT: untyped::Integer = untyped::Integer::Fixed64;
}

impl untyped::ToInteger for SFixed32 {
    const RESULT: untyped::Integer = untyped::Integer::SFixed32;
}

impl untyped::ToInteger for SFixed64 {
    const RESULT: untyped::Integer = untyped::Integer::SFixed64;
}

impl untyped::ToScalar for String {
    const RESULT: untyped::Scalar = untyped::Scalar::String;
}

impl untyped::ToScalar for Double {
    const RESULT: untyped::Scalar = untyped::Scalar::Double;
}

impl untyped::ToScalar for Float {
    const RESULT: untyped::Scalar = untyped::Scalar::Float;
}

impl untyped::ToScalar for Bytes {
    const RESULT: untyped::Scalar = untyped::Scalar::Bytes;
}

#[trait_gen(T ->
    Bool, Int32, Int64, UInt32, UInt64, SInt32, SInt64,
    Fixed32, Fixed64, SFixed32, SFixed64
)]
impl untyped::ToScalar for T {
    const RESULT: untyped::Scalar =
        untyped::Scalar::Integer(<T as untyped::ToInteger>::RESULT);
}

#[trait_gen(T -> Scalar)]
#[trait_gen(S -> String, Double, Float, Bytes)]
impl T for S {}

#[trait_gen(T -> Scalar, Integer)]
#[trait_gen(S ->
    Bool, Int32, Int64, UInt32, UInt64, SInt32, SInt64,
    Fixed32, Fixed64, SFixed32, SFixed64
)]
impl T for S {}

#[trait_gen(T ->
    Bool, Int32, Int64, UInt32, UInt64, SInt32, SInt64,
    Fixed32, Fixed64, SFixed32, SFixed64
)]
impl typed::MapKey for T {
    const UNTYPED_REPR: untyped::MapKey =
        untyped::MapKey::Integer(<T as untyped::ToInteger>::RESULT);
}

impl typed::MapKey for String {
    const UNTYPED_REPR: untyped::MapKey = untyped::MapKey::String;
}

#[trait_gen(T -> String, Double, Float, Bytes)]
impl typed::MapValue for T {
    const UNTYPED_REPR: untyped::MapValue =
        untyped::MapValue::Scalar(<T as untyped::ToScalar>::RESULT);
}

#[trait_gen(T ->
    Bool, Int32, Int64, UInt32, UInt64, SInt32, SInt64,
    Fixed32, Fixed64, SFixed32, SFixed64
)]
impl typed::MapValue for T {
    const UNTYPED_REPR: untyped::MapValue = untyped::MapValue::Scalar(
        untyped::Scalar::Integer(<T as untyped::ToInteger>::RESULT),
    );
}

#[trait_gen(T ->
    String, Double, Float, Bytes, Bool,
    Int32, Int64, UInt32, UInt64, SInt32, SInt64,
    Fixed32, Fixed64, SFixed32, SFixed64
)]
impl typed::FieldType for T {
    const UNTYPED_REPR: untyped::FieldType =
        untyped::FieldType::Scalar(<T as untyped::ToScalar>::RESULT);
}
