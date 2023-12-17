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
// Attribution-ShareAlike 4.0 International License. This only includes any
// documentation comments or `#[doc]` attributes present in this file unless
// explicitly indicated otherwise.
//
// You should have received a copy of the Creative Commons
// Attribution-ShareAlike 4.0 International License along with the IDL-Compat
// Documentation. If not, see <https://creativecommons.org/licenses/by-sa/4.0/>. 

#[derive(Eq, Hash, PartialEq)]
pub enum Integer {
    Bool,
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64
}

#[derive(Eq, Hash, PartialEq)]
pub enum Scalar {
    String,
    Integer(Integer),
    Double,
    Float,
    Bytes
}
