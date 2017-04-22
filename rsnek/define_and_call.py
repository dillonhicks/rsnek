def call_print():
    print("hello from the inside!")

call_print()


a = 12

def add_a(value):
    print(a + value)

print(add_a(234))


def another(thing1, thing2):
    x = thing1 ** thing2
    x = x << 12
    x = str(x)
    print(x + " PYTHONs")


another(2, 20)

print()
