let () =
    Files.lines "inputs/day1.txt"
    |> List.filter_map (fun line -> Strings.split_once "   " line)
    |> List.iter (fun (first, second) -> Printf.printf "%s %s\n" first second)

