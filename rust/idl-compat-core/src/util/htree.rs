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
use typenum::{Cmp, Compare, Ord, Unsigned};

pub trait HField {
    type ID: Unsigned;
}

pub trait HTree {}

pub trait HTreeUnpack<ID, NodeT>
where
    ID: Unsigned,
    NodeT: HTree,
{
    type Output: HField;
}

pub type HTreeUnpackOp<ID, NodeT> = <() as HTreeUnpack<ID, NodeT>>::Output;

pub struct HNil;

pub struct HNode<H, L, R>
where
    H: HField,
    L: HTree,
    R: HTree,
{
    pub head: PhantomData<H>,
    pub lhs: PhantomData<L>,
    pub rhs: PhantomData<R>,
}

mod prv {
    use super::{HField, HNode, HTree, HTreeUnpack, HTreeUnpackOp};
    use typenum::{Cmp, Equal, Greater, Less, Ord, Unsigned};

    pub trait HTreeChoice<ID, H, L, R, OrdT>
    where
        ID: Unsigned,
        H: HField,
        L: HTree,
        R: HTree,
        OrdT: Ord,
    {
        type Output: HField;
    }

    pub type HTreeChoiceOp<ID, H, L, R, OrdT> = <() as HTreeChoice<ID, H, L, R, OrdT>>::Output;

    impl<ID, H, L, R> HTreeChoice<ID, H, L, R, Equal> for ()
    where
        R: HTree,
        HNode<H, L, R>: HTree,
        H: HField,
        ID: Cmp<<H as HField>::ID> + Unsigned,
        L: HTree,
    {
        type Output = H;
    }

    impl<ID, H, L, R> HTreeChoice<ID, H, L, R, Less> for ()
    where
        R: HTree,
        HNode<H, L, R>: HTree,
        (): HTreeUnpack<ID, L>,
        H: HField,
        ID: Cmp<<H as HField>::ID> + Unsigned,
        L: HTree,
    {
        type Output = HTreeUnpackOp<ID, L>;
    }

    impl<ID, H, L, R> HTreeChoice<ID, H, L, R, Greater> for ()
    where
        R: HTree,
        HNode<H, L, R>: HTree,
        (): HTreeUnpack<ID, R>,
        H: HField,
        ID: Cmp<<H as HField>::ID> + Unsigned,
        L: HTree,
    {
        type Output = HTreeUnpackOp<ID, R>;
    }
}

impl HTree for HNil {}

impl<H, L, R> HTree for HNode<H, L, R>
where
    H: HField,
    L: HTree,
    R: HTree,
{
}

impl<ID, H, L, R> HTreeUnpack<ID, HNode<H, L, R>> for ()
where
    R: HTree,
    HNode<H, L, R>: HTree,
    (): prv::HTreeChoice<ID, H, L, R, Compare<ID, <H as HField>::ID>>,
    H: HField,
    ID: Cmp<<H as HField>::ID> + Unsigned,
    L: HTree,
    Compare<ID, <H as HField>::ID>: Ord,
{
    type Output = prv::HTreeChoiceOp<ID, H, L, R, Compare<ID, <H as HField>::ID>>;
}
