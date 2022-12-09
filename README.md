# algo_graph
Implementations of graph algorithms in strong [^1] accordance with Dr. Ásványi Tibor's lecture notes ([en](http://aszt.inf.elte.hu/%7Easvanyi/ds/AlgDs2/), [hu](http://aszt.inf.elte.hu/~asvanyi/ad/ad2jegyzet/)).

Besides implementing algorithms mentioned on lectures the library / demo also provides a way to render graphs and outputs of graph algorithms to [.dot files](https://en.wikipedia.org/wiki/DOT_(graph_description_language)).
You may view these files [online](https://dreampuf.github.io/GraphvizOnline/) by copying and pasting their content.

All examples in the ```examples``` folder run the algorithms on graphs from the lecture notes.
If you have rust (and cargo) installed on your machine you can run these examples with:
```
cargo run --example <name of example (w/o file extention)>
```
For example to run the Queue-based Bellman-Ford example:
```
cargo run --example qbf
```

[^1]: There were situations were sacrifices had to be made in order to keep the code idiomatic and "rustic".
