open Scanf

exception Error of string

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let rec _compute scanner current pos =
        try
                match bscanf scanner "%c" (fun x -> x) with
                | _ when current = -1 -> pos
                | '(' -> _compute scanner (current + 1) (pos + 1)
                | ')' -> _compute scanner (current - 1) (pos + 1)
                | '\n' | ' ' -> raise (Error "Never went to the floor -1")
                | c -> raise (Error (Printf.sprintf "Unknown character %c" c))
        with 
        End_of_file -> pos

let compute scanner =
        _compute scanner 0 0

let res = compute scanner;;
Printf.printf "The position of the instruction that brings santa to enter the basement is %d\n" res;;

