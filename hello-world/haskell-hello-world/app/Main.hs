module Main where

import System.Environment

main :: IO ()
main = do
  args <- getArgs
  putStrLn $ "Hello " ++ if null args then "Nobody" else head args
