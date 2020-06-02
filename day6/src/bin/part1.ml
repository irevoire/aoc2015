exception Error of string

let flatten seq =
        let rec aux current remaining () =
                match current () with
                | Seq.Cons(e, seq) -> Seq.Cons(e, aux seq remaining)
                | Seq.Nil ->
                                match remaining () with
                                | Seq.Cons(e, seq) -> aux e seq ()
                                | Seq.Nil -> Seq.Nil
        in
        aux Seq.empty seq

type coord = { x: int; y: int }
let parse_coord s: coord =
        let coord = s
        |> Str.split (Str.regexp ",")
        |> List.map String.trim
        |> List.map Pervasives.int_of_string in
        { x = List.nth coord 0 ; y = List.nth coord 1 }

let display_coord c = Printf.sprintf "%d,%d" c.x c.y

let through start ending () =
        let rec aux current finished () =
                match current with
                | _ when finished -> Seq.Nil
                | { x = x; y = y } when x = ending.x && y = ending.y ->
                                Seq.Cons (current, aux current true)
                | { x = x; y = y } when x = ending.x -> 
                                Seq.Cons (current, aux ({x = start.x; y = y + 1 }) false)
                | { x = x; y = y } -> 
                                Seq.Cons (current, aux ({x = x + 1; y = y}) false)
        in
        aux start false ()

type action =
        | Toggle
        | TurnOff
        | TurnOn

let display_action a =
        match a with
        | Toggle -> "toggle"
        | TurnOff -> "turn off"
        | TurnOn -> "turn on"

let parse_action s =
        match s with
        | "toggle" -> Toggle
        | "on" -> TurnOn
        | "off" -> TurnOff
        | s -> raise (Error (Printf.sprintf "Unknown action: %s\n" s))

type order = { action: action;  from: coord; until: coord }

let display_order o =
        Printf.sprintf "%s %s through %s"
        (display_action o.action)
        (display_coord o.from)
        (display_coord o.until)

let parse_order s =
        let split = Str.split (Str.regexp " ") s in
        if List.length split = 4 then
                { action = parse_action (List.nth split 0);
                from = parse_coord (List.nth split 1);
                until = parse_coord (List.nth split 3)}
        else
                { action = parse_action (List.nth split 1);
                from = parse_coord (List.nth split 2);
                until = parse_coord (List.nth split 4) }

let parse_instructions file =
        let rec aux file l =
                try
                        let instruction = parse_order (input_line file) in
                        aux file (l @ [instruction]) 
        with
                End_of_file -> l
                        in
        aux file []

let grid_get coord grid = grid . (coord.x) . (coord.y)
let grid_set coord grid x = grid . (coord.x) . (coord.y) <- x
let grid_update coord grid fn = grid_set coord grid (fn (grid_get coord grid))

let apply_instruction grid instruction =
        let through = through instruction.from instruction.until in
        match instruction.action with
        | Toggle -> Seq.iter (fun coord -> grid_update coord grid (not)) through
        | TurnOn -> Seq.iter (fun coord -> grid_set coord grid true) through
        | TurnOff -> Seq.iter (fun coord -> grid_set coord grid false) through

let apply_instructions grid instructions =
        List.iter (fun instr -> apply_instruction grid instr) instructions

let file = Sys.argv.(1)
let file = open_in file
let instructions = parse_instructions file

let grid = Array.make_matrix 1000 1000 false;;
apply_instructions grid instructions;;

let res = grid
        |> Array.map Array.to_seq
        |> Array.to_seq
        |> flatten
        |> Seq.map (fun el -> if el then 1 else 0)
        |> Seq.fold_left (+) 0
;;

Printf.printf "After following the instructions, %d lights are lit!\n" res;;
