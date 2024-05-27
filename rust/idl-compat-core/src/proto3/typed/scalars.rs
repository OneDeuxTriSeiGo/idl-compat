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

pub trait Scalar {}

pub trait Integer: Scalar {}

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

impl Scalar for String {}
impl Scalar for Double {}
impl Scalar for Float {}
impl Scalar for Bytes {}
impl Scalar for Bool {}
impl Scalar for Int32 {}
impl Scalar for Int64 {}
impl Scalar for UInt32 {}
impl Scalar for UInt64 {}
impl Scalar for SInt32 {}
impl Scalar for SInt64 {}
impl Scalar for Fixed32 {}
impl Scalar for Fixed64 {}
impl Scalar for SFixed32 {}
impl Scalar for SFixed64 {}

impl Integer for Bool {}
impl Integer for Int32 {}
impl Integer for Int64 {}
impl Integer for UInt32 {}
impl Integer for UInt64 {}
impl Integer for SInt32 {}
impl Integer for SInt64 {}
impl Integer for Fixed32 {}
impl Integer for Fixed64 {}
impl Integer for SFixed32 {}
impl Integer for SFixed64 {}
