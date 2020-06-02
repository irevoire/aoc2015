open Lib

let grid_get coord grid = grid . (coord.x) . (coord.y)
let grid_set coord grid x = grid . (coord.x) . (coord.y) <- x
let grid_update coord grid fn = grid_set coord grid (fn (grid_get coord grid))

let apply_instruction grid instruction =
        let through = Coord.through instruction.from instruction.until in
        match instruction.action with
        | Toggle -> Seq.iter (fun coord -> grid_update coord grid (fun a -> a + 2)) through
        | TurnOff -> Seq.iter
        (fun coord -> grid_update coord grid (fun a -> if a <= 0 then 0 else a - 1))
        through
        | TurnOn -> Seq.iter (fun coord -> grid_update coord grid (fun a -> a + 1)) through

let apply_instructions grid instructions =
        List.iter (fun instr -> apply_instruction grid instr) instructions

let file = Sys.argv.(1)
let file = open_in file
let instructions = parse_orders file

let grid = Array.make_matrix 1000 1000 0;;
apply_instructions grid instructions;;

let res = grid
        |> Array.map Array.to_seq
        |> Array.to_seq
        |> Iter.flatten
        |> Seq.fold_left (+) 0
;;

Printf.printf "The total brightness of all lights combined is %d\n" res;;
