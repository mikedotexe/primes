{-# OPTIONS --without-K --safe #-}

module Tests.Spec.ResidueClassesUnitsSpec where

open import Data.Nat using (_<_; z≤n; s≤s)
open import Relation.Binary.PropositionalEquality using (refl)

open import Core.Equiv using (to; from)
open import Core.ResidueClassesComplete using
  ( ResidueClass
  ; [_]mod_
  ; ⟦_⟧
  ; Coprime
  ; IsUnit
  ; units-are-coprime
  )

0<7 : 0 < 7
0<7 = s≤s z≤n

0<10 : 0 < 10
0<10 = s≤s z≤n

1<7 : 1 < 7
1<7 = s≤s (s≤s z≤n)

1<10 : 1 < 10
1<10 = s≤s (s≤s z≤n)

3<10 : 3 < 10
3<10 = s≤s (s≤s (s≤s (s≤s z≤n)))

5<7 : 5 < 7
5<7 = s≤s (s≤s (s≤s (s≤s (s≤s (s≤s z≤n)))))

A10 : ResidueClass 10 {0<10}
A10 = [ 3 ]mod 3<10

A7 : ResidueClass 7 {0<7}
A7 = [ 5 ]mod 5<7

coprime-A10 : Coprime ⟦ A10 ⟧ 10
coprime-A10 = refl

coprime-A7 : Coprime ⟦ A7 ⟧ 7
coprime-A7 = refl

unit-A10 : IsUnit A10
unit-A10 = from (units-are-coprime 1<10 A10) coprime-A10

unit-A7 : IsUnit A7
unit-A7 = from (units-are-coprime 1<7 A7) coprime-A7

coprime-A10-roundtrip : Coprime ⟦ A10 ⟧ 10
coprime-A10-roundtrip = to (units-are-coprime 1<10 A10) unit-A10

coprime-A7-roundtrip : Coprime ⟦ A7 ⟧ 7
coprime-A7-roundtrip = to (units-are-coprime 1<7 A7) unit-A7
