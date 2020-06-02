exception Error of string

module Iter = struct
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
end

type coord = { x: int; y: int }
module Coord = struct
        let display c = Printf.sprintf "%d,%d" c.x c.y

        let parse s: coord =
                let coord = s
                |> Str.split (Str.regexp ",")
                |> List.map String.trim
                |> List.map int_of_string in
                { x = List.nth coord 0 ; y = List.nth coord 1 }

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
end

type action =
        | Toggle
        | TurnOff
        | TurnOn
module Action = struct
        let display a =
                match a with
                | Toggle -> "toggle"
                | TurnOff -> "turn off"
                | TurnOn -> "turn on"

        let parse s =
                match s with
                | "toggle" -> Toggle
                | "on" -> TurnOn
                | "off" -> TurnOff
                | s -> raise (Error (Printf.sprintf "Unknown action: %s\n" s))
end

type order = { action: action;  from: coord; until: coord }
module Order = struct
        let display o =
                Printf.sprintf "%s %s through %s"
                (Action.display o.action)
                (Coord.display o.from)
                (Coord.display o.until)

        let parse s =
                let split = Str.split (Str.regexp " ") s in
                if List.length split = 4 then
                        { action = Action.parse (List.nth split 0);
                        from = Coord.parse (List.nth split 1);
                        until = Coord.parse (List.nth split 3)}
                else
                        { action = Action.parse (List.nth split 1);
                        from = Coord.parse (List.nth split 2);
                        until = Coord.parse (List.nth split 4) }
end

let parse_orders file =
        let rec aux file l =
                try
                        let instruction = Order.parse (input_line file) in
                        aux file (l @ [instruction]) 
        with
                End_of_file -> l
                        in
        aux file []
