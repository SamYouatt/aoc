let readfile filename = Printf.printf "filename: %s\n" filename

let lines filename =
    let channel = open_in filename in
    let rec build_lines lines =
        match input_line channel with
        | line -> build_lines (line :: lines)
        | exception End_of_file -> close_in channel; List.rev lines
    in
    build_lines []
