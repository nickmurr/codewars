def square_digits(num):
    return "".join([str(int(i)*int(i)) for i in str(num)])


x = square_digits(9119)
print(x)
