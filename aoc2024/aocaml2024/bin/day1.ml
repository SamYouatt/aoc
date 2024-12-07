let () =
  let lines = Files.lines "inputs/day1.txt" in
  List.iter (fun line -> match Strings.split_once "   " line with
  | Some(x, y) -> Printf.printf "%s %s\n" x y
  | None -> ()
) lines
