{-# OPTIONS --safe --without-K #-}
------------------------------------------------------------------------
-- Bounded-k transfer witnesses
--
-- Maintained wrapper:
-- 1. the exact Agda witness vocabulary lives in the shell module
-- 2. the curated witness catalog is generated from the Rust bounded-k lane
-- 3. downstream callers can keep importing this stable wrapper path
------------------------------------------------------------------------

module Examples.BoundedKTransferWitnesses where

open import Examples.BoundedKTransferWitnessShell public
open import Examples.Generated.BoundedKTransferWitnessCatalog public
