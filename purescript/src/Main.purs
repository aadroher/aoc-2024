module Main
  ( parseCode
  )
  where

import Prelude

import Control.Monad.Except (Except, runExcept, throwError)
import Control.Monad.List.Trans (filter)
import Data.Array as Array
import Data.Either (Either)
import Data.Int as Int
import Data.Maybe (Maybe(Just))
import Data.String (fromCodePointArray, toCodePointArray)
import Data.String.Utils as StringUtils
import Effect (Effect)
import Effect.Console (log)
import Effect.Exception (Error)

parseCode :: String -> Except Error Int
parseCode c = 
  case digitPair of
    {firstDigit: (Just n), lastDigit: (Just m)} -> pure $ n * 10 + m
    _ -> throwError "No digits found"
  where
    digits = Int.fromString <$> StringUtils.toCharArray $ StringUtils.filter (\d -> d >= "0" && d <= "9") c
    firstDigit = Array.head digits
    lastDigit = Array.last digits
    digitPair = {firstDigit, lastDigit}
day1problem1 :: Array String -> Int
day1problem1 _ = 0

main :: Effect Unit
main = do
  log "🍝"