# jblomlof-task-18

___18 &lt; 17___

## update
Faster working version with inspiration from how Toshi (sakao) structured his files.

Using some more disk space tho...

## Finished
This was a great experience. As Im writing this Im still fuming about tht `token.txt` not being sensible. TBH I should've just done my own.

### To run
Change the constants related to token.txt and korpus. Use `cargo r --release _compile` to compile index files. ISSUE: Your anitmalware might think its some danger and start using a lot of cpu. Mine used abut 50% while running compile mode, so if you want it to run faster maybe disable idk.  Use `cargo r --release <word>` to look up usage of word. 

### Fun facts
`--release` speeds up compilation from like 210 seconds to 170 seconds. (on my computer, duh).  
This program generates about 400_000 file and 7_000 folders, when used on token.txt. Meaning there's aprox 7_000 unique hashes and 400_000 words(Not unique shit like 'á' and 'a' is the same).  
When you push `223 as char` to a string in rust. It pushes more than one byte.

### problems with token.txt
All of 'ä' 'å' 'ö' have the same byte presentation in the file. Thus making it impossible to differentiate words like "för" and "fär".  I still think they are sorted correctly you could probably do something about that, but I wont this time.  My code probably doesnt handle this issue correctly.

## old stuff

#### Update

Working compile files.

#### TODO
Lookup

#### FUN FACT

when run with `--release` it almost took half as long as without it...  
I hope that's just because it stops backtracing and stuff, and not that my code is so bad it can optimize it so heavily...