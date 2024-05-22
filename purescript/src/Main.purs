module Main
  ( parseCode
  )
  where

import Prelude

import Control.Monad.Except (Except, runExcept, throwError)
import Control.Monad.List.Trans (filter)
import Data.Array (head, last)
import Data.Either (Either)
import Data.Int as Int
import Effect (Effect)
import Effect.Console (log)
import Effect.Exception (Error)

parseCode :: String -> Except Error Int
parseCode c = 
  case digitPair of
    (Just n, Just m) -> pure $ first * 10 + last
    _ -> throwError "No digits found"
  where
    digits = Int.fromString <$> filter (\d -> d >= '0' && d <= '9') c
    firstDigit = head digits
    lastDigit = last digits
    digitPair = (firstDigit, lastDigit)
day1problem1 :: Array String -> Int
day1problem1 _ = 0

main :: Effect Unit
main = do
  log "🍝"