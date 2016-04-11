module Main where

-- common GHC 7.10.2
import Control.Monad
import Control.Monad.Trans
import Control.Monad.Trans.RWS -- strict or lazy? unknown and undecided
import Data.Char
import System.IO

--TOFU:=: add a menu
main = do
    -- Run quest
    runRWST (RWST initialise >>= play) "Environment" "State"
    return ()

initialise = \r s -> return ("", s, mempty)

-- temporary type declaration
play :: String -> RWST String String String IO String
--play :: Game -> RWST Environment Log State IO Game
play = \w -> do
    case w of
        "quit" -> return w
        _      -> fetch >>= parse >>= execute >>= observe >>= play
              
--TOFU:=: separate modules and implement
--fetch :: RWST Environment Log State IO String
fetch = liftIO getLine

--parse :: String -> RWST Environment Log State IO Action
parse = return

--execute :: Action -> RWST Environment Log State IO Result
execute = return
    
--observe :: Result -> RWST Environment Log State IO Game
observe = return
    