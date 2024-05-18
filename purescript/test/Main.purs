module Test.Main where

import Prelude hiding (flip)

import Effect (Effect)
import Effect.Class.Console (log)

import Main (flip)

main :: Effect Unit
main = do
  log (show (flip const 1 2))