def square_digits(num):
    return int("".join([str(int(i) ** 2) for i in str(num)]))


x = square_digits(9119)
print(x)


###
def square_digits_test(number):
    new_number = ""
    for digit in str(number):
        digit = int(digit) ** 2
        new_number += str(digit)
    return int(new_number)
###
