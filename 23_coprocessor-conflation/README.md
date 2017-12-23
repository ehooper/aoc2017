Solution to part 2
==================

We begin with the raw program:

````
set b 67
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23
````

A feature of this input is that all jumps are constant values, so the program can be rewritten with labels and gotos:

```
 1: b = 67;
 2: c = b;
 3: if (a != 0)
	goto 5;
 4: if (1 != 0)
	goto 9;
 5: b *= 100;
 6: b -= -100000;
 7: c = b;
 8: c -= -17000;
 9: f = 1;
10: d = 2;
11: e = 2;
12: g = d;
13: g *= e;
14: g -= b;
15: if (g != 0)
	goto 17;
16: f = 0;
17: e -= -1;
18: g = e;
19: g -= b;
20: if (g != 0)
	goto 12;
21: d -= -1;
22: g = d;
23: g -= b;
24: if (g != 0)
	goto 11;
25: if (f != 0)
	goto 27;
26: h -= -1;
27: g = b;
28: g -= c;
29: if (g != 0)
	goto 31;
30: if (1 != 0)
	goto 33;
31: b -= -17;
32: if (1 != 0)
	goto 9;
```

Some of these jumps are unconditional, so they can be replaced with a simple `goto`. Any jumps outside of the program bounds (line 30 in this case) can be replaced with `HALT`.

```
 1: b = 67;
 2: c = b;
 3: if (a != 0)
	goto 5;
 4: goto 9;
 5: b *= 100;
 6: b -= -100000;
 7: c = b;
 8: c -= -17000;
 9: f = 1;
10: d = 2;
11: e = 2;
12: g = d;
13: g *= e;
14: g -= b;
15: if (g != 0)
	goto 17;
16: f = 0;
17: e -= -1;
18: g = e;
19: g -= b;
20: if (g != 0)
	goto 12;
21: d -= -1;
22: g = d;
23: g -= b;
24: if (g != 0)
	goto 11;
25: if (f != 0)
	goto 27;
26: h -= -1;
27: g = b;
28: g -= c;
29: if (g != 0)
	goto 31;
30: HALT;
31: b -= -17;
32: goto 9;
```

By carefully inspecting the jumps, we see that (thankfully) they all can be converted to conditional blocks and loops.

```
 1: b = 67;
 2: c = b;
 3: if (a != 0) {
     5: b *= 100;
     6: b -= -100000;
     7: c = b;
     8: c -= -17000;
}
loop {
     9: f = 1;
    10: d = 2;
    do {
        11: e = 2;
        do {
            12: g = d;
            13: g *= e;
            14: g -= b;
            15: if (g == 0)
                16: f = 0;
            17: e -= -1;
            18: g = e;
            19: g -= b;
        } while (g != 0);
        21: d -= -1;
        22: g = d;
        23: g -= b;
    } while (g != 0);
    25: if (f == 0)
        26: h -= -1;
    27: g = b;
    28: g -= c;
    29: if (g == 0)
        30: HALT;
    31: b -= -17;
}
```

With the jumps abstracted away, we don't need the labels any more.

```
b = 67;
c = b;
if (a != 0) {
    b *= 100;
    b -= -100000;
    c = b;
    c -= -17000;
}
loop {
    f = 1;
    d = 2;
    do {
        e = 2;
        do {
            g = d;
            g *= e;
            g -= b;
            if (g == 0)
                f = 0;
            e -= -1;
            g = e;
            g -= b;
        } while (g != 0);
        d -= -1;
        g = d;
        g -= b;
    } while (g != 0);
    if (f == 0)
        h -= -1;
    g = b;
    g -= c;
    if (g == 0)
        HALT;
    b -= -17;
}
```

To make this easier to understand, we can combine some arithmetic operations into single expressions and change substraction by negative numbers into addition.

```
b = 67;
c = b;
if (a != 0) {
    b *= 100;
    b += 100000;
    c = b + 17000;
}
loop {
    f = 1;
    d = 2;
    do {
        e = 2;
        do {
            g = d * e - b;
            if (g == 0)
                f = 0;
            e += 1;
            g = e - b;
        } while (g != 0);
        d += 1;
        g = d - b;
    } while (g != 0);
    if (f == 0)
        h += 1;
    g = b - c;
    if (g == 0)
        HALT;
    b += 17;
}
```

Now notice that `g` is always set before a condition, so we can replace `g` in these conditions with the last expression that was set.

```
b = 67;
c = b;
if (a != 0) {
    b *= 100;
    b += 100000;
    c = b + 17000;
}
loop {
    f = 1;
    d = 2;
    do {
        e = 2;
        do {
            if (d * e - b == 0)
                f = 0;
            e += 1;
        } while (e - b != 0);
        d += 1;
    } while (d - b != 0);
    if (f == 0)
        h += 1;
    if (b - c == 0)
        HALT;
    b += 17;
}
```

Let's make these conditions a little more readable.

```
b = 67;
c = b;
if (a != 0) {
    b *= 100;
    b += 100000;
    c = b + 17000;
}
loop {
    f = 1;
    d = 2;
    do {
        e = 2;
        do {
            if (d * e == b)
                f = 0;
            e += 1;
        } while (e != b);
        d += 1;
    } while (d != b);
    if (f == 0)
        h += 1;
    if (b == c)
        HALT;
    b += 17;
}
```

Now we have to so some thinking. The condition `d * e == b` is checked for all `e` between 2 and `b` (not including `b` itself). This is the same as `e == b / d` for some such `e`, or that `d` *divides* `b`. Note that we don't need to consider `b == d`, so this can be replaced with a modulus operation.

```
b = 67;
c = b;
if (a != 0) {
    b *= 100;
    b += 100000;
    c = b + 17000;
}
loop {
    f = 1;
    d = 2;
    do {
        if (b % d == 0)
            f = 0;
        d += 1;
    } while (d != b);
    if (f == 0)
        h += 1;
    if (b == c)
        HALT;
    b += 17;
}
```

The last do-while loop will set `f` to 0 if there is any `d` between 2 and `b` (not including `b`) that divides `b`, i.e. if `b` is a composite number (from the inverse of the definition of a prime number). So the program is counting the composite numbers between the initial value of `b` and `c` in increments of 17.

```
b = 67;
c = b;
if (a != 0) {
    b *= 100;
    b += 100000;
    c = b + 17000;
}
loop {
    if (!is_prime(b))
        h += 1;
    if (b == c)
        HALT;
    b += 17;
}
```

Python solution for completeness:

```python
from math import sqrt

def is_prime(n):
    return not any(map(lambda d: n % d == 0, range(2, int(sqrt(n)) + 1)))

b = 67 * 100 + 100000
c = b + 17000
h = 0

while True:
    if not is_prime(b):
        h += 1
    if b == c:
        break
    b += 17

print(h)
```
