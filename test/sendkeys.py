#!/bin/python
from time import sleep
from pykeyboard import PyKeyboard
import os

def main():
    sleep(2)
    k = PyKeyboard()
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'contents.txt')
    with open(path, 'r') as f:
        data = f.read().splitlines()
        for line in data:
            k.type_string(line)
            sleep(0.2)
            k.tap_key(k.enter_key)
            sleep(0.3)


if __name__ == '__main__':
    main()
