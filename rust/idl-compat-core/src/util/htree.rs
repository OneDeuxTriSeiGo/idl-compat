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

use core::marker::PhantomData;
use typenum::{Cmp, Compare, Equal, Greater, Less, Unsigned};

pub trait Field {
    type ID;
}

pub trait HTree {}

pub trait HTreeChoice<ID, H, L, R, OrdT> {
    type Output;
}

pub trait HTreeUnpack<ID, NodeT>
where
    ID: Unsigned,
    NodeT: HTree,
{
    type Output;
}

pub type HTreeChoiceOp<ID, H, L, R, OrdT> = <() as HTreeChoice<ID, H, L, R, OrdT>>::Output;
pub type HTreeUnpackOp<ID, NodeT> = <() as HTreeUnpack<ID, NodeT>>::Output;

pub struct HNil;

pub struct HNode<H, L, R>
where
    H: Field,
    L: HTree,
    R: HTree,
{
    pub head: PhantomData<H>,
    pub lhs: PhantomData<L>,
    pub rhs: PhantomData<R>,
}

impl HTree for HNil {}

impl<H: Field, L: HTree, R: HTree> HTree for HNode<H, L, R> {}

impl<ID, H, L, R> HTreeChoice<ID, H, L, R, Equal> for ()
where
    R: HTree,
    HNode<H, L, R>: HTree,
    H: Field,
    ID: Cmp<<H as Field>::ID> + Unsigned,
    L: HTree,
{
    type Output = H;
}

impl<ID, H, L, R> HTreeChoice<ID, H, L, R, Less> for ()
where
    R: HTree,
    HNode<H, L, R>: HTree,
    (): HTreeUnpack<ID, L>,
    H: Field,
    ID: Cmp<<H as Field>::ID> + Unsigned,
    L: HTree,
{
    type Output = <() as HTreeUnpack<ID, L>>::Output;
}

impl<ID, H, L, R> HTreeChoice<ID, H, L, R, Greater> for ()
where
    R: HTree,
    HNode<H, L, R>: HTree,
    (): HTreeUnpack<ID, R>,
    H: Field,
    ID: Cmp<<H as Field>::ID> + Unsigned,
    L: HTree,
{
    type Output = <() as HTreeUnpack<ID, R>>::Output;
}

impl<ID, H, L, R> HTreeUnpack<ID, HNode<H, L, R>> for ()
where
    R: HTree,
    HNode<H, L, R>: HTree,
    (): HTreeChoice<ID, H, L, R, Compare<ID, <H as Field>::ID>>,
    H: Field,
    ID: Cmp<<H as Field>::ID> + Unsigned,
    L: HTree,
{
    type Output = HTreeChoiceOp<ID, H, L, R, Compare<ID, <H as Field>::ID>>;
}
