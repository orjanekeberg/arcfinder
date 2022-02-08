* arcfinder

** Usage
arcfinder [FLAGS] [OPTIONS] <infile >outfile


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
