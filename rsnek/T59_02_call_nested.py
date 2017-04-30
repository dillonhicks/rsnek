def add_two(f):
    def add_two_more(g):
        return g + 2

    return add_two_more(f + 2)


print(add_two(4))

# this totally works due to the lack of scopes as of v0.5.0
# which is hilarious and disturbing.
print(add_two_more(34))
