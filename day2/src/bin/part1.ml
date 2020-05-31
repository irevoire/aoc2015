open Scanf

exception Error of string

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let surface w h l =
        2 * w * h +
        2 * h * l +
        2 * w * l

let smallest_face w h l = List.fold_left min (w * h) [h * l; w * l]

let paper_needed w h l = surface w h l + smallest_face w h l

let rec _compute scanner current =
        try
                let w, h, l = bscanf scanner "%dx%dx%d\n" (fun w h l -> (w, h, l)) in
                let paper_needed = paper_needed w h l in
                _compute scanner (current + paper_needed)
        with 
        | End_of_file -> current
        | Scan_failure _e -> current

let compute scanner =
        _compute scanner 0

let res = compute scanner;;
Printf.printf "Paper needed: %d\n" res;;

