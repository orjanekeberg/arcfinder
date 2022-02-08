import sys, re
from bestarc import findBestArc

emitradius = True
minmatch = 4

storage = []
currentX, currentY = 0, 0
relExtrusion = False

def storeMove(x, y, e):
    storage.append((x, y, e))

def processMoves():
    global currentX, currentY
    while storage:
        if not matchArc():
            currentX, currentY = storage[0][0], storage[0][1]
            print("G1 X%5.3f Y%5.3f E%.5f" % storage.pop(0))


def matchArc():
    global currentX, currentY
    if len(storage) < minmatch:
        return False
    i = minmatch
    lastBestArc = None
    bestArc = findBestArc(currentX, currentY, storage, i)
    while bestArc != None:
        lastBestArc = bestArc
        i += 1
        if len(storage) < i:
            bestArc = None
        else:
            bestArc = findBestArc(currentX, currentY, storage, i)
    if lastBestArc == None:
        return False
    Esum = 0
    if relExtrusion:
        for ii in range(i-1):
            Esum += storage[ii][2]
    else:
        Esum = storage[i-2][2]
    

    if emitradius:
        print("%s X%5.3f Y%5.3f R%5.3f E%.5f" % (lastBestArc[0], storage[i-2][0], storage[i-2][1], lastBestArc[1], Esum))
    else:
        print("%s X%5.3f Y%5.3f I%5.3f J%5.3f E%.5f" % (lastBestArc[0], storage[i-2][0], storage[i-2][1],
                                                        lastBestArc[2]-currentX, lastBestArc[3]-currentY, Esum))

    currentX, currentY = storage[i-2][0], storage[i-2][1]
    for ii in range(i-1):
        storage.pop(0)
    return True


for line in sys.stdin:
    hit = re.match("^G1 X([+-]?(?:\d+\.?\d*|\.\d+)) Y([+-]?(?:\d+\.?\d*|\.\d+)) E([+-]?(?:\d+\.?\d*|\.\d+))", line)
    if hit:
        newX = float(hit.group(1))
        newY = float(hit.group(2))
        newE = float(hit.group(3))
        storeMove(newX, newY, newE)
    else:
        processMoves()
        print(line, end='')

        hit = re.match("^G[0123] .*X([+-]?(?:\d+\.?\d*|\.\d+))", line)
        if hit:
            currentX = float(hit.group(1))
        hit = re.match("^G[0123] .*Y([+-]?(?:\d+\.?\d*|\.\d+))", line)
        if hit:
            currentY = float(hit.group(1))

    if re.match("^M82\D", line):
        relExtrusion = False
    else:
        if re.match("^M83\D", line):
            relExtrusion = True
