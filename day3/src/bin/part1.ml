open Scanf
exception Error of string

type coord = { x: int; y: int}

module SCoord = Set.Make(struct type t = coord let compare = compare end)

let rec _compute scanner (visited: SCoord.t) (current: coord) =
        try
                let current = match bscanf scanner "%c" (fun x -> x) with
                | 'v' -> { x = current.x; y = current.y + 1}
                | '^' -> { x = current.x; y = current.y - 1}
                | '>' -> { x = current.x + 1; y = current.y}
                | '<' -> { x = current.x - 1; y = current.y}
                | '\n' | ' ' -> current (* we finished *)
                | c -> raise (Error (Printf.sprintf "Unknown character %c" c))
                in
                _compute scanner (SCoord.add current visited) current
        with 
        End_of_file -> visited

let compute scanner =
        _compute scanner (SCoord.singleton {x = 0; y = 0}) {x = 0; y = 0}

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let res = compute scanner;;
Printf.printf "The instructions brings santa to the %d floors\n" (SCoord.cardinal res);;

