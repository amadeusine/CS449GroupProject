
# Table of Contents

1.  [meeting minutes](#org8791c46)
    1.  [meeting 2019.09.04 (first)](#org0236bec)
        1.  [project 1 report](#org9f358c6)
        2.  [functionality of sprint 1](#org9328a5f)
    2.  [meeting 2019.09.06 (second)](#org7e5f088)
        1.  [agenda](#org3dd35ba)
        2.  [game rules](#org390e312)
        3.  [User stories](#orged4beb7)
        4.  [tooling & design](#org5e65db6)
        5.  [discuss tooling](#org0bfc8d6)
        6.  [discuss design](#org4fcfc80)
    3.  [meeting 2019.09.27 (third)](#org592a002)
        1.  [agenda](#orgbc06ecd)
        2.  [project structure](#org9be8441)
        3.  [acceptance criteria](#orga813dae)
        4.  [work assignment](#org4b57f11)
        5.  [remaining todos](#org92fcc32)


<a id="org8791c46"></a>

# meeting minutes


<a id="org0236bec"></a>

## meeting 2019.09.04 (first)

-   going over project pdf as group
    -   discussing tech stack
    -   going over sprint assignments
    -   going over normal assignments
-   discussing the actual structure of sprint 1
    -   requirements
    -   user stories
    -   what submission might look like
    -   discussion of who gets to do what
    -   discussion of when to meet, general availability
        -   Sam will be gone from 9th through 19th
        -   Elias will be gone through the 12th - 14th


<a id="org9f358c6"></a>

### TODO project 1 report

-   want to get scrum documentation done
-   get general idea down by end of this friday (2019.09.06)
    -   structure of the project
    -   how to use the frameworks/libraries involved (personal research/reading
        per individual)
        -   Neon for rust
        -   node.js
        -   potentially express.js
-   generating cards, user stories


<a id="org9328a5f"></a>

### TODO functionality of sprint 1

-   finish the functional/technical components of the project
-   TBD


<a id="org7e5f088"></a>

## meeting 2019.09.06 (second)


<a id="org3dd35ba"></a>

### agenda

-   discussing game rules
-   discussing/writing user stories
-   discussing tooling
-   discussing design


<a id="org390e312"></a>

### game rules

-   watched a video demo'ing the game
-   discussed/clarify mechanics
    -   whether or not to include coin flip
    -   terms of loss
    -   flying mechanic


<a id="orged4beb7"></a>

### User stories

-   elias wrote user stories in a new org mode file called kanban.org on the
    repository
-   discussed problem of documentation given requirements from the pdf for
    sprint 1
-   discussed alternative means of documenting, carrying out execution of our
    cards for the project


<a id="org5e65db6"></a>

### tooling & design

-   did not achieve agenda, did not get to these topics because of the time
    it took to discuss our epics/user stories.


<a id="org0bfc8d6"></a>

### TODO discuss tooling

-   need to finalize what our stack will look like and frameworks to be
    used.
-   elias has experimented with Neon and reports that it works well, seems
    viable for the product.


<a id="org4fcfc80"></a>

### TODO discuss design

-   need to discuss how the actual product will be packaged and its
    architecture.


<a id="org592a002"></a>

## meeting 2019.09.27 (third)


<a id="orgbc06ecd"></a>

### agenda

-   discuss project structure
-   acceptance criteria
-   work assignment
-   remaining TODOs


<a id="org9be8441"></a>

### project structure

-   express.js has a lot of dependencies, only really need connect.js
    -   might try just using connect.js, which would be a lot simpler
    -   will continue with using Neon
-   board
    -   gui
        -   js renders the fontend
        -   logic/data is all handled on back
    -   data structure/representation
        -   two choices:
            1.  one big board object that includes methods for both resolving where players are **and**
                where things like mills are
            2.  two object entities, one is purely for the GUI (tracking positions on the board), the
                other would be some kind of graph structure that allows position nodes to check peers
                for occupation and whether it is the same or opposing players
    -   movement and move validity
        -   need to track flying
            -   proposition: flying is a universal property, merely constrained until player count is
                reduced.
                -   need some kind of getter/setter between board and entity management system
            -   mill detection
                -   if going with entity system, would merely be a graph traversal from any given node
                -   another idea: create a mill entity system that tracks active mills and checks each
                    mill upon each turn(?) and modifies or destroys the mill as necessary.
                    -   could save a lot of checking
                    -   as for organization/logical membership, would keep such a mill entity system
                        independent of other objects in the system for simplicity, at least for now.
            -   Checking for attack
                -   if a mill entity system is used, we natively have a means to detect valid attacks. so
                    long as the node is not in one of the mills, do not attack **unless** all available
                    nodes are in mills.
    -   game driver
        -   Will have some kind of Game entity/manager object that drives the game event loop.
            -   will take inputs from players, run them as game moves
                -   however, internal logic to the entity management system is what will ultimately validate moves
                -   game manager will have no logic for why this happens, only passes back and for game
                    inputs and the results of moves.
            -   consequentially, need to codify where and how game validation logic happens
    -   validation logic
        -   as of now, think it will be handled by the main entity management system
        -   will have a set of logic checking methods defined over the system that verify whether a
            given move is allowed


<a id="orga813dae"></a>

### acceptance criteria

-   realized we need to add numbering to the board GUI (a-g, 1-7)
-   (deferred, Sam will begin working on before next meeting)


<a id="org4b57f11"></a>

### work assignment

-   elias will begin on exploratory work for the backend (board, entity management, etc)
-   sam, michael will begin exploratory work for the frontend (GUI, communicating with backend)


<a id="org92fcc32"></a>

### remaining todos

1.  TODO kanban board setup, finalization of workflow for documentation

    -   can probably just use github for real time management, but keep organizational and notes in
        `kanban.org` file on the repo.

2.  TODO defining test cases for stories and acceptance criteria

3.  TODO refining stories

    -   same case applies with above: refine stories, and put them on github's project management
        board accordingly; actual refinement can be delegated to within `kanban.org` file.

