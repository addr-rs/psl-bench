import codecs
import sys
from publicsuffix import PublicSuffixList

psl = PublicSuffixList()

for line in sys.stdin:
    print(psl.get_public_suffix(line.rstrip()))
