import sys
import fibrust

if __name__ == '__main__':
    if len(sys.argv) > 1:
        n = int(sys.argv[1])
        print("Fib({}) = {}".format(n, fibrust.get_result(n)))
