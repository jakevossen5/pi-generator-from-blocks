import math
def main():
    m1 = 1
    n = 8
    m2 = math.pow(100, n)
    count = 0
    v1 = 0
    v2 = -5.0
    
    print("hello world")

    while (v1 < 0.0) or (abs(v1) > v2):
        oldv1 = v1
        oldv2 = v2
        
        v1 = ((m1 - m2) / (m1 + m2)) * oldv1 + (2.0 * m2)/(m1 + m2)*oldv2;
        v2 = (2.0 * m1)/(m1 + m2)* oldv1 + ((m2 - m1)/ (m1 + m2)) * oldv2;

        if v1 < 0: # it is going to switch around at least once
            count = count + 1
            v1 = v1 * -1
        count = count + 1
    print(count)
main()
