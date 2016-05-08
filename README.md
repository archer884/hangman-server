# hangman-server
This is a (really) simple Hangman game written as a (silly) exercise in client/server programming, or something like that. So far, the naive reference implementation of my solver is able to achieve a 75% success rate against the words selected by the server, which includes pretty much everything in the `enable1.txt` word list that doesn't have any repeated letters in it and isn't either less than four or more than seven characters in length.

Won: 76869 | Lost: 23131

## Update 1
This first update allows the server to prefer words with less common letters, making the dictionary 2/3rds smaller but significantly more difficult for the naive solver to crack. Results were impressive, with the ratio of wins to losses dropping to about 50/50.

Won: 50483 | Lost: 49517

The current implementation also makes shorter words more valuable than longer words, which  biases the word list toward them. This log segment makes that abundantly clear:

    Correct word: cling
    Correct word: hind
    Correct word: meou
    Won: bang (Strikes: 4)
    Won: glace (Strikes: 3)
    Won: jinked (Strikes: 4)
    Won: justly (Strikes: 5)
    Correct word: jato
    Correct word: minx
    
## Update 2
This update allows you to have the server start in easy, normal, or hard modes. Easy mode just inverts the ranking strategy used with hard mode, resulting in longer words with more commonly found letters.
