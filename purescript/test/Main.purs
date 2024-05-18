module Test.Main where

import Prelude hiding (flip, const)

import Effect (Effect)
import Effect.Class.Console (log)

import Main (flip, const)

main :: Effect Unit
main = do
  log $ show $ flip const 1 2