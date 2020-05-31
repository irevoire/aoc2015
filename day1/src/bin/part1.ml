open Scanf

exception Error of string

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let rec _compute scanner current =
        try
                match bscanf scanner "%c" (fun x -> x) with
                | '(' -> _compute scanner (current + 1)
                | ')' -> _compute scanner (current - 1)
                | '\n' | ' ' -> current (* we finished *)
                | c -> raise (Error (Printf.sprintf "Unknown character %c" c))
        with 
        End_of_file -> current

let compute scanner =
        _compute scanner 0

let res = compute scanner;;
Printf.printf "The instructions brings santa to the %d floors\n" res;;

