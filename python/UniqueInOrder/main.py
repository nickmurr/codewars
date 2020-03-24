def unique_in_order(iterable):
    out = []
    unique = None;

    for val in iterable[0:]:
        if val != unique:
            out.append(val)
            unique = val

    return out


print(unique_in_order('AAAABBBCCDAABBB'))
print(unique_in_order([1, 2, 2, 3, 3, 3, 3, 4, 5, 1, 1, 2]))
print(unique_in_order("A"))
print(unique_in_order("AA"))
print(unique_in_order([]))

###
unique_in_order = lambda l: [z for i, z in enumerate(l) if i == 0 or l[i - 1] != z]
###
