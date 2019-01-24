### Some initial thoughts on syntax.

Mutability: where and why is mutability necessary? For CSG operations mutability doesn't make much
sense, but for the sketching operations I seem to be relying heavily on mutability. Can mutability
be eliminated from sketching operations, and if not, can or should mutable operations be quarantined
for the sake of the rest of the language?

The sketch objects have a pretty clear path for both usage and implementation, but what does the
syntax look like? Do we give sketches special treatment, do we offload all of the sugar on logic in
the sketch object like the builder pattern, or is it possible to neatly accommodate the pattern when
designing the language syntax?

I'm currently using `~` to indicate unconstrained parameters in sketch operations. I like the idea;
it makes the drawing process a flexible sort of hybrid between the conventional pen-plotter type
drawing operations and the constraint based workflow that has then potential to eliminate many
headaches. But does the existence of such an operator imply that other operations should accept it
as well? Should we allow provisional extrusions that can be constrained after the fact? That's not
exactly unreasonable; solvespace does it, but it does seem a little incompatible with the idea that
solid objects are largely immutable.

I'm partial to binary operators representing common CSG operations. I think that `&` and `|` have
logical extensions to the solid 'and' and 'or'.

The situation with extrusions already rankles me - right now I assume that the extrusion operation
returns a collection of objects, because a sketch can contain disconnected regions that remain
disconnected upon extrusion. Most common operations that I expect to perform are similar in one way
or another. Even a union of two individual objects is not guaranteed to be connected. Maybe I'm
approaching this wrong and shouldn't enforce connectivity for objects; it feels a little arbitrary
anyway. Instead, functions could be provided for separating disconnected objects if it is needed.
That would simplify many things, and I'm not sure there would be any cost to it.

The constraints seem a little interesting. Maybe I already covered this, but the sketch object acts
as a context and container for all components of the sketch, with anything returned by the
`sketch.*` functions essentially being references to internal state on the sketch object.  I find
this dissatisfying, especially considering that trying to pass a sketch object a reference from a
different sketch is clearly problematic, but a runtime error at best. Although, maybe it isn't
problematic; maybe that just projects the item onto the target sketch. But constraints thing sort of
requires a graph structure, which needs management. Alternatives I can think of - we could rely on
the user to aggregate all the individual objects in the sketch and provide a constraint solving
method. But this doesn't eliminate the graph structure, it just makes it more annoying, maybe.
Sketches could be confined to special blocks that make the context something that surrounds the
code, instead of a property of some object. This is close to what I had in mind when I talked about
quarantine earlier. Maybe I need to mess around with it a bit more.

Less generally, the distance constraints might have a problem: the `hdist` and `vdist` methods work
fine on points, but run into problems if a line is not vertical or horizontal, respectively. A
runtime error would probably be appropriate there. On circles, there is even more of a problem -
from which side of the circle do we measure the distance? How do we measure the distance to an
object inside the circle? I think that the answer may be that for the distance constraints we allow
negative distances, and have the order of the arguments determine the parity. I think that it would
be relatively easy to implement, but it seems odd that the `hdist` function would measure
differently depending on argument order.


