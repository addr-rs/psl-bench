import time
import codecs
import sys
from publicsuffix import PublicSuffixList

MILLIS = 1000
MICROS = MILLIS * 1000
NANOS = MICROS * 1000

def benchmark():
    psl_file = codecs.open('benches/list.dat', encoding='utf8')
    psl = PublicSuffixList(psl_file)
    argument = sys.argv[1]

    for line in sys.stdin:
        iters = int(line.strip())

        start = time.clock()
        for x in range(iters):
            psl.get_public_suffix(argument)
        end = time.clock()

        delta = end - start
        nanos = int(delta * NANOS)
        print("%d" % nanos)
        sys.stdout.flush()

benchmark()
