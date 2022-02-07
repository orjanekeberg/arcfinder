from math import sqrt, fabs, pi, sin, cos, atan2
import matplotlib.pyplot as plt
import numpy as np

rmslimit = 0.01
anglelimit = 40 * pi/180
offsetlimit = 0.1

def sqr(x):
    return x*x



def centre(x1, y1, x2, y2, x3, y3):
    x12 = x1 - x2
    x13 = x1 - x3

    y12 = y1 - y2
    y13 = y1 - y3

    y31 = y3 - y1
    y21 = y2 - y1

    x31 = x3 - x1
    x21 = x2 - x1

    # x1^2 - x3^2
    sx13 = pow(x1, 2) - pow(x3, 2)

    # y1^2 - y3^2
    sy13 = pow(y1, 2) - pow(y3, 2)

    sx21 = pow(x2, 2) - pow(x1, 2)
    sy21 = pow(y2, 2) - pow(y1, 2)

    xden = (2 * ((x31) * (y12) - (x21) * (y13)))
    yden = (2 * ((y31) * (x12) - (y21) * (x13)))

    if xden==0:
        g = -x3
    else:
        g = ((sx13) * (y12) + (sy13) * (y12) +
             (sx21) * (y13) + (sy21) * (y13)) / xden

    if yden==0:
        f = -y3
    else:
        f = ((sx13) * (x12) + (sy13) *
             (x12) + (sx21) * (x13) +
             (sy21) * (x13)) / yden

    return -g, -f


