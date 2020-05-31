open Scanf

exception Error of string

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let volume w h l = w * h * l

let smallest_distances_around_sides w h l =
        List.fold_left min (2 * w + 2 * h) [2 * h + 2 * l; 2 * w + 2 * l]

let ribbon_needed w h l = volume w h l + smallest_distances_around_sides w h l

let rec _compute scanner current =
        try
                let w, h, l = bscanf scanner "%dx%dx%d\n" (fun w h l -> (w, h, l)) in
                let ribbon_needed = ribbon_needed w h l in
                _compute scanner (current + ribbon_needed)
        with 
        | End_of_file -> current
        | Scan_failure _e -> current

let compute scanner =
        _compute scanner 0

let res = compute scanner;;
Printf.printf "Paper needed: %d\n" res;;

