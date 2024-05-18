module Main where

import Prelude

import Effect (Effect)
import Effect.Console (log)

flip :: forall a b c. (a -> b -> c) -> b -> a -> c
flip f b a = f a b

main :: Effect Unit
main = do
  log "🍝"
