exception Error of string

let valid digest =
        String.get digest 0 == '0' &&
        String.get digest 1 == '0' &&
        String.get digest 2 == '0' &&
        String.get digest 3 == '0' &&
        String.get digest 4 == '0';;

let rec _compute input seed: int =
        let password = Printf.sprintf "%s%d" input seed in
        let md5 = Digest.string password |> Digest.to_hex in
        if md5 |> valid then
                seed
        else
                _compute input (seed + 1)

let compute input =
        _compute input 0

let input = Sys.argv.(1)
let res = compute input;;

Printf.printf "The salt is %d\n" res;;
