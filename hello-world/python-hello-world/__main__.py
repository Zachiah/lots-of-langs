import sys

try:
    name = sys.argv[1]
except IndexError:
    name = 'Nobody'

print(f"Hello {name}")
