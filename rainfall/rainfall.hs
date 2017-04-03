#!/usr/bin/env runhaskell
module Main where

import Data.List

main = getContents >>= display . compute . clean . parse

parse = concatMap (winnow . reads) . lines where
    winnow [(d, "")] = [d]
    winnow _         = []

clean = filter (>= 0) . takeWhile (/= 999)

compute samples = (μ, below, above) where
  μ       = mean samples
  below   = length (filter (\d -> μ - 5 <= d && d < μ) samples)
  above   = length (filter (\d -> μ < d && d <= μ + 5) samples)
  mean ds = sum ds / genericLength ds

display (μ, below, above)
  | isNaN μ   = putStr "No measurements given.\n"
  | otherwise = putStr $ "Mean rainfall: " ++ show μ ++ "\n" ++
                         "Below count:   " ++ show below ++ "\n" ++
                         "Above count:   " ++ show above ++ "\n"

