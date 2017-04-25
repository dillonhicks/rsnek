# [RSNEK](https://www.youtube.com/watch?v=nMSujTB-SG4)

RSNEK is an implementation of Python with Rust targeting Python 3.6+. The original motivations of RSNEK where to
allow for me to have a deep dive into Rust, a learning by immersion. Now that there is a very minimal
interpreter, I can start to ask the questions that are not possible with a legacy codebase with a bazillion 
consumers like CPython.

## Build and Run

1. Install the Rust Nightly toolchain:

    ```
    $ apt-get update && apt-get install -y \
        cmake \
        curl \
        g++ \
        gcc \
        git \
        make ;

    $ curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
    ```

2. Build the source
    ```
    cd rsnek
    PATH=$HOME/.cargo/bin:$PATH cargo build -p rsnek 
    ```
    Or to build the optimized release version (required for `--green-threads`):
    ```
    PATH=$HOME/.cargo/bin:$PATH cargo build --release -p rsnek 
    ```

3. Run the tests
    ```
    PATH=$HOME/.cargo/bin:$PATH cargo test --all
    ```

4. Run rsnek
    ```
    ./target/debug/rsnek 
    ```
    or
    ```
    ./target/release/rsnek
    ```


## Design


The design is broken up into three main components:


1. ObjectAPI: The interface for every object. In python these 
are often the set of double underscore methods `__add__`.

2. Runtime: The abstraction that is the allocation, builtin provider, and the interpreter.

3. Compiler: Takes python and puts it through a series of tubes. 


## Motivations

**How much do you pay for abstractions?**

Rust provides a fantastic type system and abstractions that are much more modern than C. Can a high level 
implementation in Rust perform like CPython? 

**Parallelism and Concurrency**

The CPython BDFL (GVR) has some strong opinions about reducing the performance of single threaded applications
in CPython versus increasing multithreaded/multiprocessed performance. My inner iconoclast and malcontent
thinks this is totally off point and bogus. Most of the troubles I have with python when working with other 
engineers are lack of a culture of static analysis and how do you do the programmatic yoga to get 

What does it look like if we consider multithreaded performance the top priority? 
What if we want to make green threads a base primitive interpreter? I am not talking about coroutines
which are just a fancy iterator. I am thinking real greenlet/fiber support.


**Tooling and the Joys of Compiling**

Since 3.6 supports annotations, is it possible to create a tool set to help us make better optimization
choices? Further, is it possible to create tooling and opt in compiler extensions that allow us to have
compiler supported codegen? Could we create a cython-esque layer to allow for automatic transpiling of
python to rust?

Could we make our own IR/MIR for this high level interpreter (a la the JVM). If someone wrote a java script front
end, could we make the interpreter instructions general enough to allow it to be a backend for more than
just one language?

CAN WE TARGET WASM AND RUN IT IN THE BROWSER!!?!? I am being totally serious about this one. 


**What is the Dream**

I imagine standing in a dark alley, watching WSGI, usgi, Flask, and the like consumed by a raging dumpster fire.

From the ashes of that fire, a natively implemented HTTP/2 with async and bi directional streaming support rises. 
Since RSNEK was built with parallelism in mind, you don't need nginx/uwsgi-foo 
to utilize all of the cores on your box since parallel execution in the interpreter isn't a big deal.

Since it is python it is easy for people to pick up.

Since it is rust... you can also compile it to wasm and run it in your browser.

Since it is rust and has great tooling, when you need to go native the cost is much lower.

## Q and A

**Q:** RSNEK? Is that the actual name? 

Maybe? It is at least a working name. Sadly, IronPython is already taken.

