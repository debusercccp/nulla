#!/usr/bin/env python3

import argparse
import time

def get_args():
    parser = argparse.ArgumentParser(
        description='It does nothing',
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)

    parser.add_argument('-n', '--times', metavar='size',
                        type=int, default=3, 
                        help='number of seconds doing nothing')

    return parser.parse_args()

def doNothing(n: int):
    print(f"Dormo per {n} secondi...")
    time.sleep(n)

def main():
    args = get_args()
    
    doNothing(args.times)

if __name__ == "__main__":
    main()
