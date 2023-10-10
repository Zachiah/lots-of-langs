[<EntryPoint>]
let main args =
    let name = if args.Length > 0 then args.[0] else "Nobody" in
    printfn $"Hello {name}"
    0