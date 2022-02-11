# ArcFinder

Arcfinder is a utility for processing GCode files used for
3D-printing.  It takes as input a GCode-file, as produced by a slicer
such as PrusaSlicer or Cura, and produces a new GCode-file as output.

Arcfinder searches for sequences of line segments which form circular
arcs and replaces them by GCode for arcs (G2 and G3).  The options can
be used to adjust how picky the algorithm should be in accepting
arc-like sequences.  The default values are choosen so it tends to
match true arcs in the underlying CAD-model, that have been
discretized into lines before reaching the GCode-stage.

## Usage

    arcfinder [OPTIONS] [INFILE [OUTFILE]]

If no INFILE or OUTFILE is given, then then input is read from Stdin,
and output is written to Stdout.  If INFILE is specified, but no OUTFILE, then
the output overwrites the input file.


## Options

### -c --centers

Emit arc centers instead of radii in G2/G3 commands.

Default is to use radii.


### -m --matches

Minimum number of line segments to be considered.

A higher number reduces the risk of classifying something as an arc by mistake.
A too high number, however, will increase the risk of missing short arcs.

Default: 4


### -e --error

Maximum allowed average (RMS) mismatch between arc and line points (in mm).

If the line segments originate from a true arc, the points will
probably be very close to the arc found.  By using a higher value,
other shapes will also be allowed to be approximated by arcs.

Default: 0.01 mm


### -a --angle

Maximum angle (in degrees) without intermediate points to bridge by an arc.

Use this parameter to prevent arcfinder from smoothing corners that
are supposed to be sharp.  The default value (40 degrees) is choosen to leave
45 and 90 degree corners intact.

Default: 40 degrees


### -d --deviation

Maximal allowed distance (in mm) between original lines and the arc,
due to the curvature of the arc.  This mainly affects long lines by
preventing them from being replaced by large radius arcs.

Default: 0.1 mm
