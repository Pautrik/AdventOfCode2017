{-# LANGUAGE TemplateHaskell #-}

data Direction = PX | NX | PY | NY deriving (Eq, Show, Read)

type Coords = (Int, Int)
type StepSize = Integer
type StepsLeft = Integer
type BlockNr = Integer

genBlock :: Direction -> Coords -> StepSize -> StepsLeft -> BlockNr -> Coords
genBlock _ coords _ _ 277678 = coords

genBlock dir coords stepSize 0 blockNr =
    genBlock newDir (linearCoords coords newDir) newStep (newStep - 1) (blockNr + 1)
    where
        newDir = nextDir dir
        newStep =
            if (dir == PY || dir == NY) then
                stepSize + 1
            else
                stepSize

genBlock dir coords stepSize stepsLeft blockNr =
    genBlock dir newCoords stepSize (stepsLeft - 1) (blockNr + 1)
    where
        newCoords = linearCoords coords dir



linearCoords :: Coords -> Direction -> Coords
linearCoords (x, y) dir
    | dir == PX = (x + 1, y)
    | dir == NX = (x - 1, y)
    | dir == PY = (x, y + 1)
    | dir == NY = (x, y - 1)


nextDir :: Direction -> Direction
nextDir PX = PY
nextDir PY = NX
nextDir NX = NY
nextDir NY = PX

