open Scanf
exception Error of string

let rec zip a b () =
        match a (), b () with
        | Seq.Cons (left, a), Seq.Cons (right, b) -> Seq.Cons ((left, right), zip a b)
        | _ -> Seq.Nil (* if anything is nil, stop *)

let rec skip n seq () =
        match n, seq () with
        | 0, seq -> seq
        | n, Seq.Cons (_el, seq) -> skip (n - 1) seq ()
        | _, Seq.Nil -> Seq.Nil

let rec any f seq =
        match seq () with
        | Seq.Cons (el, _) when f el -> true
        | Seq.Cons (_, seq) -> any f seq
        | Seq.Nil -> false

let rec all f seq =
        match seq () with
        | Seq.Cons (el, seq) when f el -> all f seq
        | Seq.Cons (_, _) -> false
        | Seq.Nil -> true

let three_vowels s =
        let nb_vowels = s
        |> String.to_seq
        |> Seq.map (fun c -> match c with
                | 'a' | 'e' | 'i' | 'o' | 'u' -> 1
                | _ -> 0)
        |> Seq.fold_left (+) 0 in
        nb_vowels >= 3

let twice_in_a_row s =
        let seq = String.to_seq s in
        seq
        |> zip (skip 1 seq)
        |> any (fun (c1, c2) -> c1 = c2)

let no_banned_string s =
        let seq = String.to_seq s in
        seq
        |> skip 1
        |> zip seq
        |> all (fun (c1, c2) ->
                        match c1, c2 with
                        | 'a', 'b' | 'c', 'd' | 'p', 'q' | 'x', 'y' -> false
                        | _ -> true
        )

let nice s = three_vowels s && twice_in_a_row s && no_banned_string s

let compute scanner =
        let rec _compute scanner n =
                try
                        let s = bscanf scanner "%s\n" (fun x -> x) in
                        if nice s then _compute scanner (n + 1) else _compute scanner n
                with
                End_of_file -> n
        in
        _compute scanner 0

let file = Sys.argv.(1)
let scanner = Scanning.from_file file

let res = compute scanner;;

Printf.printf "There is %d nice strings\n" res;;
