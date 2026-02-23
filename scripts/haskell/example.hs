import System.Environment (getArgs)

main :: IO ()
main = getArgs >>= \args -> putStrLn $ "[Haskell] args: " ++ show args
