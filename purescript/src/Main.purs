module Main where

import Prelude

import Control.Monad.List.Trans (filter)
import Data.Array (head, last)
import Data.Either (Either)
import Data.Int as Int
import Effect (Effect)
import Effect.Console (log)

parseCode :: String -> Either Int
parseCode c = 
  firstDigit
  where
    digits = Int.fromString <$> filter (\d -> d >= '0' && d <= '9') c
    firstDigit = head digits
    lastDigit = last digits
day1problem1 :: Array String -> Int
day1problem1 _ = 0

main :: Effect Unit
main = do
  log "🍝"