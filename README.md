# Graphs

## Authors

- Mikołaj Piróg (@aetn23)
- Mikołaj Wasiak (@RudyMis)

## Overview

A Rust application to edit graphs and run selected algorithms on them.

Inspiration: [this editor](https://csacademy.com/app/graph_editor/)

## Features

# Done

- adding and removing nodes,
- adding and deleting directed edged between nodes,
- moving the graph around the window screen,
- gravity between nodes,
- basic camera transformations: moving (on WSAD), rotating (on Q,E), zooming (on mouse wheel),
- running dfs visualization on the graph,
- creating a clique out of spawned nodes,

# Todo

- adding undirected edges,
- making the windows resizable,
- allowing the nodes to store a string,
- more algorithms,

## Code walkthrough

The three meaningful parts of our application are graphs, step algorithms and the whole drawing to the screen using
egui_tetra.

Graph module implements internal graph representation using petgraph, as well as utility function on graph, gravity and
drawing in particular.

Step algorithms implements running the algorithms on the graph, using Timer and Algorithm structs. The design of this
module allows to implement next algorithms with little overhead.

Drawing utilizes egui for gui (duh), which is pretty self-explanatory in the code itself. As of now, UI drawing awaits
some
better modularization, instead of simple function call to draw everything. Tetra code is mostly handled in
game_state.rs, which calls appropriate functions (drawing, updating, handling events) to handle drawing the screen.
Important thing to note is the details of updating/drawing given thing are delegated to thing's struct, i.e. graph
drawing is implemented in the graph module and called in the game_state.

We encountered little problems of rusty nature - the entire process of producing the code was surprisingly pleasant.
Sometimes we had to dust off our linear algebra skills though.
The biggest challenge was how to approach the modularization. As of now, we do not claim that our approach as of now is the
best and/or final one.

## Libraries

Petgraph for graphs structures, egui_tetra for graphics. egui_tetra is a wrapper for
egui, a gui library, and tetra, a library for game development.

dyn_partial_eq because of [this](https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5)

We must say handling the libraries went surprisingly smooth. Petgraph is widely used and pretty mature, so we expected
no problems, and it delivered. On the other hand, egui_tetra was looking suspicious - it's managed by
Literally One Guy<sup>tm</sup>, so we feared buggy behaviour - especially after Hello World! refused to even compile on
the newest version, and when it did on downgraded one, it produced extremely buggy results. Fortunately, everything went
smooth, and no problems were encountered during the development itself.

## Installation

Petgraph and egui install their dependencies' from crates - no work required on our part.
Tetra has some dependencies that need to be installed manually -
see [this](https://tetra.seventeencups.net/installation).