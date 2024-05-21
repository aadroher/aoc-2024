module Main where

import Prelude

import Control.Monad.List.Trans (filter)
import Data.Either (Either)
import Effect (Effect)
import Effect.Console (log)
import Data.Char (digitToInt)

parseCode :: String -> Either Int
parseCode c = 
  firstDigit
  where
    digits = parseI filter (\d -> d >= '0' && d <= '9') c 
day1problem1 :: Array String -> Int
day1problem1 _ = 0

main :: Effect Unit
main = do
  log "🍝"
