* arcfinder

Arcfinder it a utility for processing GCode files used for 3D-printing.
It takes a GCode file, as produced by a slicer such as PrusaSlicer of Cura,
as input, and produces a new GCode file as output.

Arcfinder searches for sequences of line segments which form circular
arcs and replaces them by GCode for arcs (G2 and G3).  The options can
be used to adjust how picky the algorithm should be in accepting
arc-like sequences.  The default values are choosen so it tends to
match true arcs in the underlying CAD-model, that have been
discretized into lines before reaching the GCode-stage.

** Usage
arcfinder [FLAGS] [OPTIONS] <INFILE >OUTFILE


** FLAGS
*** -c --centers

Emit arc centers instead of radii in G2/G3 commands.

Default is to use radii.


** OPTIONS

*** -m --matches

Minimum number of line segments to be considered.

Default: 4


*** -e --error

Maximum allowed average (RMS) mismatch between arc and line points (in mm).

Default: 0.01 mm


*** -a --angle

Maximum angle (in degrees) without intermediate points to bridge by an arc.

Default: 40 degrees


*** -d --deviation

Maximal allowed mismatch (in mm) between original lines and the arc.

Default: 0.1 mm
