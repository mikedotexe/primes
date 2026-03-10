{-# OPTIONS --safe #-}

module Specs.Tests where

open import Agda.Builtin.Bool using (Bool; true; false)
open import Data.Bool.Base    using (if_then_else_)
open import Specs.SpacingResidueModel
open import Specs.PalindromeEvenDivides

-- All should normalize to 'true'
All : Bool
All = if Specs.SpacingResidueModel.Test₁ then if Specs.SpacingResidueModel.Test₂ then if Specs.SpacingResidueModel.Test₃ then if Specs.PalindromeEvenDivides.Test₁ then Specs.PalindromeEvenDivides.Test₂ else false else false else false else false
