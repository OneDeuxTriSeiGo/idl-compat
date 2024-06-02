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

    pub type HTreeChoiceOp<ID, H, L, R, OrdT> =
        <() as HTreeChoice<ID, H, L, R, OrdT>>::Output;

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
    type Output =
        prv::HTreeChoiceOp<ID, H, L, R, Compare<ID, <H as HField>::ID>>;
}

#[cfg(test)]
mod tests {
    use super::{HField, HNil, HNode, HTree, HTreeUnpackOp};
    use typenum::{assert_type_eq, consts, Unsigned};

    trait Bar {
        type Val: HTree;
    }

    struct FieldA {}
    struct FieldB {}
    struct FieldC {}
    struct FieldD {}
    struct FieldE {}
    struct FieldF {}
    struct FieldG {}
    struct FieldH {}

    struct Foo {}

    trait FieldVal: HField {
        type IDV: Unsigned;
    }

    impl HField for FieldA {
        type ID = consts::U10;
    }
    impl HField for FieldB {
        type ID = consts::U2;
    }
    impl HField for FieldC {
        type ID = consts::U1;
    }
    impl HField for FieldD {
        type ID = consts::U3;
    }
    impl HField for FieldE {
        type ID = consts::U13;
    }
    impl HField for FieldF {
        type ID = consts::U11;
    }
    impl HField for FieldG {
        type ID = consts::U55;
    }
    impl HField for FieldH {
        type ID = consts::U54;
    }

    impl FieldVal for FieldF {
        type IDV = consts::U11;
    }

    impl Bar for Foo {
        type Val = HNode<
            FieldA,
            HNode<FieldB, HNode<FieldC, HNil, HNil>, HNode<FieldD, HNil, HNil>>,
            HNode<
                FieldE,
                HNode<FieldF, HNil, HNil>,
                HNode<FieldG, HNode<FieldH, HNil, HNil>, HNil>,
            >,
        >;
    }

    type Target = FieldF;
    type Val = <Foo as Bar>::Val;
    type Id = <Target as HField>::ID;

    // Supress False Positive
    #[allow(dead_code)]
    type Result = HTreeUnpackOp<Id, Val>;

    #[test]
    fn check_types_match() {
        assert_type_eq!(Target, Result);
    }

    #[test]
    fn check_unconstrained_traits_accessible() {
        assert_type_eq!(<Target as FieldVal>::IDV, <Result as FieldVal>::IDV);
    }
}