def bestArc(Xa, Ya, Xb, Yb, XYp):
    Xab2 = (Xa+Xb)/2
    Yab2 = (Ya+Yb)/2

    k = sqrt(sqr(Yb-Ya) + sqr(Xa-Xb))
    # Guard
    if k == 0:
        return 0, 0, 0, 1000

    def G(d):
        sum = 0
        for p in XYp:
            sum += sqr(sqrt(sqr(Xab2 + d/k*(Yb-Ya) - p[0]) +
                            sqr(Yab2 + d/k*(Xa-Xb) - p[1])) -
                       sqrt(sqr(d) + sqr(k)/4))
        return sum

    def dG(d):
        sum = 0
        for p in XYp:
            sum += 2*(sqrt(sqr(Xab2 + d/k*(Yb-Ya) - p[0]) +
                           sqr(Yab2 + d/k*(Xa-Xb) - p[1])) -
                      sqrt(sqr(d) + sqr(k)/4)) * \
                     (1/sqrt(sqr(Xab2 + d/k*(Yb-Ya) - p[0]) +
                             sqr(Yab2 + d/k*(Xa-Xb) - p[1])) *
                      ((Xab2 + d/k*(Yb-Ya) - p[0]) * (Yb-Ya)/k +
                       (Yab2 + d/k*(Xa-Xb) - p[1]) * (Xa-Xb)/k) -
                      d/sqrt(sqr(d)+sqr(k)/4))
        return sum

    # Estimate d from three points
    Xf, Yf, dummy = XYp[(len(XYp)-1)//2]
    Xc_est, Yc_est = centre(Xa, Ya, Xb, Yb, Xf, Yf)
    
    d_est = (Xc_est-Xab2)*(Yb-Ya)/k + (Yc_est-Yab2)*(Xa-Xb)/k

    # Simpsons method
    dLast = d_est - 0.1*k
    dOpt = d_est
    dgLast = dG(dLast)
    dgOpt = dG(dOpt)

    for iter in range(10):
        if dgLast == dgOpt:
            break
        dLast, dOpt = dOpt, dOpt+dgOpt*(dOpt-dLast)/(dgLast-dgOpt)
        dgLast = dgOpt
        dgOpt = dG(dOpt)

    rms = sqrt(G(dOpt) / len(XYp))
    Xc = Xab2 + dOpt/k*(Yb-Ya)
    Yc = Yab2 + dOpt/k*(Xa-Xb)
    rOpt = sqrt(sqr(dOpt) + sqr(k)/4)

    return rOpt, Xc, Yc, rms


def getAngles(Xa, Ya, Xc, Yc, points):
    lasta = atan2(Xa-Xc, Ya-Yc)
    alist = []
    for p in points:
        a = atan2(p[0]-Xc, p[1]-Yc)
        adiff = a-lasta
        while adiff <= -pi:
            adiff += 2*pi
        while adiff > pi:
            adiff -= 2*pi
        alist.append(adiff)
        lasta = a
    return alist


def findBestArc(Xa, Ya, storage, top):
    Xb, Yb = storage[top-1][0], storage[top-1][1]
    r, Xc, Yc, rms = bestArc(Xa, Ya, Xb, Yb, storage[0:top-1])
    # r += 0.001 # Ugly fix to avoid rounding problems

    if rms > rmslimit:
        return None
    angles = getAngles(Xa, Ya, Xc, Yc, storage[0:top])
    if angles[0] < 0:
        # Counter-clockwise
        maxangle = 0
        for a in angles:
            if -a > maxangle:
                maxangle = -a
            if a > 0:
                return None
        if maxangle > anglelimit:
            return None
        if sqr(maxangle)*r > offsetlimit * 4:
            return None
        if (Xb-Xa)*(Ya-Yc)+(Yb-Ya)*(Xc-Xa) > 0:
            # Big side arc
            r = -r
        return "G3", r, Xc, Yc
    else:
        # Clockwise
        maxangle = 0
        for a in angles:
            if a > maxangle:
                maxangle = a
            if a < 0:
                return None
        if maxangle > anglelimit:
            return None
        if sqr(maxangle)*r > offsetlimit * 4:
            return None
        if (Xb-Xa)*(Ya-Yc)+(Yb-Ya)*(Xc-Xa) < 0:
            # Big side arc
            r = -r
        return "G2", r, Xc, Yc


def test():
    # Test data
    Xa = 0
    Ya = 0
    Xb = 2.5
    Yb = 3

    #XYp = [[0.1, 0.4], [0.5, 1.5], [1.5, 2.6]]
    #XYp = [[0.5, 0.2], [1.0, 0.7], [1.5, 1.6]]
    #XYp = [[2/sqrt(2), 2-2/sqrt(2)],]
    #XYp = [[-0.1, 0.7], [0.5, -0.2], [1.1, 0.7]]
    #XYp = [[0.1, 0.2], [0.5, 1.2], [1.0, 2.1], [1.5, 2.7], [1.8, 2.9]]
    XYp = [[Xb*(1-cos(a*pi/180)),Yb*sin(a*pi/180)] for a in range(5, 89, 10)]

    Xa = 102.102
    Ya = 91.548
    XYp = [
        [102.931, 90.479],
        [103.611, 89.783],
        [104.725, 88.901],
        [105.970, 88.200],
        [106.852, 87.839],
        [108.247, 87.457]]
    Xb = 109.806
    Yb = 87.296

    # Plotting

    d = np.arange(-10, 10, 0.1)
    g = d.copy()
    dg = d.copy()
    for i in range(d.size):
        g[i] = G(d[i])
        dg[i] = dG(d[i])

    plt.plot(d, g)
    plt.plot(d, dg)
    plt.vlines(d_est, ymin=-1, ymax=0)
    plt.vlines(dOpt, ymin=-1, ymax=1)
    plt.hlines(0, xmin=-10, xmax=10)
    plt.show()


    figure, axes = plt.subplots()

    axes.set_aspect(1)

    plt.plot(Xa, Ya, "o")
    plt.plot(Xb, Yb, "o")
    plt.plot(Xc, Yc, "*")
    plt.plot(Xc_est, Yc_est, "+")
    plt.plot([xy[0] for xy in XYp], [xy[1] for xy in XYp], ".")

    axes.add_artist(plt.Circle((Xc, Yc), rOpt, fill=False))

    plt.show()
