def hello():
    x = 1 + 2
    y = 3 + 4
    def potato(v):
        i_am_a_nested_scope_object = v * 20

        return "yams" * i_am_a_nested_scope_object


    return potato

def operator():
    return "call me anytime"



operator()

print(1)


g = hello()

print(g(13))
