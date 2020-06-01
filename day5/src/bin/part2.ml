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

let enumerate seq () =
        let rec aux seq idx () =
        match seq () with
        | Seq.Cons (el, seq) -> Seq.Cons ((idx, el), aux seq (idx + 1))
        | Seq.Nil -> Seq.Nil
        in
        aux seq 0 ()

let rec any f seq: bool =
        match seq () with
        | Seq.Cons (el, _) when f el -> true
        | Seq.Cons (_, seq) -> any f seq
        | Seq.Nil -> false

let rec all f seq: bool =
        match seq () with
        | Seq.Cons (el, seq) when f el -> all f seq
        | Seq.Cons (_, _) -> false
        | Seq.Nil -> true

let contains c1 c2 seq: bool =
        seq
        |> skip 1
        |> zip seq
        |> any (fun (a, b) -> c1 == a && c2 == b)

let two_pairs s =
        let seq = String.to_seq s in
        seq
        |> skip 1
        |> zip seq
        |> enumerate
        |> any (fun (idx, (c1, c2)) ->
                seq
                |> skip (idx + 2)
                |> contains c1 c2)

let repeat s =
        let seq = String.to_seq s in
        seq
        |> skip 2
        |> zip (seq |> skip 1)
        |> zip seq
        |> any (fun (c1, (_c2, c3)) -> c1 = c3)

let nice s = two_pairs s && repeat s

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
