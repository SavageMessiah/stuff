module Main where

import Control.Applicative.Backwards (Backwards (forwards))
import Control.Concurrent (yield)
import Control.Monad (guard)
import Data.Foldable (traverse_)
import Data.List (group, sort)
import GHC.IO (unsafePerformIO)

loadWords :: String -> IO [String]
loadWords path = words <$> readFile path

distinctChars :: String -> Int
distinctChars = length . group . sort

countByDistinct :: [String] -> [(Int, Int)]
countByDistinct = map withCount . group . sort . map distinctChars
 where
  withCount group = (head group, length group)

ch7ex27 words = do
  traverse_ print $ countByDistinct words

oneOff :: Char -> Char -> Bool
oneOff a b = b == succ a || b == pred a

pairsWithDiff :: [String] -> [(String, String)]
pairsWithDiff words = do
  x <- words
  y <- words
  guard $ oneOffEveryComp x y
  return (x, y)
 where
  oneOffEveryComp x y = all (uncurry oneOff) $ zip x y

ch7ex28 words = do
  let pairs = pairsWithDiff words
  traverse_ print pairs

data Reverse = Palindrome String | Mirror String String
  deriving (Show)

reverseWords :: [String] -> [Reverse]
reverseWords words = do
  forwards <- words
  backwards <- words
  guard $ forwards == reverse backwards
  case (forwards, backwards) of
    (f, b) | f == b -> pure $ Palindrome f
    (f, b) -> pure $ Mirror f b

ch7ex29 :: [String] -> IO ()
ch7ex29 words = traverse_ print $ reverseWords words

main = do
  words <- loadWords "../sgb-words.txt"
  ch7ex27 words
  ch7ex28 words
  ch7ex29 words
