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

# Todo
- adding undirected edges,
- making the windows resizable,
- allowing the nodes to store a string,
- more algorithms,

## Libraries
Petgraph for graphs structures, egui_tetra for graphics. egui_tetra is a wrapper for
egui, a gui library, and tetra, a library for development.

## Installation
Petgraph and egui install their dependencies' from crates - no work required on our part.
Tetra has some dependencies that need to be installed manually - see [this](https://tetra.seventeencups.net/installation).