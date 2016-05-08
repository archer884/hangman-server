# hangman-server
This is a (really) simple Hangman game written as a (silly) exercise in client/server programming, or something like that. So far, the naive reference implementation of my solver is able to achieve a 75% success rate against the words selected by the server, which includes pretty much everything in the `enable1.txt` word list that doesn't have any repeated letters in it and isn't either less than four or more than seven characters in length.

Won: 76869 | Lost: 23131