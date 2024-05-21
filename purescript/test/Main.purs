module Test.Main where

import Prelude hiding (flip, const)

import Effect (Effect)
import Effect.Aff (delay, launchAff_)
import Effect.Class.Console (log)
import Main (day1problem1)
import Test.Spec (describe, it, pending)
import Test.Spec.Assertions (shouldEqual)
import Test.Spec.Reporter.Console (consoleReporter)
import Test.Spec.Runner (runSpec)



main :: Effect Unit
main = launchAff_ $ runSpec [consoleReporter] do
  describe "day 1" do
    describe "problem 1" do
      it "works for the example" do
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
        let expected = 142
        day1problem1 input `shouldEqual` expected