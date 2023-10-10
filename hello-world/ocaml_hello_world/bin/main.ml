let () = Printf.printf "Hello %s\n"  (if Array.length Sys.argv > 1 then Sys.argv.(1) else "Nobody")
