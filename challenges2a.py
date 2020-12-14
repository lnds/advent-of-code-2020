import sys

def policy(line):
    pol, pswd = line.split(':')
    pswd = pswd.strip()
    fr, lt = pol.split(' ')
    p1, p2 = fr.split('-')
    p1 = int(p1) - 1
    p2 = int(p2) - 1
    if pswd[p1] == lt and pswd[p2] != lt:
        return True
    if pswd[p1] != lt and pswd[p2] == lt:
        return True
    return False


total = 0
for line in sys.stdin:
    if policy(line):
        total += 1
print(total)
