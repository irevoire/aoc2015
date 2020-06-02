open Lib


let grid_get coord grid = grid . (coord.x) . (coord.y)
let grid_set coord grid x = grid . (coord.x) . (coord.y) <- x
let grid_update coord grid fn = grid_set coord grid (fn (grid_get coord grid))

let apply_instruction grid instruction =
        let through = Coord.through instruction.from instruction.until in
        match instruction.action with
        | Toggle -> Seq.iter (fun coord -> grid_update coord grid (not)) through
        | TurnOn -> Seq.iter (fun coord -> grid_set coord grid true) through
        | TurnOff -> Seq.iter (fun coord -> grid_set coord grid false) through

let apply_instructions grid instructions =
        List.iter (fun instr -> apply_instruction grid instr) instructions

let file = Sys.argv.(1)
let file = open_in file
let instructions = parse_orders file

let grid = Array.make_matrix 1000 1000 false;;
apply_instructions grid instructions;;

let res = grid
        |> Array.map Array.to_seq
        |> Array.to_seq
        |> Iter.flatten
        |> Seq.map (fun el -> if el then 1 else 0)
        |> Seq.fold_left (+) 0
;;

Printf.printf "After following the instructions, %d lights are lit!\n" res;;
