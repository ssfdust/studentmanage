#!/bin/python
from time import sleep
from pykeyboard import PyKeyboard

def main():
    sleep(10)
    k = PyKeyboard()
    with open('contents.txt', 'r') as f:
        data = f.read().splitlines()
        for line in data:
            k.type_string(line)
            sleep(0.2)
            k.tap_key(k.enter_key)
            sleep(0.3)


if __name__ == '__main__':
    main()
