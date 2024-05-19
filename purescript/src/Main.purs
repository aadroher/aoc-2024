module Main where

import Prelude

import Effect (Effect)
import Effect.Console (log)

pow :: Int -> Int -> Int
pow _ 0 = 1 
pow n m = n * pow n (m - 1)

pow' :: Int -> Int -> Int
pow' _ 0 = 1
pow' n m = pow' n (m - 1) * n
main :: Effect Unit
main = do
  log "🍝"
