import sys
from fib import fib

if __name__ == '__main__':
    print(sys.argv)
    if len(sys.argv) > 1:
        n = int(sys.argv[1])
        print("Fib({}) = {}".format(n, fib(n)))

