-- if you've seen horrors

import Data.List (sort)
import qualified Data.Map.Strict as Map

solve2 :: Map.Map Int Int -> Int -> Int
solve2 counter n  = case Map.lookup n counter of
  Just x -> x * n
  Nothing -> 0

solve :: String -> (Int, Int)
solve contents =
  let rows = getLists contents
      lists = transpose rows
      sortedLists = ((sort $ fst lists), (sort $ snd lists))
      zippedPairs = zip (fst sortedLists) (snd sortedLists)
      sol1 = foldr (\(v1, v2) x -> x+(abs $ v1-v2)) 0 zippedPairs
      counter = foldr (\n countmap -> Map.insertWith (+) n 1 countmap) Map.empty $ snd lists
      sol2 = foldr (\x acc -> acc + solve2 counter x) 0 $ fst lists
  in
    (sol1, sol2)


transpose :: [(Int, Int)] -> ([Int], [Int])
transpose (head:[]) = ([fst head], [snd head])
transpose (head:tail) = let vals = transpose tail in
  ((fst head):(fst vals), (snd head):(snd vals))

getInts :: String -> (Int, Int)
getInts s = let doubleList = take 2 (map read $ words s) in
  case doubleList of
    x:y:_ -> (x,y)


getLists :: String -> [(Int, Int)]
getLists s = map getInts $ lines s


main :: IO ()
main = do
  contents <- readFile "input.txt"
  putStrLn $ show $ solve contents
