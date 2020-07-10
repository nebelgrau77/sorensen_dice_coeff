### Sørensen–Dice coefficient calculator

https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient

Calculates the Sørensen–Dice coefficient between two strings supplied as arguments: 

_sdcoeff \<word_a\> \<word_b\>_

__TO DO: handle Unicode correctly, will panic if word ends with a character like "ł".__

Need help with the Unicode improvement: the idea is to split the word into a vector of chars, but then the bigrams need to be collected back into &str.
When I do that, it won't compile. The _get\_bigrams_ function _"returns a value referencing data owned by the current function"_.