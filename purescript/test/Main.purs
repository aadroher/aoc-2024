module Test.Main where

import Prelude hiding (flip, const)

import Effect (Effect)
import Effect.Class.Console (log)
import Main (pow)

main :: Effect Unit
main = do
  log "Hello, World!"
  log $ show $ pow 2 2147483647