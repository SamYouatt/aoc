let find_substring sub s =
    let s_len = String.length s in
    let sub_len = String.length sub in
    let rec aux i =
        if i + sub_len > s_len then None
        else if String.sub s i sub_len = sub then Some i
        else aux (i + 1)
    in
    aux 0

let split_once delimiter s =
    match find_substring delimiter s with
    | Some i ->
            let del_len = String.length delimiter in
            let first = String.sub s 0 i in
            let second = String.sub s (i + del_len) (String.length s - i - del_len) in
            Some (first, second)
    | None -> None
