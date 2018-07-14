import codecs
import sys
from publicsuffix import PublicSuffixList

psl_file = codecs.open(sys.argv[1], encoding='utf8')
psl = PublicSuffixList(psl_file)

for line in sys.stdin:
    print(psl.get_public_suffix(line.rstrip()))