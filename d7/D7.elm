module D7 exposing (..)

import Html exposing (..)
import String
import Input
import Debug


type alias Node =
    { parent : Maybe String
    , name : String
    , children : List String
    }


main =
    text
        (Input.str
            |> String.lines
            |> List.filter (\x -> not (String.isEmpty x))
            |> splitChildren
            |> formatNodeStrs
            |> (\list -> List.map (matchParent list) list)
            |> parentLess
        )


parentLess : List Node -> String
parentLess list =
    case List.filter (\x -> x.parent == Nothing) list of
        [] ->
            "No parentless Node found"

        [ x ] ->
            "The parentless Node was " ++ x.name

        l ->
            "Multiple parentless Nodes found: " ++ String.concat (List.map (\x -> x.name) l)


matchParent : List Node -> Node -> Node
matchParent list elem =
    let
        childrenCont l =
            List.filter (\x -> List.member elem.name x.children) l
    in
        case childrenCont list of
            [] ->
                elem

            x :: xs ->
                { elem | parent = Just x.name }


splitChildren : List String -> List ( String, Maybe String )
splitChildren list =
    let
        process elem =
            case String.split "->" elem of
                [] ->
                    Debug.crash "Error, unable to parse empty List"

                [ x ] ->
                    ( x, Nothing )

                [ x, y ] ->
                    ( x, Just y )

                _ ->
                    Debug.crash "I shouldn't exist"
    in
        List.map process list


formatNodeStrs : List ( String, Maybe String ) -> List Node
formatNodeStrs list =
    let
        tupleParser ( name, maybeChild ) =
            case maybeChild of
                Just childStr ->
                    Node Nothing (formatName name) (formatChildren childStr)

                Nothing ->
                    Node Nothing (formatName name) []
    in
        List.map tupleParser list


formatName : String -> String
formatName str =
    case String.split "(" str of
        [] ->
            Debug.crash "Error, why empty?????!?!?!"

        [ x ] ->
            Debug.crash "This shouldn't happn"

        x :: xs ->
            String.trim x


formatChildren : String -> List String
formatChildren str =
    String.split "," str
        |> List.map String.trim
