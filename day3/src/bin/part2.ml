open Scanf
exception Error of string

type coord = { x: int; y: int}

module SCoord = Set.Make(struct type t = coord let compare = compare end)

let rec _compute scanner (visited: SCoord.t) (santa: coord) (bot: coord) =
        try
                let santa = match bscanf scanner "%c" (fun x -> x) with
                | 'v' -> { x = santa.x; y = santa.y + 1}
                | '^' -> { x = santa.x; y = santa.y - 1}
                | '>' -> { x = santa.x + 1; y = santa.y}
                | '<' -> { x = santa.x - 1; y = santa.y}
                | '\n' | ' ' -> santa (* we finished *)
                | c -> raise (Error (Printf.sprintf "Unknown character %c" c))
                in
                let bot = match bscanf scanner "%c" (fun x -> x) with
                | 'v' -> { x = bot.x; y = bot.y + 1}
                | '^' -> { x = bot.x; y = bot.y - 1}
                | '>' -> { x = bot.x + 1; y = bot.y}
                | '<' -> { x = bot.x - 1; y = bot.y}
                | '\n' | ' ' -> bot (* we finished *)
                | c -> raise (Error (Printf.sprintf "Unknown character %c" c))
                in
                let set = visited |> SCoord.add santa |> SCoord.add bot in
                _compute scanner set santa bot
        with 
        End_of_file -> visited

let compute scanner =
        _compute scanner (SCoord.singleton {x = 0; y = 0}) {x = 0; y = 0} {x = 0; y = 0}

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let res = compute scanner;;
Printf.printf "The instructions brings santa to the %d floors\n" (SCoord.cardinal res);;

