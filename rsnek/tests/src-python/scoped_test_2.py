def hello():
    print("yup")



def scope():
    def scope_again():
        def scope_yet_again():
            def scope_scope_scope_my_boat():
                def down_the_stream():
                    return None
                return down_the_stream
            return scope_scope_scope_my_boat
        return scope_yet_again
    return scope_again
