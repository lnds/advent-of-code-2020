import sys

def policy(line):
    pol, pswd = line.split(':')
    pswd = pswd.strip()
    fr, lt = pol.split(' ')
    mi, mx = fr.split('-')
    c = [x for x in pswd if x == lt]
    lc = len(c)
    return int(mi) <= lc <= int(mx)


total = 0
for line in sys.stdin:
    if policy(line):
        total += 1
print(total)
