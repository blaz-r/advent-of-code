/*
    this one is yet again solved by hand...
    as you can see from input copy, it took me a bit to realize that z
    is working like a stack and when "div z 1" appears, we just push to z. So if we want z to be 0, we need to make sure we don't push when we "pop" with div 26.

    Simplified:
    push    w1+6

    push    w2+12

    push    w3+5

    push    w4+10

    pop     w4+10
    if w4+10-16 == w5 -> no push

    push    w6

    push    w7+4

    pop     w7+4
    if w7+4-4 == w8 -> no push

    push    w9+14

    pop     w9+14
    if w9+14-7 == w10 -> no push

    pop w6
    if w6-8 == w11 -> no push

    pop     w3+5
    if w3+5-4 == w12 -> no push

    pop     w2+12
    if w2+12-15 == w13 -> no push

    pop     w1+6
    if w1+6-8 == w14 -> no push

    This means:
    w4-6    == w5
    w7      == w8
    w9+7    == w10
    w6-8    == w11
    w3+1    == w12
    w2-3    == w13
    w1-2    == w14

    then we plug in the numbers
    Highest:
    1 ->  9
    2 ->  9
    3 ->  8
    4 ->  9
    5 ->  3
    6 ->  9
    7 ->  9
    8 ->  9
    9 ->  2
    10 -> 9
    11 -> 1
    12 -> 9
    13 -> 6
    14 -> 7

    99893999291967
    
    plug the numbers for smallest:
    1 ->  3
    2 ->  4
    3 ->  1
    4 ->  7
    5 ->  1
    6 ->  9
    7 ->  1
    8 ->  1
    9 ->  1
    10 -> 8
    11 -> 1
    12 -> 2
    13 -> 1
    14 -> 1

    34171911181211

*/


fn main() {
    println!("PART1: 99893999291967");
    println!("PART2: 34171911181211");
}
